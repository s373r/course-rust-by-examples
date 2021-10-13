// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 8. Flow of Control
//    8.1. if/else
//    8.2. loop
//         8.2.1. Nesting and labels
//         8.2.2. Returning from loops
//    8.3. while
//    8.4. for and range
//    8.5. match
//         8.5.1 Destructuring
//               8.5.1.1. tuples
//               8.5.1.2. enums
//               8.5.1.3. pointers/refs
//               8.5.1.4. structs
//         8.5.2 Guards
//         8.5.3 Binding
//    8.6. if let
//    8.7. while let

fn main() {
    // https://doc.rust-lang.org/rust-by-example/flow_control.html
    println!("\n--- 8. Flow of Control ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html
    println!("\n--- 8.1. if/else ---");
    {
        let n = 5;

        if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }

        let big_n = if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // DONE: ^ Try suppressing this expression with a semicolon.
            //         NOTE: compile error
        };
        //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

        println!("{} -> {}", n, big_n);
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
    println!("\n--- 8.2. loop ---");
    {
        let mut count = 0u32;

        println!("Let's count until infinity!");

        // Infinite loop
        loop {
            count += 1;

            if count == 3 {
                println!("three");

                // Skip the rest of this iteration
                continue;
            }

            println!("{}", count);

            if count == 5 {
                println!("OK, that's enough");

                // Exit this loop
                break;
            }
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
    println!("\n--- 8.2.1. Nesting and labels ---");
    {
        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                // This would break only the inner loop
                // break;

                // This breaks the outer loop
                break 'outer;
            }

            println!("This point will never be reached");
        }

        println!("Exited the outer loop");
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/loop/return.html
    println!("\n--- 8.2.2. Returning from loops ---");
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/while.html
    println!("\n--- 8.3. while ---");
    {
        // A counter variable
        let mut n = 1;

        // Loop while `n` is less than 101
        while n < 101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }

            // Increment counter
            n += 1;
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/for.html
    println!("\n--- 8.4. for and range ---");
    {
        // `n` will take the values: 1, 2, ..., 100 in each iteration
        // for n in 1..=100 {
        for n in 1..101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
        }

        {
            let names = vec!["Bob", "Frank", "Ferris"];

            for name in names.iter() {
                match name {
                    &"Ferris" => println!("There is a rustacean among us!"),
                    // DONE: ^ Try deleting the & and matching just "Ferris"
                    //       NOTE: compile error
                    _ => println!("Hello {}", name),
                }
            }

            println!("names: {:?}", names);
        }
        {
            let names = vec!["Bob", "Frank", "Ferris"];

            for name in names.into_iter() {
                match name {
                    "Ferris" => println!("There is a rustacean among us!"),
                    _ => println!("Hello {}", name),
                }
            }

            // println!("names: {:?}", names);
            // DONE: ^ Comment out this line
        }
        {
            let mut names = vec!["Bob", "Frank", "Ferris"];

            for name in names.iter_mut() {
                *name = match name {
                    &mut "Ferris" => "There is a rustacean among us!",
                    _ => "Hello",
                }
            }

            println!("names: {:?}", names);
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    // println!("\n--- 8.5. match ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html
    // println!("\n--- 8.5.1 Destructuring ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html
    // println!("\n--- 8.5.1.1. tuples ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_enum.html
    // println!("\n--- 8.5.1.2. enums ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
    // println!("\n--- 8.5.1.3. pointers/refs ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_structures.html
    // println!("\n--- 8.5.1.4. structs ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html
    // println!("\n--- 8.5.2 Guards ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html
    // println!("\n--- 8.5.3 Binding ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    // println!("\n--- 8.6. if let ---");
    // {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html
    // println!("\n--- 8.7. while let ---");
    // {}
}
