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
    println!("\n--- 8.5. match ---");
    {
        let number = 1;
        // DONE: ^ Try different values for `number`

        println!("Tell me about {}", number);
        match number {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 /*| 13*/ => println!("This is a prime"),
            // DONE: ^ Try adding 13 to the list of prime values
            //       NOTE: next branch cannot be executed
            // Match an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
            // DONE: ^ Try commenting out this catch-all arm
            //       NOTE: compile error -- all branches should be covered
        }

        let boolean = true;
        // Match is an expression too
        let binary = match boolean {
            // The arms of a match must cover all the possible values
            false => 0,
            true => 1,
            // DONE: ^ Try commenting out one of these arms
            //       NOTE: compile error as expected
        };

        println!("{} -> {}", boolean, binary);
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html
    println!("\n--- 8.5.1 Destructuring ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html
    println!("\n--- 8.5.1.1. tuples ---");
    {
        let triple = (1, -2, 3);
        // DONE: ^ Try different values for `triple`

        println!("Tell me about {:?}", triple);
        // Match can be used to destructure a tuple
        match triple {
            // Destructure the second and third elements
            (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
            (1, ..) => println!("First is `1` and the rest doesn't matter"),
            // `..` can be used to ignore the rest of the tuple
            _ => println!("It doesn't matter what they are"),
            // `_` means don't bind the value to a variable
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_enum.html
    println!("\n--- 8.5.1.2. enums ---");
    {
        // `allow` required to silence warnings because only one variant is used.
        #[allow(dead_code)]
        enum Color {
            // These 3 are specified solely by their name.
            Red,
            Blue,
            Green,
            // These likewise tie `u32` tuples to different names: color models.
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }

        let color = Color::HSL(122, 17, 40);
        // DONE: ^ Try different variants for `color`

        println!("What color is it?");
        // An `enum` can be destructured using a `match`.
        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) => println!(
                "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k
            ),
            // Don't need another arm because all variants have been examined
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
    println!("\n--- 8.5.1.3. pointers/refs ---");
    {
        // Assign a reference of type `i32`. The `&` signifies there
        // is a reference being assigned.
        let reference = &4;

        match reference {
            // If `reference` is pattern matched against `&val`, it results
            // in a comparison like:
            // `&i32`
            // `&val`
            // ^ We see that if the matching `&`s are dropped, then the `i32`
            // should be assigned to `val`.
            &val => println!("Got a value via destructuring: {:?}", val),
        }

        // To avoid the `&`, you dereference before matching.
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }

        // What if you don't start with a reference? `reference` was a `&`
        // because the right side was already a reference. This is not
        // a reference because the right side is not one.
        let _not_a_reference = 3;

        // Rust provides `ref` for exactly this purpose. It modifies the
        // assignment so that a reference is created for the element; this
        // reference is assigned.
        let ref _is_a_reference = 3;

        // Accordingly, by defining 2 values without references, references
        // can be retrieved via `ref` and `ref mut`.
        let value = 5;
        let mut mut_value = 6;

        // Use `ref` keyword to create a reference.
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        // Use `ref mut` similarly.
        match mut_value {
            ref mut m => {
                // Got a reference. Gotta dereference it before we can
                // add anything to it.
                *m += 10;
                println!("We added 10. `mut_value`: {:?}", m);
            }
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_structures.html
    println!("\n--- 8.5.1.4. structs ---");
    {
        struct Foo {
            x: (u32, u32),
            y: u32,
        }

        // DONE: Try changing the values in the struct to see what happens
        let foo = Foo { x: (3, 2), y: 3 };

        match foo {
            Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

            // you can destructure structs and rename the variables,
            // the order is not important
            Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

            // and you can also ignore some variables:
            Foo { y, .. } => println!("y = {}, we don't care about x", y),
            // this will give an error: pattern does not mention field `x`
            //Foo { y } => println!("y = {}", y),
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html
    println!("\n--- 8.5.2 Guards ---");
    {
        let pair = (2, 2);
        // DONE: ^ Try different values for `pair`

        println!("Tell me about {:?}", pair);
        match pair {
            (x, y) if x == y => println!("These are twins"),
            // The ^ `if condition` part is a guard
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html
    println!("\n--- 8.5.3 Binding ---");
    {
        // A function `age` which returns a `u32`.
        fn age() -> u32 {
            29
        }

        println!("Tell me what type of person you are");

        match age() {
            0 => println!("I haven't celebrated my first birthday yet"),
            // Could `match` 1 ..= 12 directly but then what age
            // would the child be? Instead, bind to `n` for the
            // sequence of 1 ..= 12. Now the age can be reported.
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            // Nothing bound. Return the result.
            n => println!("I'm an old person of age {:?}", n),
        }

        fn some_number() -> Option<u32> {
            Some(43)
        }

        match some_number() {
            // Got `Some` variant, match if its value, bound to `n`,
            // is equal to 42.
            Some(n @ 42) => println!("The Answer: {}!", n),
            // Match any other number.
            Some(n) => println!("Not interesting... {}", n),
            // Match anything else (`None` variant).
            _ => (),
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    println!("\n--- 8.6. if let ---");
    {
        {
            // Make `optional` of type `Option<i32>`
            let optional = Some(7);

            match optional {
                Some(i) => {
                    println!("This is a really long string and `{:?}`", i);
                    // ^ Needed 2 indentations just so we could destructure
                    // `i` from the option.
                }
                _ => {} // ^ Required because `match` is exhaustive. Doesn't it seem
                        // like wasted space?
            };
        }
        {
            // All have type `Option<i32>`
            let number = Some(7);
            let letter: Option<i32> = None;
            let emoticon: Option<i32> = None;

            // The `if let` construct reads: "if `let` destructures `number` into
            // `Some(i)`, evaluate the block (`{}`).
            if let Some(i) = number {
                println!("Matched {:?}!", i);
            }

            // If you need to specify a failure, use an else:
            if let Some(i) = letter {
                println!("Matched {:?}!", i);
            } else {
                // Destructure failed. Change to the failure case.
                println!("Didn't match a number. Let's go with a letter!");
            }

            // Provide an altered failing condition.
            let i_like_letters = false;

            if let Some(i) = emoticon {
                println!("Matched {:?}!", i);
                // Destructure failed. Evaluate an `else if` condition to see if the
                // alternate failure branch should be taken:
            } else if i_like_letters {
                println!("Didn't match a number. Let's go with a letter!");
            } else {
                // The condition evaluated false. This branch is the default:
                println!("I don't like letters. Let's go with an emoticon :)!");
            }
        }
        {
            // Our example enum
            enum Foo {
                Bar,
                Baz,
                Qux(u32),
            }

            // Create example variables
            let a = Foo::Bar;
            let b = Foo::Baz;
            let c = Foo::Qux(100);

            // Variable a matches Foo::Bar
            if let Foo::Bar = a {
                println!("a is foobar");
            }

            // Variable b does not match Foo::Bar
            // So this will print nothing
            if let Foo::Bar = b {
                println!("b is foobar");
            }

            // Variable c matches Foo::Qux which has a value
            // Similar to Some() in the previous example
            if let Foo::Qux(value) = c {
                println!("c is {}", value);
            }

            // Binding also works with `if let`
            if let Foo::Qux(value @ 100) = c {
                println!("c is one hundred");
                println!("c is {}", value);
            }
        }
        {
            // This enum purposely neither implements nor derives PartialEq.
            // That is why comparing Foo::Bar == a fails below.
            enum Foo {
                Bar,
            }

            let a = Foo::Bar;

            // Variable a matches Foo::Bar
            if let Foo::Bar = a {
                // DONE: ^-- this causes a compile-time error. Use `if let` instead.
                println!("a is foobar");
            }
        }
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html
    println!("\n--- 8.7. while let ---");
    {
        {
            // Make `optional` of type `Option<i32>`
            let mut optional = Some(0);

            // Repeatedly try this test.
            loop {
                match optional {
                    // If `optional` destructures, evaluate the block.
                    Some(i) => {
                        if i > 9 {
                            println!("Greater than 9, quit!");
                            optional = None;
                        } else {
                            println!("`i` is `{:?}`. Try again.", i);
                            optional = Some(i + 1);
                        }
                        // ^ Requires 3 indentations!
                    }
                    // Quit the loop when the destructure fails:
                    _ => {
                        break;
                    } // ^ Why should this be required? There must be a better way!
                }
            }
        }
        {
            // Make `optional` of type `Option<i32>`
            let mut optional = Some(0);

            // This reads: "while `let` destructures `optional` into
            // `Some(i)`, evaluate the block (`{}`). Else `break`.
            while let Some(i) = optional {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Less rightward drift and doesn't require
                // explicitly handling the failing case.
            }
            // ^ `if let` had additional optional `else`/`else if`
            // clauses. `while let` does not have these.
        }
    }
}
