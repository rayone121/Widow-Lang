use widow_lang::{VM, InstructionBuilder, encode, vm::GCConfig};
use widow_lang::compiler::instruction_builder::registers::*;
use widow_lang::lexer::{WidowLexer, Token, LocatedToken};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "lexer" {
        println!("=== Widow Language Lexer Demo ===\n");
        demo_lexer();
        return;
    }

    println!("=== Widow Language VM Demo ===\n");
    println!("Run with 'cargo run lexer' to see lexer demo\n");

    // Create a new VM with 16MB memory
    let mut vm = VM::new_default();

    // Demo 1: Basic arithmetic
    println!("Demo 1: Basic Arithmetic");
    demo_arithmetic(&mut vm);

    // Demo 2: Conditional branching
    println!("\nDemo 2: Conditional Branching");
    demo_branching(&mut vm);

    // Demo 3: Function calls
    println!("\nDemo 3: Function Calls");
    demo_function_calls(&mut vm);

    // Demo 4: Memory operations
    println!("\nDemo 4: Memory Operations");
    demo_memory_operations(&mut vm);

    // Demo 5: I/O operations
    println!("\nDemo 5: I/O Operations");
    demo_io_operations(&mut vm);

    // Demo 6: Garbage collection
    println!("\nDemo 6: Garbage Collection");
    demo_garbage_collection(&mut vm);

    println!("\n=== All demos completed successfully! ===");
}

fn demo_arithmetic(vm: &mut VM) {
    println!("Running arithmetic operations...");

    // Program: Calculate (10 + 5) * 3 - 2
    let program = vec![
        encode(InstructionBuilder::load_immediate(r1(), 10)),     // R1 = 10
        encode(InstructionBuilder::load_immediate(r2(), 5)),      // R2 = 5
        encode(InstructionBuilder::add(r3(), r1(), r2())),        // R3 = R1 + R2 = 15
        encode(InstructionBuilder::load_immediate(r4(), 3)),      // R4 = 3
        encode(InstructionBuilder::mul(r5(), r3(), r4())),        // R5 = R3 * R4 = 45
        encode(InstructionBuilder::load_immediate(r6(), 2)),      // R6 = 2
        encode(InstructionBuilder::sub(r0(), r5(), r6())),        // R0 = R5 - R6 = 43
        encode(InstructionBuilder::print(r0())),                  // Print result
        encode(InstructionBuilder::halt()),
    ];

    vm.reset();
    vm.load_program(&program).expect("Failed to load program");
    vm.run().expect("Failed to run program");

    println!("Expected result: 43");
    println!("Registers after execution:");
    print_registers(vm, &[0, 1, 2, 3, 4, 5, 6]);
}

fn demo_branching(vm: &mut VM) {
    println!("Running conditional branching...");

    // Program: Compare two numbers and print the larger one
    let program = vec![
        encode(InstructionBuilder::load_immediate(r1(), 15)),     // R1 = 15
        encode(InstructionBuilder::load_immediate(r2(), 10)),     // R2 = 10
        encode(InstructionBuilder::branch_less_than(r1(), r2(), 12)), // if R1 < R2, skip to print R2
        encode(InstructionBuilder::print(r1())),                  // Print R1 (larger)
        encode(InstructionBuilder::jump(24)),                     // Jump to end
        encode(InstructionBuilder::print(r2())),                  // Print R2 (larger)
        encode(InstructionBuilder::halt()),
    ];

    vm.reset();
    vm.load_program(&program).expect("Failed to load program");
    vm.run().expect("Failed to run program");

    println!("Expected to print: 15 (the larger number)");
}

fn demo_function_calls(vm: &mut VM) {
    println!("Running function call demonstration...");

    // Program: Simple function that doubles a number
    // Main: Call function with 21, expect 42
    let program = vec![
        // Main function (starts at address 0)
        encode(InstructionBuilder::load_immediate(r1(), 21)),     // 0: R1 = 21 (argument)
        encode(InstructionBuilder::call(20)),                     // 4: Call function at address 20
        encode(InstructionBuilder::print(r1())),                  // 8: Print result
        encode(InstructionBuilder::halt()),                       // 12: End program
        
        encode(InstructionBuilder::nop()),                        // 16: Padding
        
        // Double function (starts at address 20)
        encode(InstructionBuilder::add(r1(), r1(), r1())),        // 20: R1 = R1 + R1 (double it)
        encode(InstructionBuilder::ret()),                        // 24: Return
    ];

    vm.reset();
    vm.load_program(&program).expect("Failed to load program");
    vm.run().expect("Failed to run program");

    println!("Expected result: 42 (21 * 2)");
}

