// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 11. Creates
//    11.2. Using a Library

fn main() {
    // https://doc.rust-lang.org/rust-by-example/crates.html
    println!("\n--- 11. Creates ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/crates/using_lib.html
    println!("\n--- 11.2. Using a Library ---");
    {
        // NOTE: we build the library via Cargo.toml dependency
        //       and since we have own crates naming scheme the following `use`
        //       is needed
        use _11_crates_library as rary;

        rary::public_function();

        // Error! `private_function` is private
        // rary::private_function();

        rary::indirect_access();
    }
}
