use crate::vm::{error::VMResult, memory::Memory, registers::RegisterFile};
use std::collections::{HashMap, HashSet, VecDeque};

/// Object colors for tricolor marking algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectColor {
    White, // Unmarked (potentially garbage)
    Gray,  // Marked but children not scanned
    Black, // Marked and children scanned
}

/// Metadata for a heap object
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    pub address: u32,
    pub size: u32,
    pub color: ObjectColor,
    pub marked: bool,
    pub generation: u8,       // For generational GC (0 = young, higher = older)
    pub references: Vec<u32>, // Addresses this object references
}

/// Garbage collector configuration
#[derive(Debug, Clone)]
pub struct GCConfig {
    /// Threshold for triggering GC (heap usage percentage)
    pub gc_threshold: f32,
    /// Enable generational collection
    pub generational: bool,
    /// Maximum heap size before forcing collection
    pub max_heap_size: u32,
    /// Enable concurrent collection (simulated)
    pub concurrent: bool,
}

impl Default for GCConfig {
    fn default() -> Self {
        Self {
            gc_threshold: 0.8, // Trigger GC at 80% heap usage
            generational: true,
            max_heap_size: 64 * 1024 * 1024, // 64MB
            concurrent: false,               // Keep simple for now
        }
    }
}

/// Garbage collector statistics
#[derive(Debug, Clone)]
pub struct GCStats {
    pub collections_performed: u64,
    pub objects_collected: u64,
    pub bytes_collected: u64,
    pub last_collection_time_ms: u64,
    pub total_pause_time_ms: u64,
    pub heap_size_before: u32,
    pub heap_size_after: u32,
}

impl Default for GCStats {
    fn default() -> Self {
        Self {
            collections_performed: 0,
            objects_collected: 0,
            bytes_collected: 0,
            last_collection_time_ms: 0,
            total_pause_time_ms: 0,
            heap_size_before: 0,
            heap_size_after: 0,
        }
    }
}

/// Tricolor mark-and-sweep garbage collector
#[derive(Debug)]
pub struct GarbageCollector {
    /// Object metadata table
    objects: HashMap<u32, ObjectMetadata>,
    /// Configuration
    config: GCConfig,
    /// Statistics
    stats: GCStats,
    /// Gray queue for tricolor algorithm
    gray_queue: VecDeque<u32>,
    /// Root set (addresses reachable from registers/stack)
    root_set: HashSet<u32>,
    /// Write barrier log for concurrent collection
    write_barrier_log: Vec<(u32, u32)>, // (object, new_reference)
    /// Generation counters
    generation_sizes: [u32; 8], // Support up to 8 generations
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new(config: GCConfig) -> Self {
        Self {
            objects: HashMap::new(),
            config,
            stats: GCStats::default(),
            gray_queue: VecDeque::new(),
            root_set: HashSet::new(),
            write_barrier_log: Vec::new(),
            generation_sizes: [0; 8],
        }
    }

    /// Create with default configuration
    pub fn new_default() -> Self {
        Self::new(GCConfig::default())
    }

    /// Register a new heap object
    pub fn register_object(&mut self, address: u32, size: u32) {
        let metadata = ObjectMetadata {
            address,
            size,
            color: ObjectColor::White,
            marked: false,
            generation: 0, // New objects start in generation 0
            references: Vec::new(),
        };

        self.objects.insert(address, metadata);
        self.generation_sizes[0] += size;
    }

    /// Remove an object (when manually freed)
    pub fn unregister_object(&mut self, address: u32) {
        if let Some(obj) = self.objects.remove(&address) {
            if (obj.generation as usize) < self.generation_sizes.len() {
                self.generation_sizes[obj.generation as usize] =
                    self.generation_sizes[obj.generation as usize].saturating_sub(obj.size);
            }
        }
    }

    /// Add a reference from one object to another
    pub fn add_reference(&mut self, from: u32, to: u32) {
        if let Some(obj) = self.objects.get_mut(&from) {
            if !obj.references.contains(&to) {
                obj.references.push(to);
            }
        }

        // Write barrier for concurrent collection
        if self.config.concurrent {
            self.write_barrier_log.push((from, to));
        }
    }

    /// Remove a reference
    pub fn remove_reference(&mut self, from: u32, to: u32) {
        if let Some(obj) = self.objects.get_mut(&from) {
            obj.references.retain(|&addr| addr != to);
        }
    }

