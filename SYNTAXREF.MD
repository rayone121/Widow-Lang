# Widow Programming Language Syntax Reference

## Comments
```widow
// Single-line comment

/*
Multi-line comment
Can span multiple lines
*/

/**
Documentation comment
Used for generating docs
**/

// TODO: This is a todo comment
// FIXME: This needs to be fixed
// NOTE: Important information
```

## Imports and Modules
```widow
// Import entire module
import std.io
import std.collections

// Import specific items
from std.math import sin, cos, PI
from std.string import split, join

// Import with alias
import std.http as web
import very.long.module.name as short

// Relative imports
import .utils          // Same directory
import ..config        // Parent directory
import ...root.common  // Multiple levels up

// Conditional imports
if OS == "windows" {
    import std.windows as os
} else {
    import std.unix as os
}

// Module declaration (at top of file)
module myapp.utils
```

## Variables and Constants
```widow
// Variable declaration with automatic type inference
x = 5
name = "Alice"

// Explicit type annotations (optional)
x:i32 = 5
name:String = "Alice"

// Constants
const PI = 3.14159
const MAX_USERS:i32 = 1000

// Multiple assignment
a, b, c = 1, 2, 3
x, y = get_coordinates()
```

## Data Types
```widow
// Integer types
i:i8 = 127        // 8-bit signed integer
i:i16 = 32767     // 16-bit signed integer
i:i32 = 42        // 32-bit signed integer (default)
i:i64 = 42        // 64-bit signed integer

u:u8 = 255        // 8-bit unsigned integer
u:u16 = 65535     // 16-bit unsigned integer
u:u32 = 42        // 32-bit unsigned integer
u:u64 = 42        // 64-bit unsigned integer

// Floating point
f:f32 = 3.14      // 32-bit float
f:f64 = 3.14159   // 64-bit float (default)

// Other primitives
b = true          // Boolean
c = 'a'           // Character
s = "Hello"       // String
n = nil           // Null/None value

// Optional types
maybe_number:?i32 = 42     // Can be i32 or nil
maybe_string:?String = nil // Can be String or nil

// Null-safe operators
safe_length = maybe_string?.len()  // Returns nil if maybe_string is nil
default_name = user?.name ?? "Anonymous"  // Null coalescing
```

## String Interpolation
```widow
// Basic string interpolation
name = "Alice"
age = 30
message = `Hello, ${name}! You are ${age} years old.`

// Expression interpolation
result = `The sum is ${a + b}`

// Multi-line strings
text = `
    Line one
    Line two
    Line three
`

// Raw strings (no escaping needed)
regex = r"^\d{3}-\d{2}-\d{4}$"
windows_path = r"C:\Users\Alice\Documents"
```

## Collections
```widow
// Arrays
numbers = [1, 2, 3, 4, 5]
mixed = [1, "hello", true]  // Mixed types allowed
empty:[]i32 = []           // Empty typed array

// Array operations
numbers.append(6)
numbers.insert(0, 0)
length = numbers.len()
first = numbers[0]
last = numbers[-1]

// Slices
subset = numbers[1:4]    // Elements 1, 2, 3
from_start = numbers[:3] // First 3 elements
to_end = numbers[2:]     // From index 2 to end

// Maps (dictionaries)
scores = {"Alice": 95, "Bob": 87}
empty_map:map<String, i32> = {}

// Map operations
scores["Charlie"] = 92
alice_score = scores.get("Alice", 0)  // Default value
keys = scores.keys()
values = scores.values()

// Sets
unique_numbers = {1, 2, 3, 4, 5}
string_set:set<String> = {"apple", "banana", "cherry"}

// Tuples
point = (10, 20)
person = ("Alice", 30, true)
x, y = point  // Basic destructuring
```

## Basic Destructuring
```widow
// Simple object destructuring
{name, age} = person_object
{name, email} = user

// Simple array destructuring
[first, second] = numbers
[head, tail] = items

// Ignore values with underscore
[first, _, third] = items
```

## Ranges
```widow
// Range types
numeric_range = 1..10      // Exclusive end
inclusive_range = 1..=10   // Inclusive end
char_range = 'a'..'z'

// Range operations
range_array = (1..5).to_array()  // [1, 2, 3, 4]
range_contains = (1..10).contains(5)  // true
range_length = (1..10).len()  // 9
```

## Control Flow
```widow
// If statements
if x > 5 {
    print("x is greater than 5")
} elif x == 5 {
    print("x equals 5")
} else {
    print("x is less than 5")
}

// Ternary operator
result = if x > 0 then "positive" else "non-positive"

// For loops with range
for i in 1..5 {
    print(i)
}

// Inclusive range
for i in 1..=5 {  // Includes 5
    print(i)
}

// Step ranges
for i in 1..10 step 2 {  // 1, 3, 5, 7, 9
    print(i)
}

// For loops with collections
for name in ["Alice", "Bob", "Charlie"] {
    print("Hello, " + name)
}

// For loops with index
for i, name in enumerate(["Alice", "Bob"]) {
    print(`${i}: ${name}`)
}

// For loops with maps
for key, value in scores.items() {
    print(`${key}: ${value}`)
}

// While-style loop
for x > 0 {
    x -= 1
}

// Infinite loop
for {
    if some_condition {
        break
    }
}

// Loop control
for i in 1..10 {
    if i == 5 {
        continue  // Skip to next iteration
    }
    if i == 8 {
        break     // Exit loop
    }
    print(i)
}
```