fn demo_memory_operations(vm: &mut VM) {
    println!("Running memory operations...");

    // Program: Allocate memory, store values, load them back
    let program = vec![
        encode(InstructionBuilder::load_immediate(r1(), 100)),    // R1 = 100 (size to allocate)
        encode(InstructionBuilder::allocate(r2(), r1())),         // R2 = allocated address
        encode(InstructionBuilder::load_immediate(r3(), 42)),     // R3 = 42 (value to store)
        encode(InstructionBuilder::store(r3(), r2(), 0)),         // Store R3 at address R2+0
        encode(InstructionBuilder::load_immediate(r4(), 99)),     // R4 = 99 (another value)
        encode(InstructionBuilder::store(r4(), r2(), 4)),         // Store R4 at address R2+4
        
        // Load values back
        encode(InstructionBuilder::load(r5(), r2(), 0)),          // R5 = memory[R2+0]
        encode(InstructionBuilder::load(r6(), r2(), 4)),          // R6 = memory[R2+4]
        
        encode(InstructionBuilder::print(r5())),                  // Print first value
        encode(InstructionBuilder::print(r6())),                  // Print second value
        
        encode(InstructionBuilder::free(r2())),                   // Free allocated memory
        encode(InstructionBuilder::halt()),
    ];

    vm.reset();
    vm.load_program(&program).expect("Failed to load program");
    vm.run().expect("Failed to run program");

    println!("Expected to print: 42, then 99");
}

fn demo_io_operations(vm: &mut VM) {
    println!("Running I/O operations...");
    println!("This demo will print some numbers and then ask for input.");

    // Program: Print numbers 1, 2, 3, 4, 5 and demonstrate input
    let program = vec![
        // Just print the numbers directly (simpler than looping)
        encode(InstructionBuilder::load_immediate(r1(), 1)),      // R1 = 1
        encode(InstructionBuilder::print(r1())),                  // Print 1
        encode(InstructionBuilder::load_immediate(r1(), 2)),      // R1 = 2
        encode(InstructionBuilder::print(r1())),                  // Print 2
        encode(InstructionBuilder::load_immediate(r1(), 3)),      // R1 = 3
        encode(InstructionBuilder::print(r1())),                  // Print 3
        encode(InstructionBuilder::load_immediate(r1(), 4)),      // R1 = 4
        encode(InstructionBuilder::print(r1())),                  // Print 4
        encode(InstructionBuilder::load_immediate(r1(), 5)),      // R1 = 5
        encode(InstructionBuilder::print(r1())),                  // Print 5
        
        // Input demonstration (commented out for non-interactive demo)
        // encode(InstructionBuilder::read(r3())),                   // Read number into R3
        // encode(InstructionBuilder::print(r3())),                  // Echo the input
        
        encode(InstructionBuilder::halt()),                       // End program
    ];

    vm.reset();
    vm.load_program(&program).expect("Failed to load program");
    
    println!("Expected to print numbers 1-5:");
    vm.run().expect("Failed to run program");
}

