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
        use std::process::Command;

        // NOTE: `new_command()` v1 (with an allocation, `.collect()`)
        // fn new_command() -> Command {
        //     use std::collections::VecDeque;
        //
        //     let raw_command = if cfg!(windows) {
        //         "powershell -command sleep 5"
        //     } else {
        //         "sleep 5"
        //     };
        //
        //     let mut command: VecDeque<&str> = raw_command.split_whitespace().collect();
        //     let program = command.pop_front().unwrap();
        //     let mut child = Command::new(program);
        //
        //     child.args(command);
        //
        //     child
        // }

        // NOTE: `new_command()` v2 (w/o extra allocations)
        fn new_command() -> Command {
            let command = if cfg!(windows) {
                "powershell -command sleep 0"
            } else {
                "sleep 5"
            };

            let mut command_it = command.split_whitespace();
            let program = command_it.next().unwrap();
            let mut child = Command::new(program);

            child.args(command_it);

            child
        }

        let mut child = new_command().spawn().unwrap();
        let _result = child.wait().unwrap();

        println!("reached end of main");
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/fs.html
    println!("\n--- 20.6. Filesystem Operations ---");
    {
        use std::fs;
        use std::fs::{File, OpenOptions};
        use std::io;
        use std::io::prelude::*;
        use std::path::Path;

        // A simple implementation of `% cat path`
        fn cat(path: &Path) -> io::Result<String> {
            let mut f = File::open(path)?;
            let mut s = String::new();

            // match f.read_to_string(&mut s) {
            //     Ok(_) => Ok(s),
            //     Err(e) => Err(e),
            // }
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        // A simple implementation of `% echo s > path`
        fn echo(s: &str, path: &Path) -> io::Result<()> {
            let mut f = File::create(path)?;

            f.write_all(s.as_bytes())
        }

        // A simple implementation of `% touch path` (ignores existing files)
        fn touch(path: &Path) -> io::Result<()> {
            match OpenOptions::new().create(true).write(true).open(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }

        println!("`mkdir a`");
        // Create a directory, returns `io::Result<()>`
        match fs::create_dir("a") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {}
        }

        println!("`echo hello > a/b.txt`");
        // The previous match can be simplified using the `unwrap_or_else` method
        echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        // NOTE: print a created file
        println!("`cat a/b.txt`");
        match cat(&Path::new("a/b.txt")) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(s) => println!("> {}", s),
        }

        println!("`mkdir -p a/c/d`");
        // Recursively create a directory, returns `io::Result<()>`
        fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`touch a/c/e.txt`");
        touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`ln -s ../b.txt a/c/b.txt`");
        // Create a symbolic link, returns `io::Result<()>`
        #[cfg(unix)]
        fn create_sym_link() {
            use std::os::unix;

            unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }

        #[cfg(not(unix))]
        fn create_sym_link() {
            println!("! skipping for Windows");
        }

        create_sym_link();

        println!("`cat a/c/b.txt`");
        match cat(&Path::new("a/c/b.txt")) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(s) => println!("> {}", s),
        }

        println!("`ls a`");
        // Read the contents of a directory, returns `io::Result<Vec<Path>>`
        match fs::read_dir("a") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                for path in paths {
                    println!("> {:?}", path.unwrap().path());
                }
            }
        }

        println!("`rm a/c/e.txt`");
        // Remove a file, returns `io::Result<()>`
        fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`rmdir a/c/d`");
        // Remove an empty directory, returns `io::Result<()>`
        fs::remove_dir("a/c/d").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg.html
    println!("\n--- 20.7. Program arguments ---");
    {
        use std::env;

        let args: Vec<String> = env::args().collect();

        // The first argument is the path that was used to call the program.
        println!("My path is {}.", args[0]);

        // The rest of the arguments are the passed command line parameters.
        // Call the program like this:
        //   $ ./args arg1 arg2
        println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg/matching.html
    println!("\n--- 20.7.1. Argument parsing ---");
    {
        use std::env;

        fn increase(number: i32) {
            println!("{}", number + 1);
        }

        fn decrease(number: i32) {
            println!("{}", number - 1);
        }

        fn help() {
            println!(
                "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
            );
        }

        let args: Vec<String> = env::args().collect();

        match args.len() {
            // no arguments passed
            1 => {
                println!("My name is 'match_args'. Try passing some arguments!");
            }
            // one argument passed
            2 => match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            },
            // one command and one argument passed
            3 => {
                let cmd = &args[1];
                let num = &args[2];
                // parse the number
                let number: i32 = match num.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("error: second argument not an integer");
                        help();
                        return;
                    }
                };
                // parse the command
                match &cmd[..] {
                    "increase" => increase(number),
                    "decrease" => decrease(number),
                    _ => {
                        eprintln!("error: invalid command");
                        help();
                    }
                }
            }
            // all the other cases
            _ => {
                // show a help message
                help();
            }
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/ffi.html
    println!("\n--- 20.8. Foreign Function Interface ---");
    {
        use std::fmt;

        // NOTE: add cfg!
        // this extern block links to the libm library
        #[cfg(unix)]
        #[link(name = "m")]
        extern "C" {
            // this is a foreign function
            // that computes the square root of a single precision complex number
            fn csqrtf(z: Complex) -> Complex;

            fn ccosf(z: Complex) -> Complex;
        }

        #[cfg(not(unix))]
        fn ccosf(v: Complex) -> Complex {
            // dummy implementation
            v
        }

        #[cfg(not(unix))]
        fn csqrtf(v: Complex) -> Complex {
            // dummy implementation
            v
        }

        // Since calling foreign functions is considered unsafe,
        // it's common to write safe wrappers around them.
        fn cos(z: Complex) -> Complex {
            unsafe { ccosf(z) }
        }

        // z = -1 + 0i
        let z = Complex { re: -1., im: 0. };

        // calling a foreign function is an unsafe operation
        let z_sqrt = unsafe { csqrtf(z) };

        println!("the square root of {:?} is {:?}", z, z_sqrt);

        // calling safe API wrapped around unsafe operation
        println!("cos({:?}) = {:?}", z, cos(z));

        // Minimal implementation of single precision complex numbers
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct Complex {
            re: f32,
            im: f32,
        }

        impl fmt::Debug for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.im < 0. {
                    write!(f, "{}-{}i", self.re, -self.im)
                } else {
                    write!(f, "{}+{}i", self.re, self.im)
                }
            }
        }
    }
}
