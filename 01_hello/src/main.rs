// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 1. Hello World
//    1.1. Comments
//    1.2. Formatted print
//        1.2.1. Debug
//        1.2.2. Display
//               1.2.2.1. Testcase: List
//        1.2.3. Formatting

//! Generate library docs for the enclosing item.

/// Generate library docs for the following item.
fn main() {
    // https://doc.rust-lang.org/rust-by-example/hello.html
    println!();
    println!("--- 1. Hello World ---");
    {
        println!("Hello, world!");

        println!();
        println!("--- Activity ---");
        //        [x] Click 'Run' above to see the expected output. Next, add a new line with
        //            a second println! macro so that the output shows:
        //            > Hello World!
        //            > I'm a Rustacean!
        println!("I'm a Rustacean!");
    }

    // https://doc.rust-lang.org/rust-by-example/hello/comment.html
    println!();
    println!("--- 1.1. Comments ---");
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

    // https://doc.rust-lang.org/rust-by-example/hello/print.html
    println!();
    println!("--- 1.2. Formatted print ---");
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

        println!();
        println!("--- Activities ---");
        //        [x] Fix the two issues in the above code (see FIXME) so that it runs without
        //            error.
        //        [x] Add a println! macro that prints: Pi is roughly 3.142 by controlling
        //            the number of decimal places shown. For the purposes of this exercise,
        //            use let pi = 3.141592 as an estimate for pi.
        //            (Hint: you may need to check the std::fmt documentation for setting
        //            the number of decimals to display)

        let pi = std::f64::consts::PI;
        println!("Pi is roughly {:.3}", pi);
    }

    // https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
    println!();
    println!("--- 1.2.1. Debug ---");
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

    // https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
    println!();
    println!("--- 1.2.2. Display ---");
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

        println!();
        println!("--- Activity ---");
        //        [x] After checking the output of the above example, use the Point2D struct as
        //            a guide to add a Complex struct to the example. When printed in the same way,
        //            the output should be:
        //            > Display: 3.3 + 7.2i
        //            > Debug: Complex { real: 3.3, imag: 7.2 }

        #[derive(Debug)]
        struct Complex {
            real: f64,
            imaq: f64,
        }

        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    // https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html
    println!();
    println!("--- 1.2.2.1. Testcase: List ---");
    {
        {
            use std::fmt; // Import the `fmt` module.

            // Define a structure named `List` containing a `Vec`.
            struct List(Vec<i32>);

            impl fmt::Display for List {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    // Extract the value using tuple indexing,
                    // and create a reference to `vec`.
                    let vec = &self.0;

                    write!(f, "[")?;

                    // Iterate over `v` in `vec` while enumerating the iteration
                    // count in `count`.
                    for (count, v) in vec.iter().enumerate() {
                        // For every element except the first, add a comma.
                        // Use the ? operator to return on errors.
                        if count != 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", v)?;
                    }

                    // Close the opened bracket and return a fmt::Result value.
                    write!(f, "]")
                }
            }

            let v = List(vec![1, 2, 3]);

            println!("{}", v);
        }

        println!();
        println!("--- Activity ---");
        //        [x] Try changing the program so that the index of each element in the vector
        //            is also printed. The new output should look like this:
        //            > [0: 1, 1: 2, 2: 3]
        {
            use std::fmt;

            struct List(Vec<i32>);

            impl fmt::Display for List {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let vec = &self.0;

                    write!(f, "[")?;

                    for (index, value) in vec.iter().enumerate() {
                        if index != 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {}", index, value)?;
                    }

                    write!(f, "]")
                }
            }

            let v = List(vec![1, 2, 3]);

            println!("{}", v);
        }
    }

    // https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html
    println!();
    println!("--- 1.2.3. Formatting ---");
    {
        {
            use std::fmt::{self, Display, Formatter};

            struct City {
                name: &'static str,
                // Latitude
                lat: f32,
                // Longitude
                lon: f32,
            }

            impl Display for City {
                // `f` is a buffer, and this method must write the formatted string into it
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
                    let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

                    // `write!` is like `format!`, but it will write the formatted string
                    // into a buffer (the first argument)
                    write!(
                        f,
                        "{}: {:.3}°{} {:.3}°{}",
                        self.name,
                        self.lat.abs(),
                        lat_c,
                        self.lon.abs(),
                        lon_c
                    )
                }
            }

            #[derive(Debug)]
            struct Color {
                red: u8,
                green: u8,
                blue: u8,
            }

            for city in [
                City {
                    name: "Dublin",
                    lat: 53.347778,
                    lon: -6.259722,
                },
                City {
                    name: "Oslo",
                    lat: 59.95,
                    lon: 10.75,
                },
                City {
                    name: "Vancouver",
                    lat: 49.25,
                    lon: -123.1,
                },
            ]
            .iter()
            {
                println!("{}", *city);
            }
            for color in [
                Color {
                    red: 128,
                    green: 255,
                    blue: 90,
                },
                Color {
                    red: 0,
                    green: 3,
                    blue: 254,
                },
                Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
            ]
            .iter()
            {
                // Switch this to use {} once you've added an implementation
                // for fmt::Display.
                println!("{:?}", *color);
            }
        }
        println!();
        println!("--- Activity ---");
        //        [x] Add an implementation of the fmt::Display trait for the Color struct
        //            above so that the output displays as:
        //            > RGB (128, 255, 90) 0x80FF5A
        //            > RGB (0, 3, 254) 0x0003FE
        //            > RGB (0, 0, 0) 0x000000
        {
            use std::fmt::{self, Display, Formatter};

            #[derive(Debug)]
            struct Color {
                red: u8,
                green: u8,
                blue: u8,
            }

            impl Display for Color {
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    write!(
                        f,
                        "RGB ({r}, {g}, {b}) 0x{r:02X}{g:02X}{b:02X}",
                        r = self.red,
                        g = self.green,
                        b = self.blue,
                    )
                }
            }

            let colors = [
                Color {
                    red: 128,
                    green: 255,
                    blue: 90,
                },
                Color {
                    red: 0,
                    green: 3,
                    blue: 254,
                },
                Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
            ];

            for color in colors.iter() {
                println!("{}", *color);
            }
        }
    }
}