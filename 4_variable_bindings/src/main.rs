// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 4. Variable Bindings
//    4.1. Mutability

fn main() {
    // https://doc.rust-lang.org/rust-by-example/variable_bindings.html
    println!();
    println!("--- 4. Variable Bindings ---");
    {
        let an_integer = 1u32;
        let a_boolean = true;
        let unit = ();

        // copy `an_integer` into `copied_integer`
        let copied_integer = an_integer;

        println!("An integer: {:?}", copied_integer);
        println!("A boolean: {:?}", a_boolean);
        println!("Meet the unit value: {:?}", unit);

        // The compiler warns about unused variable bindings; these warnings can
        // be silenced by prefixing the variable name with an underscore
        let _unused_variable = 3u32;

        // let noisy_unused_variable = 2u32;
        // DONE: ^ Prefix with an underscore to suppress the warning
    }

    // https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
    println!();
    println!("--- 4.1. Mutability ---");
    {
        let _immutable_binding = 1;
        let mut mutable_binding = 1;

        println!("Before mutation: {}", mutable_binding);

        // Ok
        mutable_binding += 1;

        println!("After mutation: {}", mutable_binding);

        // Error!
        // _immutable_binding += 1;
        // DONE: ^ Comment out this line
    }
}