## Switch Statements
```widow
// Basic switch statement
switch day {
    case "Monday": {
        print("Start of work week")
    }
    case "Tuesday", "Wednesday", "Thursday": {
        print("Mid-week")
    }
    case "Friday": {
        print("End of work week")
    }
    case "Saturday", "Sunday": {
        print("Weekend")
    }
    default: {
        print("Invalid day")
    }
}

// Switch with expressions
result = switch grade {
    case "A", "A+": "Excellent"
    case "B", "B+": "Good"
    case "C": "Average"
    default: "Needs improvement"
}

// Switch with conditions
switch {
    case x > 90: "Excellent"
    case x > 80: "Good"
    case x > 70: "Average"
    default: "Poor"
}
```

## Pattern Matching
```widow
// Basic match expressions
result = match value {
    1: "one"
    2, 3: "two or three"
    n if n > 10: "big number"
    _: "something else"
}

// Match with simple destructuring
point = (3, 4)
description = match point {
    (0, 0): "origin"
    (x, 0): `on x-axis at ${x}`
    (0, y): `on y-axis at ${y}`
    (x, y): `point at (${x}, ${y})`
}

// Match with ranges
category = match age {
    0..=12: "child"
    13..=19: "teenager"
    20..=64: "adult"
    _: "senior"
}
```

## Functions
```widow
// Simple function
func add(a, b) {
    ret a + b
}

// Function with type annotations
func multiply(a:i32, b:i32) -> i32 {
    ret a * b
}

// Multiple return values
func divide(a, b) {
    if b == 0 {
        ret 0, "Division by zero"
    }
    ret a / b, nil
}

quotient, err = divide(10, 2)

// Optional parameters with defaults
func greet(name, prefix="Hello") {
    print(`${prefix}, ${name}!`)
}

// Variadic functions
func sum(...numbers) {
    total = 0
    for num in numbers {
        total += num
    }
    ret total
}

// Lambda/anonymous functions
square = func(x) { ret x * x }
numbers.map(func(x) { ret x * 2 })

// Short lambda syntax
square = |x| x * x
add = |a, b| a + b
numbers.map(|x| x * 2)

// Simple closures (all captures by reference)
func make_counter(start) {
    count = start
    ret func() {
        count += 1
        ret count
    }
}

counter = make_counter(10)
print(counter())  // 11
print(counter())  // 12

// Higher-order functions
func apply_operation(a, b, operation) {
    ret operation(a, b)
}

result = apply_operation(5, 3, add)
```

## Error Handling (Return-Based Only)
```widow
// Return-based error handling
func read_file(path) {
    if !file_exists(path) {
        ret nil, "File not found"
    }
    content = file_read(path)
    ret content, nil
}

// Usage
content, err = read_file("config.txt")
if err != nil {
    print("Error: " + err)
} else {
    print("File content: " + content)
}

// Chain error handling
func process_config() {
    content, err = read_file("config.txt")
    if err != nil {
        ret nil, err
    }

    data, err = parse_json(content)
    if err != nil {
        ret nil, err
    }

    ret data, nil
}
```

## Structs and Methods
```widow
// Struct definition
struct Person {
    name:String
    age:i32
    email:?String = nil  // Optional field with default
}

// Struct instantiation
alice = Person{
    name: "Alice",
    age: 30,
    email: "alice@example.com"
}

// Struct update syntax
older_alice = alice with {age: 31}

// Tuple structs
struct Point(f64, f64)
struct Color(u8, u8, u8)

origin = Point(0.0, 0.0)
red = Color(255, 0, 0)
```

## Implementations
```widow
// Implementation for a struct
impl Person {
    // Constructor
    func new(name, age) -> Person {
        ret Person{name: name, age: age}
    }

    // Method with self reference
    func greet(self) {
        print(`Hello, my name is ${self.name}`)
    }

    // Method that modifies self
    func have_birthday(self) {
        self.age += 1
    }

    // Static method
    func species() -> String {
        ret "Homo sapiens"
    }
}

// Usage
bob = Person.new("Bob", 25)
bob.greet()
bob.have_birthday()
species = Person.species()
```

## Traits (Interfaces)
```widow
// Trait definition
trait Display {
    func to_string(self) -> String
}

// Implement trait for struct
impl Display for Person {
    func to_string(self) -> String {
        ret `Person(name: ${self.name}, age: ${self.age})`
    }
}

// Generic trait
trait Comparable<T> {
    func compare(self, other:T) -> i32
}
```

## Simple Generics
```widow
// Generic function
func max<T>(a:T, b:T) -> T {
    ret if a > b then a else b
}

// Generic struct
struct Container<T> {
    value:T

    func new(value:T) -> Container<T> {
        ret Container{value: value}
    }

    func get(self) -> T {
        ret self.value
    }
}

// Generic with multiple type parameters
struct Pair<T, U> {
    first:T
    second:U
}

// Usage
int_container = Container<i32>.new(42)
string_pair = Pair<String, i32>{first: "answer", second: 42}
```

## Basic Concurrency
```widow
// Simple async functions
async func fetch_data(url) {
    response = await http.get(url)
    ret response.body
}

// Await async functions
data = await fetch_data("https://api.example.com")

// Channels for communication
channel = Channel<String>.new()

// Spawn simple tasks
spawn(func() {
    channel.send("Hello from task!")
})

message = channel.receive()
```
