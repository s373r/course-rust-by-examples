// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 10. Modules
//    10.1. Visibility
//    10.2. Struct visibility
//    10.3. The use declaration
//    10.4. super and self

// --- 10.1. Visibility ---
// A module named `my_mod`
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

// --- 10.3. The use declaration ---
mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

// --- 10.4. super and self ---
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();

        // We can also use `self` to access another module inside `my`:
        self::cool::function();

        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();

        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    // https://doc.rust-lang.org/rust-by-example/mod.html
    println!("\n--- 10. Modules ---");
    {}

    // https://doc.rust-lang.org/rust-by-example/mod/visibility.html
    println!("\n--- 10.1. Visibility ---");
    {
        fn function() {
            println!("called `function()`");
        }

        // Modules allow disambiguation between items that have the same name.
        function();
        my_mod::function();

        // Public items, including those inside nested modules, can be
        // accessed from outside the parent module.
        my_mod::indirect_access();
        my_mod::nested::function();
        my_mod::call_public_function_in_my_mod();

        // pub(crate) items can be called from anywhere in the same crate
        my_mod::public_function_in_crate();

        // pub(in path) items can only be called from within the module specified
        // Error! function `public_function_in_my_mod` is private
        // my_mod::nested::public_function_in_my_mod();
        // DONE: ^ Try uncommenting this line

        // Private items of a module cannot be directly accessed, even if
        // nested in a public module:

        // Error! `private_function` is private
        // my_mod::private_function();
        // DONE: ^ Try uncommenting this line

        // Error! `private_function` is private
        //my_mod::nested::private_function();
        // DONE:  ^ Try uncommenting this line

        // Error! `private_nested` is a private module
        //my_mod::private_nested::function();
        // DONE:  ^ Try uncommenting this line

        // Error! `private_nested` is a private module
        // my_mod::private_nested::restricted_function();
        // DONE:  ^ Try uncommenting this line
    }

    // https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
    println!("\n--- 10.2. Struct visibility ---");
    {
        mod my {
            // A public struct with a public field of generic type `T`
            pub struct OpenBox<T> {
                pub contents: T,
            }

            // A public struct with a private field of generic type `T`
            #[allow(dead_code)]
            pub struct ClosedBox<T> {
                contents: T,
            }

            impl<T> ClosedBox<T> {
                // A public constructor method
                pub fn new(contents: T) -> ClosedBox<T> {
                    ClosedBox { contents }
                }
            }
        }

        // Public structs with public fields can be constructed as usual
        let open_box = my::OpenBox {
            contents: "public information",
        };

        // and their fields can be normally accessed.
        println!("The open box contains: {}", open_box.contents);

        // Public structs with private fields cannot be constructed using field names.
        // Error! `ClosedBox` has private fields
        // let closed_box = my::ClosedBox { contents: "classified information" };
        // DONE: ^ Try uncommenting this line

        // However, structs with private fields can be created using
        // public constructors
        let _closed_box = my::ClosedBox::new("classified information");

        // and the private fields of a public struct cannot be accessed.
        // Error! The `contents` field is private
        // println!("The closed box contains: {}", _closed_box.contents);
        // DONE: ^ Try uncommenting this line
    }

    // https://doc.rust-lang.org/rust-by-example/mod/use.html
    println!("\n--- 10.3. The use declaration ---");
    {
        // Bind the `deeply::nested::function` path to `other_function`.
        use deeply::nested::function as other_function;

        fn function() {
            println!("called `function()`");
        }

        // Easier access to `deeply::nested::function`
        other_function();

        println!("Entering block");
        {
            // This is equivalent to `use deeply::nested::function as function`.
            // This `function()` will shadow the outer one.
            use crate::deeply::nested::function;

            // `use` bindings have a local scope. In this case, the
            // shadowing of `function()` is only in this block.
            function();

            println!("Leaving block");
        }

        function();
    }

    // https://doc.rust-lang.org/rust-by-example/mod/super.html
    println!("\n--- 10.4. super and self ---");
    {
        my::indirect_call();
    }
}
