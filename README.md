# Course: Rust By Example

Course link: https://doc.rust-lang.org/stable/rust-by-example

## Index

- [1. Hello World](01_hello/src/main.rs)
  - 1.1. Comments
  - 1.2. Formatted print
    - 1.2.1. Debug
    - 1.2.2. Display
      - 1.2.2.1. Testcase: List
    - 1.2.3. Formatting
- [2. Primitives](02_primitives/src/main.rs)
  - 2.1. Literals and operators
  - 2.2. Tuples
  - 2.3. Arrays and Slices
- [3. Custom types](03_custom_types/src/main.rs)
  - 3.1. Structures
  - 3.2. Enums
    - 3.2.1. use
    - 3.2.2. C-like
    - 3.2.3. Testcase: linked-list
  - 3.3. constants
- [4. Variable Bindings](04_variable_bindings/src/main.rs)
  - 4.1. Mutability
  - 4.2. Scope and Shadowing
  - 4.3. Declare first
  - 4.4. Freezing
- [5. Types](05_types/src/main.rs)
  - 5.1. Casting
  - 5.2. Literals
  - 5.3. Inference
  - 5.4. Aliasing
- [6. Conversion](06_conversion/src/main.rs)
  - 6.1. From and Into
  - 6.2. TryFrom and TryInto
  - 6.3. To and from Strings
- [7. Expressions](07_expressions/src/main.rs)
- [8. Flow of Control](08_flow_control/src/main.rs)
  - 8.1. if/else
  - 8.2. loop
    - 8.2.1 Nesting and labels
    - 8.2.1 Returning from loops
  - 8.3. while
  - 8.4. for and range
  - 8.5. match
    - 8.5.1 Destructuring
      - 8.5.1.1. tuples
      - 8.5.1.2. enums
      - 8.5.1.3. pointers/refs
      - 8.5.1.4. structs
    - 8.5.2 Guards
    - 8.5.3 Binding
  - 8.6. if let
  - 8.7. while let
- [9. Functions](09_functions/src/main.rs)
  - 9.1. Associated functions & Methods
  - 9.2. Closures
    - 9.2.1. Capturing
    - 9.2.2. As input parameters
    - 9.2.3. Type anonymity
    - 9.2.4. Input functions
    - 9.2.5. As output parameters
    - 9.2.6. Examples in std
      - 9.2.6.1. Iterator::any
      - 9.2.6.2. Searching through iterators
  - 9.3. Higher Order Functions
  - 9.4. Diverging functions
- [10. Modules](10_modules/src/main.rs)
  - 10.1. Visibility
  - 10.2. Struct visibility
  - 10.3. The use declaration
  - 10.4. super and self
  - [10.5. File hierarchy](10_modules_file_hierarchy/src/main.rs)
- [11. Creates](11_crates/src/main.rs)
  - [11.1. Creating a Library](11_crates_library)
  - 11.2. Using a Library
