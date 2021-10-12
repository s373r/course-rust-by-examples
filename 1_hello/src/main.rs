// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 1. Hello World
//    1.1 Comments
//    1.2 Formatted print
//        1.2.1 Debug
//        1.2.2 Display

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

    //
    // --- 1.2.2 Display ---
    // https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
    //
    {
        // Import (via `use`) the `fmt` module to make it available.
        use std::fmt;

        // Define a structure for which `fmt::Display` will be implemented. This is
        // a tuple struct named `Structure` that contains an `i32`.
        struct Structure(i32);

        // To use the `{}` marker, the trait `fmt::Display` must be implemented
        // manually for the type.
        impl fmt::Display for Structure {
            // This trait requires `fmt` with this exact signature.
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Write strictly the first element into the supplied output
                // stream: `f`. Returns `fmt::Result` which indicates whether the
                // operation succeeded or failed. Note that `write!` uses syntax which
                // is very similar to `println!`.
                write!(f, "{}", self.0)
            }
        }

        // A structure holding two numbers. `Debug` will be derived so the results can
        // be contrasted with `Display`.
        #[derive(Debug)]
        struct MinMax(i64, i64);

        // Implement `Display` for `MinMax`.
        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Use `self.number` to refer to each positional data point.
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        // Define a structure where the fields are nameable for comparison.
        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }

        // Similarly, implement `Display` for `Point2D`
        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Customize so only `x` and `y` are denoted.
                write!(f, "x: {}, y: {}", self.x, self.y)
            }
        }

        let minmax = MinMax(0, 14);

        println!("Compare structures:");
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range = MinMax(-300, 300);
        let small_range = MinMax(-3, 3);

        println!(
            "The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range
        );

        let point = Point2D { x: 3.3, y: 7.2 };

        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        // Error. Both `Debug` and `Display` were implemented, but `{:b}`
        // requires `fmt::Binary` to be implemented. This will not work.
        // println!("What does Point2D look like in binary: {:b}?", point);

        // --- Activity ---
        // [x] After checking the output of the above example, use the Point2D struct as
        //     a guide to add a Complex struct to the example. When printed in the same way,
        //     the output should be:
        //     > Display: 3.3 + 7.2i
        //     > Debug: Complex { real: 3.3, imag: 7.2 }

        #[derive(Debug)]
        struct Complex {
            real: f64,
            imaq: f64,
        }

        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Customize so only `x` and `y` are denoted.
                write!(f, "{} + {}i", self.real, self.imaq)
            }
        }

        let complex_number = Complex {
            real: 3.3,
            imaq: 7.2,
        };

        println!("Display: {}", complex_number);
        println!("Debug: {:?}", complex_number);
    }
}
