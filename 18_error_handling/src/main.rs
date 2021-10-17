// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 18. Error handling
//    18.1. panic
//    18.2. Option & unwrap
//          18.2.1. Unpacking options with ?
//          18.2.2. Combinators: map
//          18.2.3. Combinators: and_then
//    18.3. Result
//          18.3.1. map for Result
//          18.3.2. aliases for Result
//          18.3.3. Early returns
//          18.3.4. Introducing ?
//    18.4. Multiple error types
//          18.4.1. Pulling Results out of Options
//          18.4.2. Defining an error type
//          18.4.3. Boxing errors
//          18.4.4. Other uses of ?
//          18.4.5. Wrapping errors
//    18.5. Iterating over Results

fn main() {
    // https://doc.rust-lang.org/stable/rust-by-example/error.html
    println!("\n--- 18. Error handling ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/panic.html
    println!("\n--- 18.1. panic ---");
    {
        fn drink(beverage: &str) {
            // You shouldn't drink too much sugary beverages.
            if beverage == "lemonade" {
                panic!("AAAaaaaa!!!!");
            }

            println!("Some refreshing {} is all I need.", beverage);
        }

        drink("water");
        // drink("lemonade");
        // NOTE: commented to continue execution
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap.html
    println!("\n--- 18.2. Option & unwrap ---");
    {
        // The adult has seen it all, and can handle any drink well.
        // All drinks are handled explicitly using `match`.
        fn give_adult(drink: Option<&str>) {
            // Specify a course of action for each case.
            match drink {
                Some("lemonade") => println!("Yuck! Too sugary."),
                Some(inner) => println!("{}? How nice.", inner),
                None => println!("No drink? Oh well."),
            }
        }

        // Others will `panic` before drinking sugary drinks.
        // All drinks are handled implicitly using `unwrap`.
        fn drink(drink: Option<&str>) {
            // `unwrap` returns a `panic` when it receives a `None`.
            let inside = drink.unwrap();
            if inside == "lemonade" {
                panic!("AAAaaaaa!!!!");
            }

            println!("I love {}s!!!!!", inside);
        }

        let water = Some("water");
        let lemonade = Some("lemonade");
        let void = None;

        give_adult(water);
        give_adult(lemonade);
        give_adult(void);

        let coffee = Some("coffee");

        drink(coffee);
        // drink(None);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/question_mark.html
    println!("\n--- 18.2.1. Unpacking options with ? ---");
    {
        struct Person {
            job: Option<Job>,
        }

        #[derive(Clone, Copy)]
        struct Job {
            phone_number: Option<PhoneNumber>,
        }

        #[derive(Clone, Copy)]
        struct PhoneNumber {
            area_code: Option<u8>,
            #[allow(dead_code)]
            number: u32,
        }

        impl Person {
            // Gets the area code of the phone number of the person's job, if it exists.
            fn work_phone_area_code(&self) -> Option<u8> {
                // This would need many nested `match` statements without the `?` operator.
                // It would take a lot more code - try writing it yourself and see which
                // is easier.
                self.job?.phone_number?.area_code
            }
        }

        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    number: 439222222,
                }),
            }),
        };

        assert_eq!(p.work_phone_area_code(), Some(61));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/map.html
    println!("\n--- 18.2.2. Combinators: map ---");
    {
        #[derive(Debug)]
        enum Food {
            Apple,
            Carrot,
            #[allow(dead_code)]
            Potato,
        }

        #[derive(Debug)]
        struct Peeled(Food);
        #[derive(Debug)]
        struct Chopped(Food);
        #[derive(Debug)]
        struct Cooked(Food);

        // Peeling food. If there isn't any, then return `None`.
        // Otherwise, return the peeled food.
        fn peel(food: Option<Food>) -> Option<Peeled> {
            match food {
                Some(food) => Some(Peeled(food)),
                None => None,
            }
        }

        // Chopping food. If there isn't any, then return `None`.
        // Otherwise, return the chopped food.
        fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
            match peeled {
                Some(Peeled(food)) => Some(Chopped(food)),
                None => None,
            }
        }

        // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
        fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
            chopped.map(|Chopped(food)| Cooked(food))
        }

        // A function to peel, chop, and cook food all in sequence.
        // We chain multiple uses of `map()` to simplify the code.
        fn process(food: Option<Food>) -> Option<Cooked> {
            food.map(|f| Peeled(f))
                .map(|Peeled(f)| Chopped(f))
                .map(|Chopped(f)| Cooked(f))
        }

        // Check whether there's food or not before trying to eat it!
        fn eat(food: Option<Cooked>) {
            match food {
                Some(food) => println!("Mmm. I love {:?}", food),
                None => println!("Oh no! It wasn't edible."),
            }
        }

        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));
        // Let's try the simpler looking `process()` now.
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/and_then.html
    println!("\n--- 18.2.3. Combinators: and_then ---");
    {
        #[derive(Debug)]
        enum Food {
            CordonBleu,
            Steak,
            Sushi,
        }
        #[derive(Debug)]
        enum Day {
            Monday,
            Tuesday,
            Wednesday,
        }

        // We don't have the ingredients to make Sushi.
        fn have_ingredients(food: Food) -> Option<Food> {
            match food {
                Food::Sushi => None,
                _ => Some(food),
            }
        }

        // We have the recipe for everything except Cordon Bleu.
        fn have_recipe(food: Food) -> Option<Food> {
            match food {
                Food::CordonBleu => None,
                _ => Some(food),
            }
        }

        #[allow(dead_code)]
        // To make a dish, we need both the recipe and the ingredients.
        // We can represent the logic with a chain of `match`es:
        fn cookable_v1(food: Food) -> Option<Food> {
            match have_recipe(food) {
                None => None,
                Some(food) => match have_ingredients(food) {
                    None => None,
                    Some(food) => Some(food),
                },
            }
        }

        // This can conveniently be rewritten more compactly with `and_then()`:
        fn cookable_v2(food: Food) -> Option<Food> {
            have_recipe(food).and_then(have_ingredients)
        }

        fn eat(food: Food, day: Day) {
            match cookable_v2(food) {
                Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
                None => println!("Oh no. We don't get to eat on {:?}?", day),
            }
        }

        let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

        eat(cordon_bleu, Day::Monday);
        eat(steak, Day::Tuesday);
        eat(sushi, Day::Wednesday);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result.html
    // println!("\n--- 18.3. Result ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/result_map.html
    // println!("\n--- 18.3.1. map for Result ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/result_alias.html
    // println!("\n--- 18.3.2. aliases for Result ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/early_returns.html
    // println!("\n--- 18.3.3. Early returns ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/enter_question_mark.html
    // println!("\n--- 18.3.4. Introducing ? ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types.html
    // println!("\n--- 18.4. Multiple error types ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/option_result.html
    // println!("\n--- 18.4.1. Pulling Results out of Options ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html
    // println!("\n--- 18.4.2. Defining an error type ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html
    // println!("\n--- 18.4.3. Boxing errors ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html
    // println!("\n--- 18.4.4. Other uses of ? ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html
    // println!("\n--- 18.4.5. Wrapping errors ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    // println!("\n--- 18.5. Iterating over Results ---");
    {}
}
