// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 10.5. File hierarchy

mod my;

fn main() {
    // https://doc.rust-lang.org/rust-by-example/mod/split.html
    println!("\n--- 10.5. File hierarchy ---");
    {
        // This declaration will look for a file named `my.rs` or `my/mod.rs` and will
        // insert its contents inside a module named `my` under this scope
        // mod my;
        // NOTE: ^ moved to top

        fn function() {
            println!("called `function()`");
        }

        my::function();

        function();

        my::indirect_access();

        my::nested::function();
    }
}
