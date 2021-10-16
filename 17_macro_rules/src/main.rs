// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 17. macro_rules!
//    17.1. Syntax
//          17.1.1. Designators
//          17.1.2. Overload
//          17.1.3. Repeat
//    17.2. DRY (Don't Repeat Yourself)
//    17.3. Domain Specific Languages (DSLs)
//    17.4. Variadic Interfaces

fn main() {
    // https://doc.rust-lang.org/stable/rust-by-example/macros.html
    println!("\n--- 17. macro_rules! ---");
    {
        // This is a simple macro named `say_hello`.
        macro_rules! say_hello {
            // `()` indicates that the macro takes no argument.
            () => {
                // The macro will expand into the contents of this block.
                println!("Hello!");
            };
        }

        // This call will expand into `println!("Hello");`
        say_hello!()
    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/syntax.html
    println!("\n--- 17.1. Syntax ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/macros/designators.html
    println!("\n--- 17.1.1. Designators ---");
    {
        // These are some of the available designators:
        // - block
        // - expr is used for expressions
        // - ident is used for variable/function names
        // - item
        // - literal is used for literal constants
        // - pat (pattern)
        // - path
        // - stmt (statement)
        // - tt (token tree)
        // - ty (type)
        // - vis (visibility qualifier)

        macro_rules! create_function {
            // This macro takes an argument of designator `ident` and
            // creates a function named `$func_name`.
            // The `ident` designator is used for variable/function names.
            ($func_name:ident) => {
                fn $func_name() {
                    // The `stringify!` macro converts an `ident` into a string.
                    println!("You called {:?}()", stringify!($func_name));
                }
            };
        }

        // Create functions named `foo` and `bar` with the above macro.
        create_function!(foo);
        create_function!(bar);

        macro_rules! print_result {
            // This macro takes an expression of type `expr` and prints
            // it as a string along with its result.
            // The `expr` designator is used for expressions.
            ($expression:expr) => {
                // `stringify!` will convert the expression *as it is* into a string.
                println!("{:?} = {:?}", stringify!($expression), $expression);
            };
        }

        foo();
        bar();

        print_result!(1u32 + 1);

        // Recall that blocks are expressions too!
        print_result!({
            let x = 1u32;

            x * x + 2 * x - 1
        });
    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/overload.html
    println!("\n--- 17.1.2. Overload ---");
    {
        // `test!` will compare `$left` and `$right`
        // in different ways depending on how you invoke it:
        macro_rules! test {
            // Arguments don't need to be separated by a comma.
            // Any template can be used!
            ($left:expr; and $right:expr) => {
                println!(
                    "{:?} and {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left && $right
                )
            };
            // ^ each arm must end with a semicolon.
            ($left:expr; or $right:expr) => {
                println!(
                    "{:?} or {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left || $right
                )
            };
        }

        test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
        test!(true; or false);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/repeat.html
    println!("\n--- 17.1.3. Repeat ---");
    {
        // `find_min!` will calculate the minimum of any number of arguments.
        macro_rules! find_min {
            // Base case:
            ($x:expr) => ($x);
            // `$x` followed by at least one `$y,`
            ($x:expr, $($y:expr),+) => (
                // Call `find_min!` on the tail `$y`
                std::cmp::min($x, find_min!($($y),+))
            )
        }

        println!("{}", find_min!(1u32));
        println!("{}", find_min!(1u32 + 2, 2u32));
        println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/dry.html
    println!("\n--- 17.2. DRY (Don't Repeat Yourself) ---");
    {
        use std::iter;
        use std::ops::{Add, Mul, Sub};

        macro_rules! assert_equal_len {
            // The `tt` (token tree) designator is used for
            // operators and tokens.
            ($a:expr, $b:expr, $func:ident, $op:tt) => {
                assert!(
                    $a.len() == $b.len(),
                    "{:?}: dimension mismatch: {:?} {:?} {:?}",
                    stringify!($func),
                    ($a.len(),),
                    stringify!($op),
                    ($b.len(),)
                );
            };
        }

        macro_rules! op {
            ($func:ident, $bound:ident, $op:tt, $method:ident) => {
                #[allow(dead_code)]
                fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
                    assert_equal_len!(xs, ys, $func, $op);

                    for (x, y) in xs.iter_mut().zip(ys.iter()) {
                        *x = $bound::$method(*x, *y);
                        // *x = x.$method(*y);
                    }
                }
            };
        }

        // Implement `add_assign`, `mul_assign`, and `sub_assign` functions.
        op!(add_assign, Add, +=, add);
        op!(mul_assign, Mul, *=, mul);
        op!(sub_assign, Sub, -=, sub);

        // NOTE: rewrite without unit-tests
        macro_rules! test {
            ($func:ident, $x:expr, $y:expr, $z:expr) => {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    $func(&mut x, &y);

                    assert_eq!(x, z);
                }
            };
        }

        // Test `add_assign`, `mul_assign`, and `sub_assign`.
        test!(add_assign, 1u32, 2u32, 3u32);
        test!(mul_assign, 2u32, 3u32, 6u32);
        test!(sub_assign, 3u32, 2u32, 1u32);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/dsl.html
    println!("\n--- 17.3. Domain Specific Languages (DSLs) ---");
    {
        macro_rules! calculate {
            (eval $e:expr) => {{
                {
                    let val: usize = $e; // Force types to be integers
                    println!("{} = {}", stringify!{$e}, val);
                }
            }};
        }

        calculate! {
            eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
        }

        calculate! {
            eval (1 + 2) * (3 / 4)
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/variadics.html
    println!("\n--- 17.4. Variadic Interfaces ---");
    {
        macro_rules! calculate {
            // The pattern for a single `eval`
            (eval $e:expr) => {{
                {
                    let val: usize = $e; // Force types to be integers
                    println!("{} = {}", stringify!{$e}, val);
                }
            }};

            // Decompose multiple `eval`s recursively
            (eval $e:expr, $(eval $es:expr),+) => {{
                calculate! { eval $e }
                calculate! { $(eval $es),+ }
            }};
        }

        calculate! { // Look ma! Variadic `calculate!`!
            eval 1 + 2,
            eval 3 + 4,
            eval (2 * 3) + 1
        }
    }
}
