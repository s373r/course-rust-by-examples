// SPDX-License-Identifier: GPL-3.0-or-later

// 1. Hello World  | https://doc.rust-lang.org/rust-by-example/hello.html
//    1.1 Comments | https://doc.rust-lang.org/rust-by-example/hello/comment.html

//! Generate library docs for the enclosing item.

/// Generate library docs for the following item.
fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // Regular comments which are ignored by the compiler:
    // Line comments which go to the end of the line.
    /* Block comments which go to the closing delimiter. */
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // Doc comments which are parsed into HTML library documentation:
    // (see above) Generate library docs for the following item.
    // (see above) Generate library docs for the enclosing item.
}
