// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 2. Primitives
//    2.1. Literals and operators
//    2.2. Tuples

fn main() {
    // https://doc.rust-lang.org/rust-by-example/primitives.html
    println!();
    println!("--- 2. Primitives ---");
    {
        // Variables can be type annotated.
        let logical: bool = true;

        println!("{}", logical);

        let a_float: f64 = 1.0; // Regular annotation
        let an_integer = 5i32; // Suffix annotation

        // Or a default will be used.
        let default_float = 3.0; // `f64`
        let default_integer = 7; // `i32`

        // A type can also be inferred from context
        let mut inferred_type = 12; // Type i64 is inferred from another line
        inferred_type = 4294967296i64;

        // A mutable variable's value can be changed.
        let mut mutable = 12; // Mutable `i32`
        mutable = 21;

        // Error! The type of a variable can't be changed.
        // mutable = true;

        // Variables can be overwritten with shadowing.
        let mutable = true;
    }

    // https://doc.rust-lang.org/rust-by-example/primitives/literals.html
    println!();
    println!("--- 2.1. Literals and operators ---");
    {
        // Integer addition
        println!("1 + 2 = {}", 1u32 + 2);

        // Integer subtraction
        println!("1 - 2 = {}", 1i32 - 2);
        // println!("1 - 2 = {}", 1u32 - 2); // NOTE: compilation error
        // DONE: ^ Try changing `1i32` to `1u32` to see why the type is important

        // Short-circuiting boolean logic
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);

        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);
    }

    // https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
    println!();
    println!("--- 2.2. Tuples ---");
    {
        // Tuples can be used as function arguments and as return values
        fn reverse(pair: (i32, bool)) -> (bool, i32) {
            // `let` can be used to bind the members of a tuple to variables
            let (integer, boolean) = pair;

            (boolean, integer)
        }

        // The following struct is for the activity.
        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);

        // A tuple with a bunch of different types
        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );

        // Values can be extracted from the tuple using tuple indexing
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        // Tuples can be tuple members
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        // Tuples are printable
        println!("tuple of tuples: {:?}", tuple_of_tuples);

        // But long Tuples cannot be printed
        // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("too long tuple: {:?}", too_long_tuple);
        // DONE: ^ Uncomment the above 2 lines to see the compiler error

        let pair = (1, true);
        println!("pair is {:?}", pair);

        println!("the reversed pair is {:?}", reverse(pair));

        // To create one element tuples, the comma is required to tell them apart
        // from a literal surrounded by parentheses
        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));

        //tuples can be destructured to create bindings
        let tuple = (1, "hello", 4.5, true);

        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);

        println!();
        println!("--- Activities ---");
        //        [x] Recap: Add the fmt::Display trait to the Matrix struct in the above example,
        //            so that if you switch from printing the debug format {:?} to the display
        //            format {}, you see the following output:
        //            > ( 1.1 1.2 )
        //            > ( 2.1 2.2 )
        //            You may want to refer back to the example for print display.
        //        [x] Add a transpose function using the reverse function as a template,
        //            which accepts a matrix as an argument, and returns a matrix in which
        //            two elements have been swapped. For example:
        //            > println!("Matrix:\n{}", matrix);
        //            > println!("Transpose:\n{}", transpose(matrix));
        //            results in the output:
        //            > Matrix:
        //            > ( 1.1 1.2 )
        //            > ( 2.1 2.2 )
        //            > Transpose:
        //            > ( 1.1 2.1 )
        //            > ( 1.2 2.2 )

        use std::fmt;

        impl fmt::Display for Matrix {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
            }
        }

        fn transpose(Matrix(a, b, c, d): Matrix) -> Matrix {
            Matrix(a, c, b, d)
        }

        println!("Matrix:\n{}", matrix);
        println!("Transpose:\n{}", transpose(matrix));
    }
}
