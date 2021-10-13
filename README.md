# Course: Rust By Examples

Course link: https://doc.rust-lang.org/stable/rust-by-example

## Index

- [1. Hello World](1_hello/src/main.rs)
  - 1\.1. Comments
  - 1\.2. Formatted print
    - 1\.2.1. Debug
    - 1\.2.2. Display
      - 1\.2.2.1. Testcase: List
    - 1\.2.3. Formatting
- [2. Primitives](2_primitives/src/main.rs)
  - 2\.1. Literals and operators
  - 2\.2. Tuples
  - 2\.3. Arrays and Slices
- [3. Custom types](3_custom_types/src/main.rs)
  - 3.\1. Structures
  - 3.\2. Enums
    - 3\.2.1. use
    - 3\.2.2. C-like
    - 3\.2.3. Testcase: linked-list
  - 3.\3. constants
- [4. Variable Bindings](4_variable_bindings/src/main.rs)
  - 4.\1. Mutability

## Run in terminal

> ℹ️ Windows: use `cargo.exe` not `cargo`

```shell
# 1. Hello World
cargo run --manifest-path ./1_hello/Cargo.toml
```
```shell
# 2. Primitives
cargo run --manifest-path ./2_primitives/Cargo.toml
```
```shell
# 3. Custom types
cargo run --manifest-path ./3_custom_types/Cargo.toml
```
```shell
# 4. Custom types
cargo run --manifest-path ./4_variable_bindings/Cargo.toml
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
