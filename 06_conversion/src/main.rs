// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 6. Conversion
//    6.1. From and Into
//    6.2. TryFrom and TryInto
//    6.3. To and from Strings

fn main() {
    // https://doc.rust-lang.org/rust-by-example/conversion.html
    println!("\n--- 6. Conversion ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
    println!("\n--- 6.1. From and Into ---");
    {
        let my_str = "hello";
        let _my_string = String::from(my_str);

        // NOTE: Included to Rust 2018 prelude
        // use std::convert::From;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        let num = Number::from(30);
        println!("My number is {:?}", num);

        let int = 5;
        // DONE: Try removing the type declaration
        //       NOTE: Compile error
        let num: Number = int.into();
        println!("My number is {:?}", num);
    }

    // https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html
    println!("\n--- 6.2. TryFrom and TryInto ---");
    {
        use std::convert::TryFrom;
        use std::convert::TryInto;

        #[derive(Debug, PartialEq)]
        struct EvenNumber(i32);

        impl TryFrom<i32> for EvenNumber {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNumber(value))
                } else {
                    Err(())
                }
            }
        }

        // TryFrom
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        // TryInto
        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }

    // https://doc.rust-lang.org/rust-by-example/conversion/string.html
    println!("\n--- 6.3. To and from Strings ---");
    {
        use std::fmt;

        struct Circle {
            radius: i32,
        }

        impl fmt::Display for Circle {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Circle of radius {}", self.radius)
            }
        }

        let circle = Circle { radius: 6 };
        println!("{}", circle.to_string());

        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();
        // NOTE:                          ^^^^^^^^^
        // NOTE: that is called: turbofish syntax

        let sum = parsed + turbo_parsed;
        println!("Sum: {:?}", sum);
    }
}
