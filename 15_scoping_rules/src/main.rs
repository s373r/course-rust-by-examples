// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 15. Scoping rules
//    15.1. RAII
//    15.2. Ownership and moves
//          15.2.1. Mutability
//          15.2.2. Partial moves
//    15.3. Borrowing
//          15.3.1. Mutability
//          15.3.2. Aliasing
//          15.3.3. The ref pattern
//    15.4. Lifetimes
//          15.4.1. Explicit annotation
//          15.4.2. Functions
//          15.4.3. Methods
//          15.4.4. Structs
//          15.4.5. Traits
//          15.4.6. Bounds
//          15.4.7. Coercion
//          15.4.8. Static
//          15.4.9. Elision

fn main() {
    // https://doc.rust-lang.org/stable/rust-by-example/scope.html
    println!("\n--- 15. Scoping rules ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html
    println!("\n--- 15.1. RAII ---");
    {
        {
            // raii.rs
            fn create_box() {
                // Allocate an integer on the heap
                let _box1 = Box::new(3i32);

                // `_box1` is destroyed here, and memory gets freed
            }

            // Allocate an integer on the heap
            let _box2 = Box::new(5i32);

            // A nested scope:
            {
                // Allocate an integer on the heap
                let _box3 = Box::new(4i32);

                // `_box3` is destroyed here, and memory gets freed
            }

            // Creating lots of boxes just for fun
            // There's no need to manually free memory!
            for _ in 0u32..1_000 {
                create_box();
            }

            // `_box2` is destroyed here, and memory gets freed
        }
        {
            struct ToDrop;

            impl Drop for ToDrop {
                fn drop(&mut self) {
                    println!("ToDrop is being dropped");
                }
            }

            let _x = ToDrop;
            println!("Made a ToDrop!");
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/move.html
    println!("\n--- 15.2. Ownership and moves ---");
    {
        // This function takes ownership of the heap allocated memory
        fn destroy_box(c: Box<i32>) {
            println!("Destroying a box that contains {}", c);

            // `c` is destroyed and the memory freed
        }

        // _Stack_ allocated integer
        let x = 5u32;

        // *Copy* `x` into `y` - no resources are moved
        let y = x;

        // Both values can be independently used
        println!("x is {}, and y is {}", x, y);

        // `a` is a pointer to a _heap_ allocated integer
        let a = Box::new(5i32);

        println!("a contains: {}", a);

        // *Move* `a` into `b`
        let b = a;
        // The pointer address of `a` is copied (not the data) into `b`.
        // Both are now pointers to the same heap allocated data, but
        // `b` now owns it.

        // Error! `a` can no longer access the data, because it no longer owns the
        // heap memory
        // println!("a contains: {}", a);
        // DONE: ^ Try uncommenting this line

        // This function takes ownership of the heap allocated memory from `b`
        destroy_box(b);

        // Since the heap memory has been freed at this point, this action would
        // result in dereferencing freed memory, but it's forbidden by the compiler
        // Error! Same reason as the previous Error
        // println!("b contains: {}", b);
        // DONE: ^ Try uncommenting this line
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/move/mut.html
    println!("\n--- 15.2.1. Mutability ---");
    {
        let immutable_box = Box::new(5u32);

        println!("immutable_box contains {}", immutable_box);

        // Mutability error
        //*immutable_box = 4;

        // *Move* the box, changing the ownership (and mutability)
        let mut mutable_box = immutable_box;

        println!("mutable_box contains {}", mutable_box);

        // Modify the contents of the box
        *mutable_box = 4;

        println!("mutable_box now contains {}", mutable_box);

        // NOTE: move the box *back*
        let immutable_box = mutable_box;

        println!("immutable_box contains {}", immutable_box);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/move/partial_move.html
    println!("\n--- 15.2.2. Partial moves ---");
    {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let person = Person {
            name: String::from("Alice"),
            age: 20,
        };

        // `name` is moved out of person, but `age` is referenced
        let Person { name, ref age } = person;

        println!("The person's age is {}", age);

        println!("The person's name is {}", name);

        // Error! borrow of partially moved value: `person` partial move occurs
        // println!("The person struct is {:?}", person);

        // `person` cannot be used but `person.age` can be used as it is not moved
        println!("The person's age from person struct is {}", person.age);

        // NOTE: try to access `person.name`: compile error
        // println!("The person's name from person struct is {}", person.name);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow.html
    println!("\n--- 15.3. Borrowing ---");
    {
        // This function takes ownership of a box and destroys it
        fn eat_box_i32(boxed_i32: Box<i32>) {
            println!("Destroying box that contains {}", boxed_i32);
        }

        // This function borrows an i32
        fn borrow_i32(borrowed_i32: &i32) {
            println!("This int is: {}", borrowed_i32);
        }

        // Create a boxed i32, and a stacked i32
        let boxed_i32 = Box::new(5_i32);
        let stacked_i32 = 6_i32;

        // Borrow the contents of the box. Ownership is not taken,
        // so the contents can be borrowed again.
        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            // Take a reference to the data contained inside the box
            let _ref_to_i32: &i32 = &boxed_i32;

            // Error!
            // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
            // eat_box_i32(boxed_i32);
            // DONE: ^ Comment out this line

            // Attempt to borrow `_ref_to_i32` after inner value is destroyed
            borrow_i32(_ref_to_i32);
            // `_ref_to_i32` goes out of scope and is no longer borrowed.
        }

        // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
        eat_box_i32(boxed_i32);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/mut.html
    println!("\n--- 15.3.1. Mutability ---");
    {
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        struct Book {
            // `&'static str` is a reference to a string allocated in read only memory
            author: &'static str,
            title: &'static str,
            year: u32,
        }

        // This function takes a reference to a book
        fn borrow_book(book: &Book) {
            println!(
                "I immutably borrowed {} - {} edition",
                book.title, book.year
            );
        }

        // This function takes a reference to a mutable book and changes `year` to 2014
        fn new_edition(book: &mut Book) {
            book.year = 2014;
            println!("I mutably borrowed {} - {} edition", book.title, book.year);
        }

        // Create an immutable Book named `immutabook`
        let immutabook = Book {
            // string literals have type `&'static str`
            author: "Douglas Hofstadter",
            title: "Gödel, Escher, Bach",
            year: 1979,
        };

        // Create a mutable copy of `immutabook` and call it `mutabook`
        let mut mutabook = immutabook;

        // Immutably borrow an immutable object
        borrow_book(&immutabook);

        // Immutably borrow a mutable object
        borrow_book(&mutabook);

        // Borrow a mutable object as mutable
        new_edition(&mut mutabook);

        // Error! Cannot borrow an immutable object as mutable
        // new_edition(&mut immutabook);
        // DONE: ^ Comment out this line
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/alias.html
    println!("\n--- 15.3.2. Aliasing ---");
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let mut point = Point { x: 0, y: 0, z: 0 };

        let borrowed_point = &point;
        let another_borrow = &point;

        // Data can be accessed via the references and the original owner
        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );

        // Error! Can't borrow `point` as mutable because it's currently
        // borrowed as immutable.
        // let mutable_borrow = &mut point;
        // DONE: ^ Try uncommenting this line

        // The borrowed values are used again here
        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );

        // The immutable references are no longer used for the rest of the code so
        // it is possible to reborrow with a mutable reference.
        let mutable_borrow = &mut point;

        // Change data via mutable reference
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // Error! Can't borrow `point` as immutable because it's currently
        // borrowed as mutable.
        // let y = &point.y;
        // DONE: ^ Try uncommenting this line

        // Error! Can't print because `println!` takes an immutable reference.
        // println!("Point Z coordinate is {}", point.z);
        // DONE: ^ Try uncommenting this line

        // Ok! Mutable references can be passed as immutable to `println!`
        println!(
            "Point has coordinates: ({}, {}, {})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );

        // The mutable reference is no longer used for the rest of the code so it
        // is possible to reborrow
        let new_borrowed_point = &point;
        println!(
            "Point now has coordinates: ({}, {}, {})",
            new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
        );
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html
    // println!("\n--- 15.3.3. The ref pattern ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html
    // println!("\n--- 15.4. Lifetimes ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/explicit.html
    // println!("\n--- 15.4.1. Explicit annotation ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/fn.html
    // println!("\n--- 15.4.2. Functions ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/methods.html
    // println!("\n--- 15.4.3. Methods ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/struct.html
    // println!("\n--- 15.4.4. Structs ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/trait.html
    // println!("\n--- 15.4.5. Traits ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_bounds.html
    // println!("\n--- 15.4.6. Bounds ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_coercion.html
    // println!("\n--- 15.4.7. Coercion ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html
    // println!("\n--- 15.4.8. Static ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/elision.html
    // println!("\n--- 15.4.9. Elision ---");
    {}
}
