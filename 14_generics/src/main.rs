// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 14. Generics
//    14.1. Functions
//    14.2. Implementation
//    14.3. Traits
//    14.4. Bounds
//          14.4.1. Testcase: empty bounds
//    14.5. Multiple bounds
//    14.6. Where clauses
//    14.7. New Type Idiom
//    14.8. Associated items
//          14.8.1. The Problem
//          14.8.2. Associated types
//    14.9. Phantom type parameters
//          14.9.1. Testcase: unit clarification

fn main() {
    // https://doc.rust-lang.org/rust-by-example/generics.html
    println!("\n--- 14. Generics ---");
    {
        // A concrete type `A`.
        struct A;

        // In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
        // Therefore, `Single` is a concrete type, and `A` is defined as above.
        struct Single(A);
        //            ^ Here is `Single`s first use of the type `A`.

        // Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
        // Because the type parameter `T` is generic, it could be anything, including
        // the concrete type `A` defined at the top.
        struct SingleGen<T>(T);

        // `Single` is concrete and explicitly takes `A`.
        let _s = Single(A);

        // Create a variable `_char` of type `SingleGen<char>`
        // and give it the value `SingleGen('a')`.
        // Here, `SingleGen` has a type parameter explicitly specified.
        let _char: SingleGen<char> = SingleGen('a');

        // `SingleGen` can also have a type parameter implicitly specified:
        let _t = SingleGen(A); // Uses `A` defined at the top.
        let _i32 = SingleGen(6); // Uses `i32`.
        let _char = SingleGen('a'); // Uses `char`.

        // NOTE: compile error
        // let _2_ints = SingleGen(1, 2);
    }

    // https://doc.rust-lang.org/rust-by-example/generics/gen_fn.html
    println!("\n--- 14.1. Functions ---");
    {
        struct A; // Concrete type `A`.
        struct S(A); // Concrete type `S`.
        struct SGen<T>(T); // Generic type `SGen`.

        // The following functions all take ownership of the variable passed into
        // them and immediately go out of scope, freeing the variable.

        // Define a function `reg_fn` that takes an argument `_s` of type `S`.
        // This has no `<T>` so this is not a generic function.
        fn reg_fn(_s: S) {}

        // Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
        // It has been explicitly given the type parameter `A`, but because `A` has not
        // been specified as a generic type parameter for `gen_spec_t`, it is not generic.
        fn gen_spec_t(_s: SGen<A>) {}

        // Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
        // It has been explicitly given the type parameter `i32`, which is a specific type.
        // Because `i32` is not a generic type, this function is also not generic.
        fn gen_spec_i32(_s: SGen<i32>) {}

        // Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
        // Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
        fn generic<T>(_s: SGen<T>) {}

        // Using the non-generic functions
        reg_fn(S(A)); // Concrete type.
        gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
        gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

        // Explicitly specified type parameter `char` to `generic()`.
        generic::<char>(SGen('a'));

        // Implicitly specified type parameter `char` to `generic()`.
        generic(SGen('c'));
    }

    // https://doc.rust-lang.org/rust-by-example/generics/impl.html
    println!("\n--- 14.2. Implementation ---");
    {
        struct Val {
            val: f64,
        }

        struct GenVal<T> {
            gen_val: T,
        }

        // impl of Val
        impl Val {
            fn value(&self) -> &f64 {
                &self.val
            }
        }

        // impl of GenVal for a generic type `T`
        impl<T> GenVal<T> {
            fn value(&self) -> &T {
                &self.gen_val
            }
        }

        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };

        println!("{}, {}", x.value(), y.value());
    }

    // https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html
    println!("\n--- 14.3. Traits ---");
    {
        // Non-copyable types.
        struct Empty;
        struct Null;

        // A trait generic over `T`.
        trait DoubleDrop<T> {
            // Define a method on the caller type which takes an
            // additional single parameter `T` and does nothing with it.
            fn double_drop(self, _: T);
        }

        // Implement `DoubleDrop<T>` for any generic parameter `T` and
        // caller `U`.
        impl<T, U> DoubleDrop<T> for U {
            // This method takes ownership of both passed arguments,
            // deallocating both.
            fn double_drop(self, _: T) {}
        }

        let empty = Empty;
        let null = Null;

        // Deallocate `empty` and `null`.
        empty.double_drop(null);

        // empty;
        // null;
        // ^ DONE: Try uncommenting these lines.
        //         NOTE: "use after moving" errors
    }

    // https://doc.rust-lang.org/rust-by-example/generics/bounds.html
    println!("\n--- 14.4. Bounds ---");
    {
        // A trait which implements the print marker: `{:?}`.
        use std::fmt::Debug;

        trait HasArea {
            fn area(&self) -> f64;
        }

        impl HasArea for Rectangle {
            fn area(&self) -> f64 {
                self.length * self.height
            }
        }

        #[derive(Debug)]
        struct Rectangle {
            length: f64,
            height: f64,
        }
        #[allow(dead_code)]
        struct Triangle {
            length: f64,
            height: f64,
        }

        // The generic `T` must implement `Debug`. Regardless
        // of the type, this will work properly.
        fn print_debug<T: Debug>(t: &T) {
            println!("{:?}", t);
        }

        // `T` must implement `HasArea`. Any type which meets
        // the bound can access `HasArea`'s function `area`.
        fn area<T: HasArea>(t: &T) -> f64 {
            t.area()
        }

        let rectangle = Rectangle {
            length: 3.0,
            height: 4.0,
        };

        let _triangle = Triangle {
            length: 3.0,
            height: 4.0,
        };

        print_debug(&rectangle);
        println!("Area: {}", area(&rectangle));

        // print_debug(&_triangle);
        // println!("Area: {}", area(&_triangle));
        // ^ DONE: Try uncommenting these.
        // | Error: Does not implement either `Debug` or `HasArea`.
    }

    // https://doc.rust-lang.org/rust-by-example/generics/bounds/testcase_empty.html
    println!("\n--- 14.4.1. Testcase: empty bounds ---");
    {
        struct Cardinal;
        struct BlueJay;
        struct Turkey;

        trait Red {}
        trait Blue {}

        impl Red for Cardinal {}
        impl Blue for BlueJay {}

        // These functions are only valid for types which implement these
        // traits. The fact that the traits are empty is irrelevant.
        fn red<T: Red>(_: &T) -> &'static str {
            "red"
        }
        fn blue<T: Blue>(_: &T) -> &'static str {
            "blue"
        }

        let cardinal = Cardinal;
        let blue_jay = BlueJay;
        let _turkey = Turkey;

        // `red()` won't work on a blue jay nor vice versa
        // because of the bounds.
        println!("A cardinal is {}", red(&cardinal));
        println!("A blue jay is {}", blue(&blue_jay));
        // println!("A turkey is {}", red(&_turkey));
        // ^ DONE: Try uncommenting this line.
        //         NOTE: compile error as expected
    }

    // https://doc.rust-lang.org/rust-by-example/generics/multi_bounds.html
    // println!("\n--- 14.5. Multiple bounds ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/generics/where.html
    // println!("\n--- 14.6. Where clauses ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/generics/new_types.html
    // println!("\n--- 14.7. New Type Idiom ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/generics/assoc_items.html
    // println!("\n--- 14.8. Associated items ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/generics/assoc_items/the_problem.html
    // println!("\n--- 14.8.1. The Problem ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html
    // println!("\n--- 14.8.2. Associated types ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/generics/phantom.html
    // println!("\n--- 14.9. Phantom type parameters ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/generics/phantom/testcase_units.html
    // println!("\n--- 14.9.1. Testcase: unit clarification ---");
    {}
}
