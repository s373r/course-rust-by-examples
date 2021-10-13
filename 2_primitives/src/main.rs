// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 2. Primitives

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
}
