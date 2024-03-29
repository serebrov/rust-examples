use std::io;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;


fn main() {
    // How to interrupt the program with panic macro.
    // panic();

    // How to deal with recoverable errors.
    // recoverable_errors();

    // Error propagation:
    let result = read_username_from_file();
    if let Ok(value) = result {
        println!("Username: {}", value);
    }

    // Note: the situation becomes somewhat more complex if the
    // function wants to propagate more than one error type,
    // for example:
    //
    //     fn read_data_from_file() -> Result<String, io::Error> {
    //        // For example, we read some data from the text file and
    //        // parse it, we can propagate io::Error from here, 
    //        // but can't propagate another error, for example ParseIntError
    //     }
    //
    //  There are several solutions to that, like wrapping errors into
    //  each other or introducing own error types, see:
    //
    //  - https://doc.rust-lang.org/rust-by-example/error/multiple_error_types.html
    //  - http://brson.github.io/2016/11/30/starting-with-error-chain
    //  - https://docs.rs/error-chain/0.12.0/error_chain/
    //  - http://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
    //  Note: the error chain crate seems to be [depreciated](https://github.com/rust-lang-nursery/failure/issues/181) in favor of the [`failure` crate](https://rust-lang-nursery.github.io/failure/).

    // Note: while Result<T, E> concept is much safer than error codes
    // that are used in C or callback(error, result) approach in
    // JavaScript, it is still possible to ignore / miss the fact
    // that the error has happened.
    // Like in the example above - the file does not exist, 
    // so the error has happened, but we only handle the Ok case 
    // and ignore the error.
    // Note: it was also mentioned in the rust book that `if let` construct
    // should be used intentionally, when we explicitely want to only handle
    // one case. The safer way is to use `match` that will force us to check
    // all the options.
}

fn panic() {
    // Unrecoverable Errors with `panic!` macro.
    // When the panic! macro executes, the program prints a failure message,
    // unwinds and cleans up the stack, and then quits.
    //
    // This most commonly occurs when a bug of some kind has been detected
    // and (or) it’s not clear to the programmer how to handle the error.
    panic!("crash and burn");

    // The output:
    //    Compiling panic v0.1.0 (file:///projects/panic)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
    //      Running `target/debug/panic`
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:4
    // note: Run with `RUST_BACKTRACE=1` for a backtrace.

    // To get the stacktrace, run (as suggested) `RUST_BACKTRACE=1 cargo run`.
}

fn recoverable_errors() {
    // Recoverable errors are handled with Result<T, E> enum,
    // it looks like this:
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // So we either get an `Ok` variant with associated result or
    // an `Err` variant with associated error:
    let _f = File::open("hello.txt");
    let _f = match _f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    // Shorter version, using the `unwrap` method, which will panic
    // on error:
    let _f = File::open("hello.txt").unwrap();
    // Or use `expect` that is similar to `unwrap`, but we provide a
    // custom error message
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");

    // It is also possible to match the error kind to perform different
    // actions for different errors:
    let _f = File::open("hello.txt");
    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // Create file if it is not found.
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            // Panic otherwise.
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    // The above can also be written using closures,
    // The Result<T, E> type has various methods that accept a closure, such as `map_err`
    // and `unwrap_or_else`:
    let _f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

// This function propagates the error handing into the calling code,
// it returns the Result<String, io::Error> and we can
// decide how to handle the error in the code that invokes the
// function.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    // return Ok("test".to_string());

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// The error propagating boilerplate can be reduced with the '?' operator:
fn read_username_from_file_short() -> Result<String, io::Error> {
    // The `?` operator will return the error if it happens or
    // we will continue with the value associated with Ok variant.
    //
    // Question: would it be possible (and would it be good) if Rust
    // had the "transparent" implementation for "?" - where we would
    // just write the code without any additional operators and the
    // error would be automatically propagated if the function return
    // type allows that?
    // Probably it would not be good, we should either handle the error
    // here or express an intention to pass the error to the caller.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