- [12. Cargo](https://doc.rust-lang.org/rust-by-example/cargo.html) `[link]`
  - 12.1. Dependencies
  - 12.2. Conventions
  - 12.3. Testing
  - 12.4. Build Scripts
- [13. Attributes](13_attributes/src/main.rs)
  - 13.1. dead_code
  - 13.2. Crates
  - 13.3. cfg
    - 13.3.1. Custom
- [14. Generics](14_generics/src/main.rs)
  - 14.1. Functions
  - 14.2. Implementation
  - 14.3. Traits
  - 14.4. Bounds
    - 14.4.1. Testcase: empty bounds
  - 14.5. Multiple bounds
  - 14.6. Where clauses
  - 14.7. New Type Idiom
  - 14.8. Associated items
    - 14.8.1. The Problem
      - 14.8.2. Associated types
  - 14.9. Phantom type parameters
    - 14.9.1. Testcase: unit clarification
- [15. Scoping rules](15_scoping_rules/src/main.rs)
  - 15.1. RAII
  - 15.2. Ownership and moves
    - 15.2.1. Mutability
    - 15.2.2. Partial moves
  - 15.3. Borrowing
    - 15.3.1. Mutability
    - 15.3.2. Aliasing
    - 15.3.3. The ref pattern
  - 15.4. Lifetimes
    - 15.4.1. Explicit annotation
    - 15.4.2. Functions
    - 15.4.3. Methods
    - 15.4.4. Structs
    - 15.4.5. Traits
    - 15.4.6. Bounds
    - 15.4.7. Coercion
    - 15.4.8. Static
    - 15.4.9. Elision
- [16. Traits](16_traits/src/main.rs)
  - 16.1. Derive
  - 16.2. Returning Traits with dyn
  - 16.3. Operator Overloading
  - 16.4. Drop
  - 16.5. Iterators
  - 16.6. impl Trait
  - 16.7. Clone
  - 16.8. Supertraits
  - 16.9. Disambiguating overlapping traits
- [17. macro_rules!](17_macro_rules)
  - 17.1. Syntax
    - 17.1.1. Designators
    - 17.1.2. Overload
    - 17.1.3. Repeat
  - 17.2. DRY (Don't Repeat Yourself)
  - 17.3. Domain Specific Languages (DSLs)
  - 17.4. Variadic Interfaces
- [18. Error handling](18_error_handling/src/main.rs)
  - 18.1. panic
  - 18.2. Option & unwrap
    - 18.2.1. Unpacking options with ?
    - 18.2.2. Combinators: map
    - 18.2.3. Combinators: and_then
  - 18.3. Result
    - 18.3.1. map for Result
    - 18.3.2. aliases for Result
    - 18.3.3. Early returns
    - 18.3.4. Introducing ?
  - 18.4. Multiple error types
    - 18.4.1. Pulling Results out of Options
    - 18.4.2. Defining an error type
    - 18.4.3. Boxing errors
    - 18.4.4. Other uses of ?
    - 18.4.5. Wrapping errors
  - 18.5. Iterating over Results
- [19. Std library types](19_std_library_types/src/main.rs)
  - 19.1. Box, stack and heap
  - 19.2. Vectors
  - 19.3. Strings
  - 19.4. Option
  - 19.5. Result
    - 19.5.1. ?
  - 19.6. panic!
  - 19.7. HashMap
    - 19.7.1. Alternate/custom key types
    - 19.7.2. HashSet
  - 19.8. Rc
  - 19.9. Arc
- [20. Std misc](20_std_misc/src/main.rs)
  - 20.1. Threads
    - 20.1.1. Testcase: map-reduce
  - 20.2. Channels
  - 20.3. Path
  - 20.4. File I/O
    - 20.4.1. open
    - 20.4.2. create
    - 20.4.3. read_lines
  - 20.5. Child processes
    - 20.5.1. Pipes
    - 20.5.2. Wait

[comment]: <> (  - 20.6. Filesystem Operations)

[comment]: <> (  - 20.7. Program arguments)

[comment]: <> (    - 20.7.1. Argument parsing)

[comment]: <> (  - 20.8. Foreign Function Interface)

## Run in terminal

All chapters have their own folders so binary run commands have the following format: `cargo run --manifest-path ./N_CHAPTER/Cargo.toml`

> ℹ️ Windows: use PowerShell

Example:

```shell
cargo run --manifest-path ./02_primitives/Cargo.toml
```

## Notes

### Comments

- Some of my thoughts are prefixed with `NOTE:`
  - Example: `// NOTE: Algorithm complexity: O(n)`
- Resolved course TODOs are prefixed with `DONE:`
  - Example: `// DONE: ^ Uncomment the above 2 lines to see the compiler error`
- Other comments copied from the course
                                        
### A new chapter

> ℹ️ Cargo projects cannot be named leading from a digit

To create a new chapter-related subfolder, please use the following format: `cargo new N_name --name _N_name` 

#### Quick commands

> ℹ️ Update N and NAME variable values

Unix-like:
```shell
N=01; NAME=hello; cargo new "${N}_${NAME}" --name "_${N}_${NAME}"
```

Windows (Powershell):
```shell
$N='01'; $NAME='hello'; cargo new ${N}_${NAME} --name _${N}_${NAME}
```

## Code conduction

This project uses [Gitmoji](https://gitmoji.carloscuesta.me) for commit messages.

## License

[GPLv3+](LICENSE)