fn demo_garbage_collection(vm: &mut VM) {
    println!("Running garbage collection demonstration...");

    // Create a custom GC config for demonstration
    let gc_config = GCConfig {
        gc_threshold: 0.3, // Lower threshold for demo
        generational: true,
        max_heap_size: 2000, // Adjusted to trigger GC with demo allocations
        concurrent: false,
    };
    
    // Create a new VM with custom GC config
    let mut gc_vm = VM::new_with_gc(2 * 1024 * 1024, gc_config); // 2MB memory
    
    // Program that allocates objects and creates references
    let program = vec![
        // Allocate first object (100 bytes)
        encode(InstructionBuilder::load_immediate(r1(), 100)),     // Size
        encode(InstructionBuilder::allocate(r2(), r1())),          // R2 = allocated address
        encode(InstructionBuilder::print(r2())),                   // Print address
        
        // Allocate second object (200 bytes) 
        encode(InstructionBuilder::load_immediate(r1(), 200)),     // Size
        encode(InstructionBuilder::allocate(r3(), r1())),          // R3 = allocated address
        encode(InstructionBuilder::print(r3())),                   // Print address
        
        // Store reference from first object to second (simulate object reference)
        encode(InstructionBuilder::store(r3(), r2(), 0)),          // memory[R2] = R3
        
        // Allocate third object (150 bytes) - this should trigger GC
        encode(InstructionBuilder::load_immediate(r1(), 150)),     // Size
        encode(InstructionBuilder::allocate(r4(), r1())),          // R4 = allocated address
        encode(InstructionBuilder::print(r4())),                   // Print address
        
        // Clear references to make objects eligible for collection
        encode(InstructionBuilder::load_immediate(r2(), 0)),       // Clear R2
        encode(InstructionBuilder::load_immediate(r3(), 0)),       // Clear R3
        
        // Force garbage collection
        encode(InstructionBuilder::allocate(r5(), r1())),          // This should trigger GC
        encode(InstructionBuilder::print(r5())),                   // Print new address
        
        encode(InstructionBuilder::halt()),
    ];

    gc_vm.load_program(&program).expect("Failed to load GC demo program");
    
    println!("GC stats before execution:");
    let stats_before = gc_vm.get_gc().get_stats();
    println!("  Collections: {}", stats_before.collections_performed);
    println!("  Objects tracked: {}", gc_vm.get_gc().object_count());
    
    println!("Running program with automatic GC...");
    gc_vm.run().expect("Failed to run GC demo");
    
    println!("GC stats after execution:");
    let stats_after = gc_vm.get_gc().get_stats();
    println!("  Collections: {}", stats_after.collections_performed);
    println!("  Objects collected: {}", stats_after.objects_collected);
    println!("  Bytes collected: {}", stats_after.bytes_collected);
    println!("  Objects still tracked: {}", gc_vm.get_gc().object_count());
    println!("  Total pause time: {} ms", stats_after.total_pause_time_ms);
    
    // Demonstrate manual GC
    println!("\nForcing manual garbage collection...");
    gc_vm.force_gc().expect("Failed to force GC");
    
    let final_stats = gc_vm.get_gc().get_stats();
    println!("Final GC stats:");
    println!("  Total collections: {}", final_stats.collections_performed);
    println!("  Total objects collected: {}", final_stats.objects_collected);
    println!("  Total bytes collected: {}", final_stats.bytes_collected);
    println!("  Objects remaining: {}", gc_vm.get_gc().object_count());
}

