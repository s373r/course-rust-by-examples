// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 23. Compatibility
//     23.1. Raw identifiers

fn main() {
    // https://doc.rust-lang.org/stable/rust-by-example/compatibility.html
    println!("\n--- 23. Compatibility ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/compatibility/raw_identifiers.html
    println!("\n--- 23.1. Raw identifiers ---");
    {
        // NOTE: `try` is reserved in Rust 2018
        // extern crate foo;
        //
        // foo::try();

        // NOTE: to fix that we use use raw identifiers
        // extern crate foo;
        //
        // foo::r#try();
    }
}