    /// Check if garbage collection should be triggered
    pub fn should_collect(&self, memory: &Memory) -> bool {
        let stats = memory.get_stats();

        if self.config.max_heap_size > 0 {
            // Proceed only if max_heap_size is configured
            // Calculate the threshold in absolute bytes based on configured max_heap_size
            let heap_used_trigger_point =
                (self.config.max_heap_size as f32 * self.config.gc_threshold) as u32;

            // Condition 1: Trigger if heap usage reaches the calculated trigger point
            if stats.heap_used >= heap_used_trigger_point {
                return true;
            }

            // Condition 2: Trigger if heap usage meets or exceeds the absolute max_heap_size
            // This is a safeguard and handles cases where threshold might be >= 1.0
            if stats.heap_used >= self.config.max_heap_size {
                return true;
            }
        }

        false
    }

    /// Perform garbage collection
    pub fn collect(&mut self, memory: &mut Memory, registers: &RegisterFile) -> VMResult<()> {
        let start_time = std::time::Instant::now();
        let heap_before = memory.get_stats().heap_used;

        // Phase 1: Build root set
        self.build_root_set(memory, registers)?;

        // Phase 2: Mark phase (tricolor algorithm)
        self.mark_phase()?;

        // Phase 3: Sweep phase
        let collected = self.sweep_phase(memory)?;

        // Phase 4: Update statistics
        let collection_time = start_time.elapsed().as_millis() as u64;
        let heap_after = memory.get_stats().heap_used;

        self.update_stats(
            collected.0,
            collected.1,
            collection_time,
            heap_before,
            heap_after,
        );

        // Phase 5: Promote surviving objects to next generation
        if self.config.generational {
            self.promote_survivors();
        }

        Ok(())
    }

    /// Build the root set from registers and stack
    fn build_root_set(&mut self, memory: &Memory, registers: &RegisterFile) -> VMResult<()> {
        self.root_set.clear();

        // Add addresses from registers
        for i in 0..32 {
            if let Ok(value) = registers.read(i) {
                let addr = value as u32;
                if self.is_valid_heap_address(addr, memory) {
                    self.root_set.insert(addr);
                }
            }
        }

        // Add addresses from stack
        let sp = memory.get_stack_pointer();
        let stats = memory.get_stats();
        let stack_base = stats.total_memory - (stats.total_memory / 4); // Approximate stack base

        let mut current_sp = sp;
        while current_sp < stack_base {
            if let Ok(value) = memory.read_word(current_sp) {
                if self.is_valid_heap_address(value, memory) {
                    self.root_set.insert(value);
                }
            }
            current_sp += 4;
        }

        Ok(())
    }

    /// Mark phase using tricolor algorithm
    fn mark_phase(&mut self) -> VMResult<()> {
        // Initialize: all objects are white, roots become gray
        for obj in self.objects.values_mut() {
            obj.color = ObjectColor::White;
            obj.marked = false;
        }

        // Add root set to gray queue
        self.gray_queue.clear();
        let root_addrs: Vec<u32> = self.root_set.iter().cloned().collect();
        for root_addr in root_addrs {
            if self.objects.contains_key(&root_addr) {
                self.mark_gray(root_addr);
            }
        }

        // Process write barrier log for concurrent collection
        let write_barrier_log_clone: Vec<(u32, u32)> = self.write_barrier_log.clone();
        for &(from, to) in &write_barrier_log_clone {
            if self.objects.contains_key(&from) && self.objects.contains_key(&to) {
                self.mark_gray(from);
            }
        }
        self.write_barrier_log.clear();

        // Process gray queue
        while let Some(addr) = self.gray_queue.pop_front() {
            self.mark_black(addr)?;
        }

        Ok(())
    }

    /// Mark an object as gray (reachable but not scanned)
    fn mark_gray(&mut self, addr: u32) {
        if let Some(obj) = self.objects.get_mut(&addr) {
            if obj.color == ObjectColor::White {
                obj.color = ObjectColor::Gray;
                obj.marked = true;
                self.gray_queue.push_back(addr);
            }
        }
    }

    /// Mark an object as black (reachable and scanned)
    fn mark_black(&mut self, addr: u32) -> VMResult<()> {
        if let Some(obj) = self.objects.get(&addr).cloned() {
            // Mark all referenced objects as gray
            for &ref_addr in &obj.references {
                self.mark_gray(ref_addr);
            }

            // Mark this object as black
            if let Some(obj) = self.objects.get_mut(&addr) {
                obj.color = ObjectColor::Black;
            }
        }

        Ok(())
    }

