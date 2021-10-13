// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 3. Custom types
//    3.1. Structures

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
}
