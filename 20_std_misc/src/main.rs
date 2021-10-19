// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 20. Std misc
//    20.1. Threads
//          20.1.1. Testcase: map-reduce
//    20.2. Channels
//    20.3. Path
//    20.4. File I/O
//          20.4.1. open
//          20.4.2. create
//          20.4.3. read_lines
//    20.5. Child processes
//          20.5.1. Pipes
//          20.5.2. Wait
//    20.6. Filesystem Operations
//    20.7. Program arguments
//          20.7.1. Argument parsing
//    20.8. Foreign Function Interface

fn main() {
    // https://doc.rust-lang.org/stable/rust-by-example/std_misc.html
    println!("\n--- 20. Std misc ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/threads.html
    println!("\n--- 20.1. Threads ---");
    {
        use std::thread;

        const NTHREADS: u32 = 10;

        // This is the `main` thread
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];

        for i in 0..NTHREADS {
            // Spin up another thread
            children.push(thread::spawn(move || {
                println!("this is thread number {}", i);
            }));
        }

        for child in children {
            // Wait for the thread to finish. Returns a result.
            let _ = child.join();
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/threads/testcase_mapreduce.html
    println!("\n--- 20.1.1. Testcase: map-reduce ---");
    {
        use std::thread;

        const NTHREADS: usize = 10;

        // This is the `main` thread

        // This is our data to process.
        // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
        // Each whitespace separated chunk will be handled in a different thread.
        //
        // DONE: see what happens to the output if you insert spaces!
        //       NOTE: we will just spawn more threads ¯\_(ツ)_/¯
        //             (in the original variant)
        let data = "8 696789773741647 1853297327050364959
118613225 75564723 9632 97 542624 962850
7085623 4701860 851 90796069 001472 5639
38397966 7071 06094 172783 238747 669219
5238 0795 25788 82365254 593033303 02837
5 84953271357440 4 1 04889788573429781 2
6992021 6 4 38980873548808 4137209 56532
1627842 46374 5258 9 86034537 4828574668";

        // Make a vector to hold the child-threads which we will spawn.
        let mut children = vec![];

        /*************************************************************************
         * "Map" phase
         *
         * Divide our data into segments, and apply initial processing
         ************************************************************************/

        // split our data into segments for individual calculation
        // each chunk will be a reference (&str) into the actual data
        // NOTE: wow! no allocations at all!
        // NOTE: original variant
        // let chunked_data = data.split_whitespace();

        let chunks: Vec<Vec<&str>> = (0..NTHREADS).map(|_| Vec::new()).collect();
        let chunked_data =
            data.split_whitespace()
                .enumerate()
                .fold(chunks, |mut acc, (i, value)| {
                    let subarray_index = i % NTHREADS;

                    acc[subarray_index].push(value);
                    acc
                });
        // Iterate over the data segments.
        // .enumerate() adds the current loop index to whatever is iterated
        // the resulting tuple "(index, element)" is then immediately
        // "destructured" into two variables, "i" and "data_segment" with a
        // "destructuring assignment"

        // NOTE: original variant
        // for (i, data_segment) in chunked_data.enumerate() {
        for (i, data_segments) in chunked_data.into_iter().enumerate() {
            println!("data segments {} is \"{:?}\"", i, data_segments);

            // Process each data segment in a separate thread
            //
            // spawn() returns a handle to the new thread,
            // which we MUST keep to access the returned value
            //
            // 'move || -> u32' is syntax for a closure that:
            // * takes no arguments ('||')
            // * takes ownership of its captured variables ('move') and
            // * returns an unsigned 32-bit integer ('-> u32')
            //
            // Rust is smart enough to infer the '-> u32' from
            // the closure itself so we could have left that out.
            //
            // DONE: try removing the 'move' and see what happens
            //       NOTE: compile error
            children.push(thread::spawn(move || -> u32 {
                // Calculate the intermediate sum of this segment:
                let result = data_segments
                    .into_iter()
                    // NOTE: original variant where data_segments has type &str
                    //       (not it is Vector<&str>)
                    // iterate over the characters of our segment..
                    // .chars()
                    // .. convert text-characters to their number value..
                    .flat_map(|s| s.chars())
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    // .. and sum the resulting iterator of numbers
                    .sum();

                // println! locks stdout, so no text-interleaving occurs
                println!("processed segment {}, result={}", i, result);

                // "return" not needed, because Rust is an "expression language", the
                // last evaluated expression in each block is automatically its value.
                result
            }));
        }

        /*************************************************************************
         * "Reduce" phase
         *
         * Collect our intermediate results, and combine them into a final result
         ************************************************************************/

        // combine each thread's intermediate results into a single final sum.
        //
        // we use the "turbofish" ::<> to provide sum() with a type hint.
        //
        // DONE: try without the turbofish, by instead explicitly
        // specifying the type of final_result
        let children_count = children.len();
        let final_result: u32 = children.into_iter().map(|c| c.join().unwrap()).sum();

        println!("Final sum result: {}", final_result);

        println!("--- Activity ---");
        //        [x] It is not wise to let our number of threads depend on user inputted data.
        //            What if the user decides to insert a lot of spaces? Do we really
        //            want to spawn 2,000 threads? Modify the program so that the data is always
        //            chunked into a limited number of chunks, defined by a static constant
        //            at the beginning of the program.

        assert_eq!(1342, final_result);
        assert_eq!(NTHREADS, children_count);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/channels.html
    println!("\n--- 20.2. Channels ---");
    {
        use std::sync::mpsc;
        use std::sync::mpsc::{Receiver, Sender};
        use std::thread;

        static NTHREADS: i32 = 3;

        // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
        // where `T` is the type of the message to be transferred
        // (type annotation is superfluous)
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut children = Vec::new();

        for id in 0..NTHREADS {
            // The sender endpoint can be copied
            let thread_tx = tx.clone();

            // Each thread will send its id via the channel
            let child = thread::spawn(move || {
                // The thread takes ownership over `thread_tx`
                // Each thread queues a message in the channel
                thread_tx.send(id).unwrap();

                // Sending is a non-blocking operation, the thread will continue
                // immediately after sending its message
                println!("thread {} finished", id);
            });

            children.push(child);
        }

        // Here, all the messages are collected
        let mut ids = Vec::with_capacity(NTHREADS as usize);
        for _ in 0..NTHREADS {
            // The `recv` method picks a message from the channel
            // `recv` will block the current thread if there are no messages available
            ids.push(rx.recv());
        }

        // Wait for the threads to complete any remaining work
        for child in children {
            child.join().expect("oops! the child thread panicked");
        }

        // Show the order in which the messages were sent
        println!("{:?}", ids);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/path.html
    println!("\n--- 20.3. Path ---");
    {
        use std::path::Path;

        // Create a `Path` from an `&'static str`
        let path = Path::new(".");

        // The `display` method returns a `Display`able structure
        let _display = path.display();

        // `join` merges a path with a byte container using the OS specific
        // separator, and returns the new path
        let new_path = path.join("a").join("b");

        // Convert the path into a string slice
        match new_path.to_str() {
            None => panic!("new path is not a valid UTF-8 sequence"),
            Some(s) => println!("new path is {}", s),
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file.html
    println!("\n--- 20.4. File I/O ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html
    println!("\n--- 20.4.1. open ---");
    {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        // Create a path to the desired file
        let path = Path::new("hello.txt");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        // `file` goes out of scope, and the "hello.txt" file gets closed
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/create.html
    println!("\n--- 20.4.2. create ---");
    {
        static LOREM_IPSUM: &str =
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let path = Path::new("lorem_ipsum.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
    println!("\n--- 20.4.3. read_lines ---");
    {
        use std::fs::File;
        use std::io::{self, BufRead};
        use std::path::Path;

        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines("./hosts") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }

        // The output is wrapped in a Result to allow matching on errors
        // Returns an Iterator to the Reader of the lines of the file.
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where
            P: AsRef<Path>,
        {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/process.html
    println!("\n--- 20.5. Child processes ---");
    {
        use std::process::Command;

        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);

            print!("rustc succeeded and stdout was:\n{}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);

            print!("rustc failed and stderr was:\n{}", s);
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/process/pipe.html
    println!("\n--- 20.5.1. Pipes ---");
    {
        if cfg!(windows) {
            println!("Windows has not `wc` command. Exiting...");
        } else {
            use std::io::prelude::*;
            use std::process::{Command, Stdio};

            static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

            // Spawn the `wc` command
            let process = match Command::new("wc")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
            {
                Err(why) => panic!("couldn't spawn wc: {}", why),
                Ok(process) => process,
            };

            // Write a string to the `stdin` of `wc`.
            //
            // `stdin` has type `Option<ChildStdin>`, but since we know this instance
            // must have one, we can directly `unwrap` it.
            match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
                Err(why) => panic!("couldn't write to wc stdin: {}", why),
                Ok(_) => println!("sent pangram to wc"),
            }

            // Because `stdin` does not live after the above calls, it is `drop`ed,
            // and the pipe is closed.
            //
            // This is very important, otherwise `wc` wouldn't start processing the
            // input we just sent.

            // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
            let mut s = String::new();
            match process.stdout.unwrap().read_to_string(&mut s) {
                Err(why) => panic!("couldn't read wc stdout: {}", why),
                Ok(_) => print!("wc responded with:\n{}", s),
            }
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/process/wait.html
    println!("\n--- 20.5.2. Wait ---");
    {
        use std::collections::VecDeque;
        use std::process::Command;

        fn new_command() -> Command {
            let raw_command = if cfg!(windows) {
                "powershell -command sleep 5"
            } else {
                "sleep 5"
            };

            let mut command: VecDeque<&str> = raw_command.split_whitespace().collect();
            let program = command.pop_front().unwrap();
            let mut child = Command::new(program);

            child.args(command);

            child
        }

        let mut child = new_command().spawn().unwrap();
        let _result = child.wait().unwrap();

        println!("reached end of main");
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/fs.html
    // println!("\n--- 20.6. Filesystem Operations ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg.html
    // println!("\n--- 20.7. Program arguments ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg/matching.html
    // println!("\n--- 20.7.1. Argument parsing ---");
    {}

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/ffi.html
    // println!("\n--- 20.8. Foreign Function Interface ---");
    {}
}
