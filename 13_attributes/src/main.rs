// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 13. Attributes
//    13.1. dead_code
//    13.2. Crates
//    13.3. cfg
//          13.3.1. Custom

fn main() {
    // https://doc.rust-lang.org/rust-by-example/attribute.html
    println!("\n--- 13. Attributes ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/attribute/unused.html
    println!("\n--- 13.1. dead_code ---");
    {
        fn used_function() {}

        // `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
        #[allow(dead_code)]
        fn unused_function() {}

        #[allow(dead_code)]
        fn noisy_unused_function() {}
        // DONE: ^ Add an attribute to suppress the warning

        used_function();
    }

    // https://doc.rust-lang.org/rust-by-example/attribute/crate.html
    println!("\n--- 13.2. Crates ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
    println!("\n--- 13.3. cfg ---");
    {
        // This function only gets compiled if the target OS is linux
        #[cfg(target_os = "linux")]
        // NOTE: shorted variant
        #[cfg(linux)]
        fn are_you_on_linux() {
            println!("You are running linux!");
        }

        // And this function only gets compiled if the target OS is *not* linux
        #[cfg(not(target_os = "linux"))]
        // NOTE: shorted variant
        // #[cfg(not(linux))]
        fn are_you_on_linux() {
            println!("You are *not* running linux!");
        }

        are_you_on_linux();

        println!("Are you sure?");
        // if cfg!(target_os = "linux") {
        // NOTE: shorter variant
        if cfg!(linux) {
            println!("Yes. It's definitely linux!");
        } else {
            println!("Yes. It's definitely *not* linux!");
        }
    }

    // https://doc.rust-lang.org/rust-by-example/attribute/cfg/custom.html
    println!("\n--- 13.3.1. Custom ---");
    {
        // #[cfg(some_condition)]
        // NOTE: alternative: feature-based approach
        #[cfg(feature = "some_condition")]
        fn conditional_function() {
            println!("condition met!");
        }

        conditional_function();
    }
}
