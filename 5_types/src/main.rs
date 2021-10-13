// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 5. Types
//    5.1. Casting
//    5.2. Literals
//    5.3. Inference
//    5.4. Aliasing

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    // https://doc.rust-lang.org/rust-by-example/types.html
    println!();
    println!("--- 5. Types ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/types/cast.html
    println!();
    println!("--- 5.1. Casting ---");
    {
        let decimal = 65.4321_f32;

        // Error! No implicit conversion
        // let integer: u8 = decimal;
        // DONE: ^ Comment out this line

        // Explicit conversion
        let integer = decimal as u8;
        let character = integer as char;

        // Error! There are limitations in conversion rules.
        // A float cannot be directly converted to a char.
        // let character = decimal as char;
        // DONE: ^ Comment out this line

        println!("Casting: {} -> {} -> {}", decimal, integer, character);

        // when casting any value to an unsigned type, T,
        // T::MAX + 1 is added or subtracted until the value
        // fits into the new type

        // 1000 already fits in a u16
        println!("1000 as a u16 is: {}", 1000 as u16);

        // 1000 - 256 - 256 - 256 = 232
        // Under the hood, the first 8 least significant bits (LSB) are kept,
        // while the rest towards the most significant bit (MSB) get truncated.
        println!("1000 as a u8 is : {}", 1000 as u8);
        // -1 + 256 = 255
        println!("  -1 as a u8 is : {}", (-1i8) as u8);

        // For positive numbers, this is the same as the modulus
        println!("1000 mod 256 is : {}", 1000 % 256);

        // When casting to a signed type, the (bitwise) result is the same as
        // first casting to the corresponding unsigned type. If the most significant
        // bit of that value is 1, then the value is negative.

        // Unless it already fits, of course.
        println!(" 128 as a i16 is: {}", 128 as i16);
        // 128 as u8 -> 128, whose two's complement in eight bits is:
        println!(" 128 as a i8 is : {}", 128 as i8);

        // repeating the example above
        // 1000 as u8 -> 232
        println!("1000 as a u8 is : {}", 1000 as u8);
        // and the two's complement of 232 is -24
        println!(" 232 as a i8 is : {}", 232 as i8);

        // Since Rust 1.45, the `as` keyword performs a *saturating cast*
        // when casting from float to int. If the floating point value exceeds
        // the upper bound or is less than the lower bound, the returned value
        // will be equal to the bound crossed.

        // 300.0 is 255
        println!("300.0 is {}", 300.0_f32 as u8);
        // -100.0 as u8 is 0
        println!("-100.0 as u8 is {}", -100.0_f32 as u8);
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN as u8);

        // This behavior incurs a small runtime cost and can be avoided
        // with unsafe methods, however the results might overflow and
        // return **unsound values**. Use these methods wisely:
        unsafe {
            // 300.0 is 44
            println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
            // -100.0 as u8 is 156
            println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
            // nan as u8 is 0
            println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
        }
    }

    // https://doc.rust-lang.org/rust-by-example/types/literals.html
    println!();
    println!("--- 5.2. Literals ---");
    {
        // Suffixed literals, their types are known at initialization
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        // Unsuffixed literals, their types depend on how they are used
        let i = 1;
        let f = 1.0;

        // `size_of_val` returns the size of a variable in bytes
        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }

    // https://doc.rust-lang.org/rust-by-example/types/inference.html
    println!();
    println!("--- 5.3. Inference ---");
    {
        // Because of the annotation, the compiler knows that `elem` has type u8.
        let elem = 5u8;

        // Create an empty vector (a growable array).
        let mut vec = Vec::new();
        // At this point the compiler doesn't know the exact type of `vec`, it
        // just knows that it's a vector of something (`Vec<_>`).

        // Insert `elem` in the vector.
        vec.push(elem);
        // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
        // DONE: ^ Try commenting out the `vec.push(elem)` line
        //       NOTE: compile error if commenting

        println!("{:?}", vec);
    }

    // https://doc.rust-lang.org/rust-by-example/types/alias.html
    println!();
    println!("--- 5.4. Aliasing ---");
    {
        // `NanoSecond` is a new name for `u64`.
        type NanoSecond = u64;
        type Inch = u64;

        // Use an attribute to silence warning.
        // #[allow(non_camel_case_types)]
        // type u64_t = u64;
        // DONE: ^ Try removing the attribute
        //       NOTE: warning will be produced

        // `NanoSecond` = `Inch` = `u64_t` = `u64`.
        let nanoseconds: NanoSecond = 5 as u64_t;
        let inches: Inch = 2 as u64_t;

        // Note that type aliases *don't* provide any extra type safety, because
        // aliases are *not* new types
        println!(
            "{} nanoseconds + {} inches = {} unit?",
            nanoseconds,
            inches,
            nanoseconds + inches
        );
    }
}
