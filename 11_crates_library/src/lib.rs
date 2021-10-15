// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 11.1. Creating a Library

// https://doc.rust-lang.org/rust-by-example/crates/lib.html

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