    /// Sweep phase - collect white objects
    fn sweep_phase(&mut self, memory: &mut Memory) -> VMResult<(u64, u64)> {
        let mut objects_collected = 0;
        let mut bytes_collected = 0;
        let mut to_remove = Vec::new();

        for (&addr, obj) in &self.objects {
            if obj.color == ObjectColor::White {
                // This object is garbage
                to_remove.push(addr);
                objects_collected += 1;
                bytes_collected += obj.size as u64;

                // Free the memory
                if let Err(_) = memory.free(addr) {
                    // Object might have been manually freed already
                }
            }
        }

        // Remove collected objects from tracking
        for addr in to_remove {
            self.unregister_object(addr);
        }

        Ok((objects_collected, bytes_collected))
    }

    /// Promote surviving objects to next generation
    fn promote_survivors(&mut self) {
        for obj in self.objects.values_mut() {
            if obj.marked && obj.generation < 7 {
                // Move size from old generation to new
                if (obj.generation as usize) < self.generation_sizes.len() {
                    self.generation_sizes[obj.generation as usize] =
                        self.generation_sizes[obj.generation as usize].saturating_sub(obj.size);
                }

                obj.generation += 1;

                if (obj.generation as usize) < self.generation_sizes.len() {
                    self.generation_sizes[obj.generation as usize] += obj.size;
                }
            }
        }
    }

    /// Update GC statistics
    fn update_stats(
        &mut self,
        objects_collected: u64,
        bytes_collected: u64,
        collection_time: u64,
        heap_before: u32,
        heap_after: u32,
    ) {
        self.stats.collections_performed += 1;
        self.stats.objects_collected += objects_collected;
        self.stats.bytes_collected += bytes_collected;
        self.stats.last_collection_time_ms = collection_time;
        self.stats.total_pause_time_ms += collection_time;
        self.stats.heap_size_before = heap_before;
        self.stats.heap_size_after = heap_after;
    }

    /// Check if an address is a valid heap address
    fn is_valid_heap_address(&self, addr: u32, memory: &Memory) -> bool {
        let stats = memory.get_stats();
        let heap_base = if stats.total_memory > 0x10000 {
            0x10000
        } else {
            stats.total_memory / 4
        };
        let stack_base = if stats.total_memory > 0x100000 {
            stats.total_memory - 0x100000
        } else {
            stats.total_memory * 3 / 4
        };

        addr >= heap_base && addr < stack_base
    }

    /// Force garbage collection
    pub fn force_collect(&mut self, memory: &mut Memory, registers: &RegisterFile) -> VMResult<()> {
        self.collect(memory, registers)
    }

    /// Minor collection (young generation only)
    pub fn minor_collect(&mut self, memory: &mut Memory, registers: &RegisterFile) -> VMResult<()> {
        if !self.config.generational {
            return self.collect(memory, registers);
        }

        // Only collect generation 0 and 1 objects
        let old_objects: HashMap<u32, ObjectMetadata> = self.objects.clone();

        // Temporarily filter to only young objects
        self.objects.retain(|_, obj| obj.generation <= 1);

        // Perform collection
        let result = self.collect(memory, registers);

        // Restore old objects that weren't collected
        for (addr, obj) in old_objects {
            if obj.generation > 1 && !self.objects.contains_key(&addr) {
                self.objects.insert(addr, obj);
            }
        }

        result
    }

    /// Get GC configuration
    pub fn get_config(&self) -> &GCConfig {
        &self.config
    }

    /// Update GC configuration
    pub fn set_config(&mut self, config: GCConfig) {
        self.config = config;
    }

