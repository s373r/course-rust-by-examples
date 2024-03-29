// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 3. Custom types
//    3.1. Structures
//    3.2. Enums
//         3.2.1. use
//         3.2.2. C-like
//         3.2.3. Testcase: linked-list
//    3.3. constants

fn main() {
    // https://doc.rust-lang.org/rust-by-example/custom_types.html
    println!();
    println!("--- 3. Custom types ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
    println!();
    println!("--- 3.1. Structures ---");
    {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        // A unit struct
        struct Unit;

        // A tuple struct
        struct Pair(i32, f32);

        // A struct with two fields
        #[derive(Debug)]
        struct Point {
            x: f32,
            y: f32,
        }

        // Structs can be reused as fields of another struct
        #[derive(Debug)]
        struct Rectangle {
            // A rectangle can be specified by where the top left and bottom right
            // corners are in space.
            top_left: Point,
            bottom_right: Point,
        }

        // Create struct with field init shorthand
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };

        // Print debug struct
        println!("{:?}", peter);

        // Instantiate a `Point`
        let point: Point = Point { x: 10.3, y: 0.4 };

        // Access the fields of the point
        println!("point coordinates: ({}, {})", point.x, point.y);

        // Make a new point by using struct update syntax to use the fields of our
        // other one
        let bottom_right = Point { x: 5.2, ..point };

        // `bottom_right.y` will be the same as `point.y` because we used that field
        // from `point`
        println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

        // Destructure the point using a `let` binding
        let Point {
            x: left_edge,
            y: top_edge,
        } = point;

        let _rectangle = Rectangle {
            // struct instantiation is an expression too
            top_left: Point {
                x: left_edge,
                y: top_edge,
            },
            bottom_right,
        };

        // Instantiate a unit struct
        let _unit = Unit;

        // Instantiate a tuple struct
        let pair = Pair(1, 0.1);

        // Access the fields of a tuple struct
        println!("pair contains {:?} and {:?}", pair.0, pair.1);

        // Destructure a tuple struct
        let Pair(integer, decimal) = pair;

        println!("pair contains {:?} and {:?}", integer, decimal);

        println!();
        println!("--- Activities ---");
        //        [x] Add a function rect_area which calculates the area of a Rectangle
        //            (try using nested destructuring).
        //        [x] Add a function square which takes a Point and a f32 as arguments,
        //            and returns a Rectangle with its lower left corner on the point,
        //            and a width and height corresponding to the f32.

        fn rect_area(rect: Rectangle) -> f32 {
            let Rectangle {
                top_left: Point { x: x0, y: y0 },
                bottom_right: Point { x: x1, y: y1 },
            } = rect;

            (x1 - x0) * (y1 - y0)
        }

        let rectangle = Rectangle {
            top_left: Point { x: 1f32, y: 2f32 },
            bottom_right: Point { x: 3f32, y: 5f32 },
        };

        println!("{}", rect_area(rectangle));

        fn square(point: Point, side: f32) -> Rectangle {
            let Point { x, y } = point;

            Rectangle {
                top_left: point,
                bottom_right: Point {
                    x: x + side,
                    y: y + side,
                },
            }
        }

        let point = Point { x: 1f32, y: 2f32 };
        println!("{:?}", square(point, 3f32));
    }

    // https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
    println!();
    println!("--- 3.2. Enums ---");
    {
        // Create an `enum` to classify a web event. Note how both
        // names and type information together specify the variant:
        // `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
        // Each is different and independent.
        enum WebEvent {
            // An `enum` may either be `unit-like`,
            PageLoad,
            PageUnload,
            // like tuple structs,
            KeyPress(char),
            Paste(String),
            // or c-like structures.
            Click { x: i64, y: i64 },
        }

        // A function which takes a `WebEvent` enum as an argument and
        // returns nothing.
        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                // Destructure `c` from inside the `enum`.
                WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
                WebEvent::Paste(s) => println!("pasted \"{}\".", s),
                // Destructure `Click` into `x` and `y`.
                WebEvent::Click { x, y } => {
                    println!("clicked at x={}, y={}.", x, y);
                }
            }
        }

        let pressed = WebEvent::KeyPress('x');
        // `to_owned()` creates an owned `String` from a string slice.
        let pasted = WebEvent::Paste("my text".to_owned());
        let click = WebEvent::Click { x: 20, y: 80 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);

        enum VeryVerboseEnumOfThingsToDoWithNumbers {
            Add,
            #[allow(dead_code)]
            Subtract,
        }

        // Creates a type alias
        type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

        // We can refer to each variant via its alias, not its long and inconvenient
        // name.
        let x = Operations::Add;

        impl VeryVerboseEnumOfThingsToDoWithNumbers {
            fn run(&self, x: i32, y: i32) -> i32 {
                match self {
                    Self::Add => x + y,
                    Self::Subtract => x - y,
                }
            }
        }

        println!("{}", x.run(1, 2));
    }

    // https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
    println!();
    println!("--- 3.2.1. use ---");
    {
        #[allow(dead_code)]
        enum Status {
            Rich,
            Poor,
        }

        #[allow(dead_code)]
        enum Work {
            Civilian,
            Soldier,
        }

        // Explicitly `use` each name so they are available without
        // manual scoping.
        use Status::{Poor, Rich};
        // Automatically `use` each name inside `Work`.
        use Work::*;

        // Equivalent to `Status::Poor`.
        let status = Poor;
        // Equivalent to `Work::Civilian`.
        let work = Civilian;

        match status {
            // Note the lack of scoping because of the explicit `use` above.
            Rich => println!("The rich have lots of money!"),
            Poor => println!("The poor have no money..."),
        }

        match work {
            // Note again the lack of scoping.
            Civilian => println!("Civilians work!"),
            Soldier => println!("Soldiers fight!"),
        }
    }

    // https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html
    println!();
    println!("--- 3.2.2. C-like ---");
    {
        // enum with implicit discriminator (starts at 0)
        #[allow(dead_code)]
        enum Number {
            Zero,
            One,
            Two,
        }

        // enum with explicit discriminator
        #[allow(dead_code)]
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }

        // `enums` can be cast as integers.
        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);

        println!("roses are #{:06x}", Color::Red as i32);
        println!("violets are #{:06x}", Color::Blue as i32);
    }

    // https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
    println!();
    println!("--- 3.2.3. Testcase: linked-list ---");
    {
        use List::*;

        enum List {
            // Cons: Tuple struct that wraps an element and a pointer to the next node
            Cons(u32, Box<List>),
            // Nil: A node that signifies the end of the linked list
            Nil,
        }

        // Methods can be attached to an enum
        impl List {
            // Create an empty list
            fn new() -> List {
                // `Nil` has type `List`
                Nil
            }

            // Consume a list, and return the same list with a new element at its front
            fn prepend(self, elem: u32) -> List {
                // `Cons` also has type List
                Cons(elem, Box::new(self))
            }

            // Return the length of the list
            fn len(&self) -> u32 {
                // `self` has to be matched, because the behavior of this method
                // depends on the variant of `self`
                // `self` has type `&List`, and `*self` has type `List`, matching on a
                // concrete type `T` is preferred over a match on a reference `&T`
                // after Rust 2018 you can use self here and tail (with no ref) below as well,
                // rust will infer &s and ref tail.
                // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
                // match *self {
                //     // Can't take ownership of the tail, because `self` is borrowed;
                //     // instead take a reference to the tail
                //     Cons(_, ref tail) => 1 + tail.len(),
                //     // Base Case: An empty list has zero length
                //     Nil => 0,
                // }

                // NOTE: Rust 2018 variant (*self -> self, ref tail -> tail):
                //       Algorithm complexity: O(n)
                match self {
                    Cons(_, tail) => 1 + tail.len(),
                    Nil => 0,
                }
            }

            // Return representation of the list as a (heap allocated) string
            fn stringify(&self) -> String {
                // match *self {
                //     Cons(head, ref tail) => {
                //         // `format!` is similar to `print!`, but returns a heap
                //         // allocated string instead of printing to the console
                //         format!("{}, {}", head, tail.stringify())
                //     }
                //     Nil => {
                //         format!("Nil")
                //     }
                // }

                // NOTE: Rust 2018 variant (*self -> self, ref tail -> tail):
                match self {
                    Cons(head, tail) => {
                        // `format!` is similar to `print!`, but returns a heap
                        // allocated string instead of printing to the console

                        // NOTE: recursive call
                        format!("{}, {}", head, tail.stringify())
                    }
                    Nil => {
                        format!("Nil")
                    }
                }
            }
        }

        // Create an empty linked list
        let mut list = List::new();

        // Prepend some elements
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        // Show the final state of the list
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }

    // https://doc.rust-lang.org/rust-by-example/custom_types/constants.html
    println!();
    println!("--- 3.3. constants ---");
    {
        // Globals are declared outside all other scopes.
        static LANGUAGE: &str = "Rust";
        const THRESHOLD: i32 = 10;

        fn is_big(n: i32) -> bool {
            // Access constant in some function
            n > THRESHOLD
        }

        let n = 16;

        // Access constant in the main thread
        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THRESHOLD);
        println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

        // Error! Cannot modify a `const`.
        // THRESHOLD = 5;
        // DONE: ^ Comment out this line
    }
}
