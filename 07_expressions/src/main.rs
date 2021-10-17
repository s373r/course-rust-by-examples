// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 7. Expressions

fn main() {
    // https://doc.rust-lang.org/rust-by-example/expression.html
    println!("\n--- 7. Expressions ---");
    {
        // variable binding
        let x = 5;

        // expression;
        x;
        x + 1;
        15;

        let x = 5u32;

        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;

            // This expression will be assigned to `y`
            x_cube + x_squared + x
        };

        let z = {
            // The semicolon suppresses this expression and `()` is assigned to `z`
            2 * x;
        };

        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }
}