fn demo_lexer() {
    let source_code = r#"
        // Function definition with type annotations
        func fibonacci(n:i32) -> i32 {
            if n <= 1 {
                ret n
            } else {
                ret fibonacci(n - 1) + fibonacci(n - 2)
            }
        }

        // Variables and constants
        const MAX_SIZE = 100
        name = "Alice"
        age:i32 = 30
        
        // String interpolation
        greeting = `Hello, ${name}! You are ${age} years old.`
        
        // Collections
        numbers = [1, 2, 3, 4, 5]
        scores = {"Alice": 95, "Bob": 87}
        
        // For loop with range
        for i in 1..=10 {
            if i % 2 == 0 {
                print(`${i} is even`)
            }
        }
        
        // Pattern matching
        result = match value {
            1: "one"
            2, 3: "two or three"
            n if n > 10: "big number"
            _: "something else"
        }
        
        // TODO: Add more examples
        /* This is a multi-line
           block comment */
        /** Documentation comment **/
    "#;

    println!("Source code:");
    println!("{}\n", source_code);

    // Create lexer and tokenize
    let mut lexer = WidowLexer::new(source_code);
    let mut token_count = 0;
    let mut error_count = 0;

    println!("=== Token Stream ===");
    println!("{:<5} {:<15} {:<25} {:<10} {}", 
             "Line", "Column", "Token", "Span", "Text");
    println!("{:-<80}", "");

    while let Some(token_result) = lexer.next_token() {
        match token_result {
            Ok(located_token) => {
                token_count += 1;
                let LocatedToken { token, span, start_pos, .. } = located_token;
                let text = &source_code[span.clone()];
                
                // Skip newlines for cleaner output
                if matches!(token, Token::Newline) {
                    continue;
                }
                
                println!("{:<5} {:<15} {:<25} {:<10} {}", 
                         start_pos.line,
                         start_pos.column,
                         format!("{:?}", token),
                         format!("{}..{}", span.start, span.end),
                         escape_whitespace(text));
            }
            Err(error_token) => {
                error_count += 1;
                let text = &source_code[error_token.span.clone()];
                println!("ERROR at {}:{} - Invalid token: '{}'", 
                         error_token.start_pos.line,
                         error_token.start_pos.column,
                         escape_whitespace(text));
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Total tokens: {}", token_count);
    println!("Errors: {}", error_count);

    // Demonstrate token classification
    println!("\n=== Token Classification Examples ===");
    let examples = vec![
        ("func", "keyword"),
        ("fibonacci", "identifier"), 
        ("42", "integer literal"),
        ("3.14", "float literal"),
        ("\"hello\"", "string literal"),
        ("+", "operator"),
        ("==", "comparison operator"),
        ("// comment", "line comment"),
        ("// TODO: something", "line comment with TODO"),
    ];

    for (text, description) in examples {
        let mut example_lexer = WidowLexer::new(text);
        if let Some(Ok(located_token)) = example_lexer.next_token() {
            println!("{:<15} -> {:<25} ({})", 
                     text, 
                     format!("{:?}", located_token.token),
                     description);
                     
            // Show token properties
            let token = &located_token.token;
            let mut properties = Vec::new();
            
            if token.is_keyword() {
                properties.push("keyword");
            }
            if token.is_literal() {
                properties.push("literal");
            }
            if token.is_operator() {
                properties.push("operator");
            }
            if token.is_comment() {
                properties.push("comment");
            }
            
            if !properties.is_empty() {
                println!("                Properties: {}", properties.join(", "));
            }
        }
    }
}

fn escape_whitespace(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            c if c.is_control() => format!("\\u{{{:04x}}}", c as u32),
            c => c.to_string(),
        })
        .collect()
}

fn print_registers(vm: &VM, registers: &[u8]) {
    for &reg in registers {
        if let Ok(value) = vm.get_registers().read(reg) {
            println!("  R{}: {}", reg, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_demos() {
        let mut vm = VM::new_default();
        
        // Test that all demos can at least load and start running
        demo_arithmetic(&mut vm);
        demo_garbage_collection(&mut vm);
        // Note: Other demos require user input or have specific expectations
        // so they're not included in automated tests
    }

    #[test]
    fn test_garbage_collection_integration() {
        let mut vm = VM::new_default();
        
        // Test that GC integration works
        let program = vec![
            encode(InstructionBuilder::load_immediate(r1(), 100)),
            encode(InstructionBuilder::allocate(r2(), r1())),
            encode(InstructionBuilder::halt()),
        ];
        
        vm.load_program(&program).unwrap();
        vm.run().unwrap();
        
        // Should have one object tracked
        assert_eq!(vm.get_gc().object_count(), 1);
        
        // Force GC should work
        assert!(vm.force_gc().is_ok());
    }

    #[test]
    fn test_comprehensive_instruction_set() {
        let mut vm = VM::new_default();

        // Test all major instruction types
        let program = vec![
            // Arithmetic
            encode(InstructionBuilder::load_immediate(r1(), 10)),
            encode(InstructionBuilder::load_immediate(r2(), 3)),
            encode(InstructionBuilder::add(r3(), r1(), r2())),        // 13
            encode(InstructionBuilder::sub(r4(), r1(), r2())),        // 7
            encode(InstructionBuilder::mul(r5(), r1(), r2())),        // 30
            encode(InstructionBuilder::div(r6(), r1(), r2())),        // 3
            
            // Logical operations
            encode(InstructionBuilder::load_immediate(r7(), 0b1010)),
            encode(InstructionBuilder::load_immediate(r8(), 0b1100)),
            encode(InstructionBuilder::and(r9(), r7(), r8())),        // 0b1000 = 8
            encode(InstructionBuilder::or(r10(), r7(), r8())),        // 0b1110 = 14
            encode(InstructionBuilder::xor(r11(), r7(), r8())),       // 0b0110 = 6
            
            encode(InstructionBuilder::halt()),
        ];

        vm.load_program(&program).expect("Failed to load program");
        vm.run().expect("Failed to run program");

        // Verify results
        assert_eq!(vm.get_registers().read(3).unwrap(), 13);
        assert_eq!(vm.get_registers().read(4).unwrap(), 7);
        assert_eq!(vm.get_registers().read(5).unwrap(), 30);
        assert_eq!(vm.get_registers().read(6).unwrap(), 3);
        assert_eq!(vm.get_registers().read(9).unwrap(), 8);
        assert_eq!(vm.get_registers().read(10).unwrap(), 14);
        assert_eq!(vm.get_registers().read(11).unwrap(), 6);
    }
}