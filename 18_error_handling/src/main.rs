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
    println!("\n--- 18.3. Result ---");
    {
        {
            fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
                // Let's try using `unwrap()` to get the number out. Will it bite us?
                let first_number = first_number_str.parse::<i32>().unwrap();
                let second_number = second_number_str.parse::<i32>().unwrap();
                first_number * second_number
            }

            let twenty = multiply("10", "2");
            println!("double is {}", twenty);

            // NOTE: commented to continue execution
            // let tt = multiply("t", "2");
            // println!("double is {}", tt);
        }
        {
            use std::num::ParseIntError;

            fn main() -> Result<(), ParseIntError> {
                let number_str = "10";
                let number = match number_str.parse::<i32>() {
                    Ok(number) => number,
                    Err(e) => return Err(e),
                };
                println!("{}", number);
                Ok(())
            }

            // NOTE: commented to continue execution
            main().unwrap();
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/result_map.html
    println!("\n--- 18.3.1. map for Result ---");
    {
        {
            use std::num::ParseIntError;

            // With the return type rewritten, we use pattern matching without `unwrap()`.
            fn multiply(
                first_number_str: &str,
                second_number_str: &str,
            ) -> Result<i32, ParseIntError> {
                match first_number_str.parse::<i32>() {
                    Ok(first_number) => match second_number_str.parse::<i32>() {
                        Ok(second_number) => Ok(first_number * second_number),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }

            fn print(result: Result<i32, ParseIntError>) {
                match result {
                    Ok(n) => println!("n is {}", n),
                    Err(e) => println!("Error: {}", e),
                }
            }

            // This still presents a reasonable answer.
            let twenty = multiply("10", "2");
            print(twenty);

            // The following now provides a much more helpful error message.
            let tt = multiply("t", "2");
            print(tt);
        }
        {
            use std::num::ParseIntError;

            // As with `Option`, we can use combinators such as `map()`.
            // This function is otherwise identical to the one above and reads:
            // Modify n if the value is valid, otherwise pass on the error.
            fn multiply(
                first_number_str: &str,
                second_number_str: &str,
            ) -> Result<i32, ParseIntError> {
                first_number_str.parse::<i32>().and_then(|first_number| {
                    second_number_str
                        .parse::<i32>()
                        .map(|second_number| first_number * second_number)
                })
            }

            fn print(result: Result<i32, ParseIntError>) {
                match result {
                    Ok(n) => println!("n is {}", n),
                    Err(e) => println!("Error: {}", e),
                }
            }

            // This still presents a reasonable answer.
            let twenty = multiply("10", "2");
            print(twenty);

            // The following now provides a much more helpful error message.
            let tt = multiply("t", "2");
            print(tt);
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/result_alias.html
    println!("\n--- 18.3.2. aliases for Result ---");
    {
        use std::num::ParseIntError;

        // Define a generic alias for a `Result` with the error type `ParseIntError`.
        type AliasedResult<T> = Result<T, ParseIntError>;

        // Use the above alias to refer to our specific `Result` type.
        fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str
                    .parse::<i32>()
                    .map(|second_number| first_number * second_number)
            })
        }

        // Here, the alias again allows us to save some space.
        fn print(result: AliasedResult<i32>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/early_returns.html
    println!("\n--- 18.3.3. Early returns ---");
    {
        use std::num::ParseIntError;

        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            let first_number = match first_number_str.parse::<i32>() {
                Ok(first_number) => first_number,
                Err(e) => return Err(e),
            };

            let second_number = match second_number_str.parse::<i32>() {
                Ok(second_number) => second_number,
                Err(e) => return Err(e),
            };

            Ok(first_number * second_number)
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/enter_question_mark.html
    println!("\n--- 18.3.4. Introducing ? ---");
    {
        {
            use std::num::ParseIntError;

            fn multiply(
                first_number_str: &str,
                second_number_str: &str,
            ) -> Result<i32, ParseIntError> {
                let first_number = first_number_str.parse::<i32>()?;
                let second_number = second_number_str.parse::<i32>()?;

                Ok(first_number * second_number)
            }

            fn print(result: Result<i32, ParseIntError>) {
                match result {
                    Ok(n) => println!("n is {}", n),
                    Err(e) => println!("Error: {}", e),
                }
            }

            print(multiply("10", "2"));
            print(multiply("t", "2"));
        }
        {
            // To compile and run this example without errors, while using Cargo, change the value
            // of the `edition` field, in the `[package]` section of the `Cargo.toml` file, to "2015".

            use std::num::ParseIntError;

            fn multiply(
                first_number_str: &str,
                second_number_str: &str,
            ) -> Result<i32, ParseIntError> {
                // let first_number = try!(first_number_str.parse::<i32>());
                let first_number = first_number_str.parse::<i32>()?;
                // let second_number = try!(second_number_str.parse::<i32>());
                let second_number = second_number_str.parse::<i32>()?;

                Ok(first_number * second_number)
            }

            fn print(result: Result<i32, ParseIntError>) {
                match result {
                    Ok(n) => println!("n is {}", n),
                    Err(e) => println!("Error: {}", e),
                }
            }

            print(multiply("10", "2"));
            print(multiply("t", "2"));
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types.html
    println!("\n--- 18.4. Multiple error types ---");
    {
        fn double_first(vec: Vec<&str>) -> i32 {
            let first = vec.first().unwrap(); // Generate error 1
            2 * first.parse::<i32>().unwrap() // Generate error 2
        }

        let numbers = vec!["42", "93", "18"];
        let _empty: Vec<&str> = vec![];
        let _strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {}", double_first(numbers));

        // NOTE: commented to continue execution
        // println!("The first doubled is {}", double_first(_empty));
        // Error 1: the input vector is empty

        // NOTE: commented to continue execution
        // println!("The first doubled is {}", double_first(_strings));
        // Error 2: the element doesn't parse to a number
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/option_result.html
    println!("\n--- 18.4.1. Pulling Results out of Options ---");
    {
        {
            use std::num::ParseIntError;

            fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
                vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
            }

            let numbers = vec!["42", "93", "18"];
            let empty = vec![];
            let strings = vec!["tofu", "93", "18"];

            println!("The first doubled is {:?}", double_first(numbers));

            println!("The first doubled is {:?}", double_first(empty));
            // Error 1: the input vector is empty

            println!("The first doubled is {:?}", double_first(strings));
            // Error 2: the element doesn't parse to a number
        }
        {
            use std::num::ParseIntError;

            fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
                let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

                opt.map_or(Ok(None), |r| r.map(Some))
            }

            let numbers = vec!["42", "93", "18"];
            let empty = vec![];
            let strings = vec!["tofu", "93", "18"];

            println!("The first doubled is {:?}", double_first(numbers));
            println!("The first doubled is {:?}", double_first(empty));
            println!("The first doubled is {:?}", double_first(strings));
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html
    println!("\n--- 18.4.2. Defining an error type ---");
    {
        use std::fmt;

        type Result<T> = std::result::Result<T, DoubleError>;

        // Define our error types. These may be customized for our error handling cases.
        // Now we will be able to write our own errors, defer to an underlying error
        // implementation, or do something in between.
        #[derive(Debug, Clone)]
        struct DoubleError;

        // Generation of an error is completely separate from how it is displayed.
        // There's no need to be concerned about cluttering complex logic with the display style.
        //
        // Note that we don't store any extra info about the errors. This means we can't state
        // which string failed to parse without modifying our types to carry that information.
        impl fmt::Display for DoubleError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            vec.first()
                // Change the error to our new type.
                .ok_or(DoubleError)
                .and_then(|s| {
                    s.parse::<i32>()
                        // Update to the new error type here also.
                        .map_err(|_| DoubleError)
                        .map(|i| 2 * i)
                })
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html
    println!("\n--- 18.4.3. Boxing errors ---");
    {
        use std::error;
        use std::fmt;

        // DONE: Change the alias to `Box<error::Error>`.
        //       NOTE: with Rust 2018 that is deprecated but compilable
        type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

        #[derive(Debug, Clone)]
        struct EmptyVec;

        impl fmt::Display for EmptyVec {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        impl error::Error for EmptyVec {}

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            vec.first()
                .ok_or_else(|| EmptyVec.into()) // Converts to Box
                .and_then(|s| {
                    s.parse::<i32>()
                        .map_err(|e| e.into()) // Converts to Box
                        .map(|i| 2 * i)
                })
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html
    println!("\n--- 18.4.4. Other uses of ? ---");
    {
        use std::error;
        use std::fmt;

        // DONE: Change the alias to `Box<dyn error::Error>`.
        //       NOTE: with Rust 2018 that is deprecated but compilable
        type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

        #[derive(Debug)]
        struct EmptyVec;

        impl fmt::Display for EmptyVec {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        impl error::Error for EmptyVec {}

        // The same structure as before but rather than chain all `Results`
        // and `Options` along, we `?` to get the inner value out immediately.
        fn double_first(vec: Vec<&str>) -> Result<i32> {
            let first = vec.first().ok_or(EmptyVec)?;
            let parsed = first.parse::<i32>()?;
            Ok(2 * parsed)
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html
    println!("\n--- 18.4.5. Wrapping errors ---");
    {
        use std::error;
        use std::error::Error as _;
        use std::fmt;
        use std::num::ParseIntError;

        type Result<T> = std::result::Result<T, DoubleError>;

        #[derive(Debug)]
        enum DoubleError {
            EmptyVec,
            // We will defer to the parse error implementation for their error.
            // Supplying extra info requires adding more data to the type.
            Parse(ParseIntError),
        }

        impl fmt::Display for DoubleError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    DoubleError::EmptyVec => {
                        write!(f, "please use a vector with at least one element")
                    }
                    // The wrapped error contains additional information and is available
                    // via the source() method.
                    DoubleError::Parse(..) => {
                        write!(f, "the provided string could not be parsed as int")
                    }
                }
            }
        }

        impl error::Error for DoubleError {
            fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                match *self {
                    DoubleError::EmptyVec => None,
                    // The cause is the underlying implementation error type. Is implicitly
                    // cast to the trait object `&error::Error`. This works because the
                    // underlying type already implements the `Error` trait.
                    DoubleError::Parse(ref e) => Some(e),
                }
            }
        }

        // Implement the conversion from `ParseIntError` to `DoubleError`.
        // This will be automatically called by `?` if a `ParseIntError`
        // needs to be converted into a `DoubleError`.
        impl From<ParseIntError> for DoubleError {
            fn from(err: ParseIntError) -> DoubleError {
                DoubleError::Parse(err)
            }
        }

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            let first = vec.first().ok_or(DoubleError::EmptyVec)?;
            // Here we implicitly use the `ParseIntError` implementation of `From` (which
            // we defined above) in order to create a `DoubleError`.
            let parsed = first.parse::<i32>()?;

            Ok(2 * parsed)
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("The first doubled is {}", n),
                Err(e) => {
                    println!("Error: {}", e);
                    if let Some(source) = e.source() {
                        println!("  Caused by: {}", source);
                    }
                }
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    println!("\n--- 18.5. Iterating over Results ---");
    {
        {
            let strings = vec!["tofu", "93", "18"];
            let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
            println!("Results: {:?}", numbers);
        }
        {
            let strings = vec!["tofu", "93", "18"];
            let numbers: Vec<_> = strings
                .into_iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            println!("Results: {:?}", numbers);
        }
        {
            use std::num::ParseIntError;

            // NOTE: important comments, copied to here:
            //       Result implements FromIter so that a vector of results (Vec<Result<T, E>>)
            //       can be turned into a result with a vector (Result<Vec<T>, E>).
            //       Once an Result::Err is found, the iteration will terminate.

            // NOTE: just for testing that affirmation ^
            fn iteration(value: &str) -> Result<i32, ParseIntError> {
                println!("iteration with value {}", value);

                value.parse()
            }

            let strings = vec!["tofu", "93", "18"];
            // NOTE: wow! wrapping into `Result<T, E>` makes so huge difference -- cool!
            // NOTE: for real -- only one iteration has been performed, awesome!
            let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| iteration(s)).collect();
            println!("Results: {:?}", numbers);
        }
        {
            let strings = vec!["tofu", "93", "18"];
            let (numbers, errors): (Vec<_>, Vec<_>) = strings
                .into_iter()
                .map(|s| s.parse::<i32>())
                .partition(Result::is_ok);
            println!("Numbers: {:?}", numbers);
            println!("Errors: {:?}", errors);
        }
        {
            let strings = vec!["tofu", "93", "18"];
            let (numbers, errors): (Vec<_>, Vec<_>) = strings
                .into_iter()
                .map(|s| s.parse::<i32>())
                .partition(Result::is_ok);
            let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
            let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
            println!("Numbers: {:?}", numbers);
            println!("Errors: {:?}", errors);
        }
    }
}
