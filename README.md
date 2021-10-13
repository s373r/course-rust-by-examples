# Course: Rust By Examples

Course link: https://doc.rust-lang.org/stable/rust-by-example

## Index

- [1. Hello World](1_hello/src/main.rs)
  - 1.1. Comments
  - 1.2. Formatted print
    - 1.2.1. Debug
    - 1.2.2. Display
      - 1.2.2.1. Testcase: List
    - 1.2.3. Formatting
- [2. Primitives](2_primitives/src/main.rs)
  - 2.1. Literals and operators
  - 2.2. Tuples
  - 2.3. Arrays and Slices
- [3. Custom types](3_custom_types/src/main.rs)
  - 3.1. Structures
  - 3.2. Enums
    - 3.2.1. use
    - 3.2.2. C-like
    - 3.2.3. Testcase: linked-list
  - 3.3. constants
- [4. Variable Bindings](4_variable_bindings/src/main.rs)
  - 4.1. Mutability
  - 4.2. Scope and Shadowing
  - 4.3. Declare first
  - 4.4. Freezing
- [5. Types](5_types/src/main.rs)
  - 5.1. Casting
  - 5.2. Literals
  - 5.3. Inference
  - 5.4. Aliasing
- [6. Conversion](6_conversion/src/main.rs)
  - 6.1. From and Into
  - 6.2. TryFrom and TryInto
  - 6.3. To and from Strings
- [7. Expressions](7_expressions/src/main.rs)
- [8. Flow of Control](8_flow_control/src/main.rs)
  - 8.1. if/else
  - 8.2. loop
    - 8.2.1 Nesting and labels
    - 8.2.1 Returning from loops
  - 8.3. while
  - 8.4. for and range

[comment]: <> (  - 8.5. match)
[comment]: <> (    - 8.5.1 Destructuring)
[comment]: <> (      - 8.5.1.1. tuples)
[comment]: <> (      - 8.5.1.2. enums)
[comment]: <> (      - 8.5.1.3. pointers/refs)
[comment]: <> (      - 8.5.1.4. structs)
[comment]: <> (    - 8.5.2 Guards)
[comment]: <> (    - 8.5.3 Binding)
[comment]: <> (  - 8.6. if let)
[comment]: <> (  - 8.7. while let)

## Run in terminal

All chapters have their own folders so binary run commands have the following format: `cargo run --manifest-path ./N_CHAPTER/Cargo.toml`

> ℹ️ Windows: use `cargo.exe` not `cargo`

Example:

```shell
cargo run --manifest-path ./2_primitives/Cargo.toml
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

To create a new chapter-related subfolder, please use the following format: `cargo new N_name --name name` 

#### Quick commands

> ℹ️ Update N and NAME variable values

Unix-like:
```shell
N=1; NAME=hello; cargo new "${N}_${NAME}" --name "_${N}_${NAME}"
```

Windows (Powershell):
```shell
$N='1'; $NAME='hello'; cargo new ${N}_${NAME} --name _${N}_${NAME}
```

## Code conduction

This project uses [Gitmoji](https://gitmoji.carloscuesta.me) for commit messages.

## License

[GPLv3+](LICENSE)