    /// Get GC statistics
    pub fn get_stats(&self) -> &GCStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = GCStats::default();
    }

    /// Get number of tracked objects
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Get total size of tracked objects
    pub fn total_object_size(&self) -> u32 {
        self.objects.values().map(|obj| obj.size).sum()
    }

    /// Print GC state for debugging
    pub fn dump_state(&self) -> String {
        let mut output = String::new();
        output.push_str("=== Garbage Collector State ===\n");
        output.push_str(&format!("Objects tracked: {}\n", self.objects.len()));
        output.push_str(&format!(
            "Total object size: {} bytes\n",
            self.total_object_size()
        ));
        output.push_str(&format!(
            "Collections performed: {}\n",
            self.stats.collections_performed
        ));
        output.push_str(&format!(
            "Objects collected: {}\n",
            self.stats.objects_collected
        ));
        output.push_str(&format!(
            "Bytes collected: {} bytes\n",
            self.stats.bytes_collected
        ));
        output.push_str(&format!(
            "Total pause time: {} ms\n",
            self.stats.total_pause_time_ms
        ));

        if self.config.generational {
            output.push_str("\nGeneration sizes:\n");
            for (r#gen, &size) in self.generation_sizes.iter().enumerate() {
                if size > 0 {
                    output.push_str(&format!("  Gen {}: {} bytes\n", r#gen, size));
                }
            }
        }

        output.push_str("\nObject details:\n");
        for (addr, obj) in &self.objects {
            output.push_str(&format!(
                "  0x{:08X}: {} bytes, gen {}, {:?}, {} refs\n",
                addr,
                obj.size,
                obj.generation,
                obj.color,
                obj.references.len()
            ));
        }

        output
    }

    fn format_stats(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "Total allocated: {} bytes\n",
            self.total_object_size()
        ));
        output.push_str(&format!("Live objects: {}\n", self.object_count()));
        output.push_str("Generation sizes:\n");
        if !self.config.generational {
            output.push_str("  No generations configured (generational GC disabled).\n");
        } else {
            for (r#gen, &size) in self.generation_sizes.iter().enumerate() {
                let allocated_in_gen = size;
                output.push_str(&format!(
                    "  Gen {}: {} bytes (allocated: {} bytes)\n",
                    r#gen, size, allocated_in_gen
                ));
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::memory::Memory;
    use crate::vm::registers::RegisterFile;

    #[test]
    fn test_gc_creation() {
        let gc = GarbageCollector::new_default();
        assert_eq!(gc.object_count(), 0);
        assert_eq!(gc.get_stats().collections_performed, 0);
    }

    #[test]
    fn test_object_registration() {
        let mut gc = GarbageCollector::new_default();

        gc.register_object(0x1000, 100);
        gc.register_object(0x2000, 200);

        assert_eq!(gc.object_count(), 2);
        assert_eq!(gc.total_object_size(), 300);
    }

    #[test]
    fn test_reference_tracking() {
        let mut gc = GarbageCollector::new_default();

        gc.register_object(0x1000, 100);
        gc.register_object(0x2000, 200);

        gc.add_reference(0x1000, 0x2000);

        let obj = gc.objects.get(&0x1000).unwrap();
        assert!(obj.references.contains(&0x2000));
    }

    #[test]
    fn test_simple_collection() {
        let mut memory = Memory::new(1024 * 1024);
        let registers = RegisterFile::new();
        let mut gc = GarbageCollector::new_default();

        // Allocate some objects
        let addr1 = memory.allocate(100).unwrap();
        let addr2 = memory.allocate(200).unwrap();

        gc.register_object(addr1, 100);
        gc.register_object(addr2, 200);

        // Perform collection (both should be collected as garbage)
        let result = gc.collect(&mut memory, &registers);
        assert!(result.is_ok());

        // Objects should be collected
        assert_eq!(gc.object_count(), 0);
        assert!(gc.get_stats().collections_performed > 0);
    }

    #[test]
    fn test_reachable_objects() {
        let mut memory = Memory::new(1024 * 1024);
        let mut registers = RegisterFile::new();
        let mut gc = GarbageCollector::new_default();

        // Allocate objects
        let addr1 = memory.allocate(100).unwrap();
        let addr2 = memory.allocate(200).unwrap();

        gc.register_object(addr1, 100);
        gc.register_object(addr2, 200);

        // Make addr1 reachable from register
        registers.write(1, addr1 as i32).unwrap();

        // Add reference from addr1 to addr2
        gc.add_reference(addr1, addr2);

        // Perform collection
        let result = gc.collect(&mut memory, &registers);
        assert!(result.is_ok());

        // Both objects should survive (addr1 is root, addr2 is referenced)
        assert_eq!(gc.object_count(), 2);
    }

    #[test]
    fn test_gc_threshold() {
        let memory = Memory::new(1000);
        let config = GCConfig {
            gc_threshold: 0.5,
            ..Default::default()
        };
        let gc = GarbageCollector::new(config);

        // With small memory, should trigger collection
        assert!(gc.should_collect(&memory));
    }
}
