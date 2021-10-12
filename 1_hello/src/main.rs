// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 1. Hello World
//    1.1 Comments
//    1.2 Formatted print
//        1.2.1 Debug

//! Generate library docs for the enclosing item.

/// Generate library docs for the following item.
fn main() {
    //
    // --- 1. Hello World ---
    // https://doc.rust-lang.org/rust-by-example/hello.html
    //
    {
        println!("Hello, world!");

        // --- Activity ---
        // [x] Click 'Run' above to see the expected output. Next, add a new line with a second println!
        //     macro so that the output shows:
        //     > Hello World!
        //     > I'm a Rustacean!
        println!("I'm a Rustacean!");
    }

    //
    // --- 1.1 Comments ---
    // https://doc.rust-lang.org/rust-by-example/hello/comment.html
    //
    {
        // Regular comments which are ignored by the compiler:
        // Line comments which go to the end of the line.
        /* Block comments which go to the closing delimiter. */
        let x = 5 + /* 90 + */ 5;

        println!("Is `x` 10 or 100? x = {}", x);

        // Doc comments which are parsed into HTML library documentation:
        // (see above) Generate library docs for the following item.
        // (see above) Generate library docs for the enclosing item.
    }

    //
    // --- 1.2 Formatted print ---
    // https://doc.rust-lang.org/rust-by-example/hello/print.html
    //
    {
        // In general, the `{}` will be automatically replaced with any
        // arguments. These will be stringified.
        println!("{} days", 31);

        // Without a suffix, 31 becomes an i32. You can change what type 31 is
        // by providing a suffix. The number 31i64 for example has the type i64.
        println!("{} seconds", 1543999222287i64);

        // There are various optional patterns this works with. Positional
        // arguments can be used.
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        // As can named arguments.
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );

        // Special formatting can be specified after a `:`.
        println!(
            "{} of {:b} people know binary, the other half doesn't",
            1, 2
        );

        // You can right-align text with a specified width. This will output
        // "     1". 5 white spaces and a "1".
        println!("{number:>width$}", number = 1, width = 6);
        println!("{number:>2}", number = 7);

        // You can pad numbers with extra zeroes. This will output "000001".
        println!("{number:0>width$}", number = 1, width = 6);

        // Rust even checks to make sure the correct number of arguments are
        // used.
        println!("My name is {0}, {1} {0}", "Bond", "James");

        // Create a structure named `Structure` which contains an `i32`.
        #[allow(dead_code)]
        struct Structure(i32);

        // However, custom types such as this structure require more complicated
        // handling. This will not work.
        // println!("This struct `{}` won't print...", Structure(3));

        // --- Activities ---
        // [x] Fix the two issues in the above code (see FIXME) so that it runs without error.
        // [x] Add a println! macro that prints: Pi is roughly 3.142 by controlling
        //     the number of decimal places shown. For the purposes of this exercise,
        //     use let pi = 3.141592 as an estimate for pi.
        //     (Hint: you may need to check the std::fmt documentation for setting
        //     the number of decimals to display)

        let pi = std::f64::consts::PI;
        println!("Pi is roughly {:.3}", pi);
    }

    //
    // --- 1.2.1 Debug ---
    // https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
    //
    {
        // This structure cannot be printed either with `fmt::Display` or
        // with `fmt::Debug`.
        #[allow(dead_code)]
        struct UnPrintable(i32);

        // The `derive` attribute automatically creates the implementation
        // required to make this `struct` printable with `fmt::Debug`.
        #[derive(Debug)]
        struct DebugPrintable(i32);

        // println!("{:?}", UnPrintable(1));
        println!("{:?}", DebugPrintable(2));

        // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
        // is a structure which contains a single `i32`.
        #[derive(Debug)]
        struct Structure(i32);

        // Put a `Structure` inside of the structure `Deep`. Make it printable
        // also.
        #[derive(Debug)]
        struct Deep(Structure);

        // Printing with `{:?}` is similar to with `{}`.
        println!("{:?} months in a year.", 12);
        println!(
            "{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor = "actor's"
        );

        // `Structure` is printable!
        println!("Now {:?} will print!", Structure(3));

        // The problem with `derive` is there is no control over how
        // the results look. What if I want this to just show a `7`?
        println!("Now {:?} will print!", Deep(Structure(7)));
    }
}
