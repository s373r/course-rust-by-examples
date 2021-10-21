// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 22. Unsafe Operations

fn main() {
    // https://doc.rust-lang.org/stable/rust-by-example/unsafe.html
    println!("\n--- 22. Unsafe Operations ---");
    {
        {
            let raw_p: *const u32 = &10;

            unsafe {
                assert_eq!(*raw_p, 10);
            }
        }
        {
            use std::slice;

            let some_vector = vec![1, 2, 3, 4];

            let pointer = some_vector.as_ptr();
            let length = some_vector.len();

            unsafe {
                let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

                assert_eq!(some_vector.as_slice(), my_slice);
            }
        }
    }
}
