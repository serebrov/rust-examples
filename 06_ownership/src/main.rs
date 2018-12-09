// Ownership concept is something similar to C++ auto_ptr,
// a "smart pointer" that wraps the regular pointer to
// automatically call the destructor when it has no owner
// (a variable that points to the value in memory).
//
// Rust ownership rules are:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
fn main() {
    // The ownership rules are applied to the scope, for example,
    // a function or an explicit scope created with { / }:
    //
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
        println!("s: {}", s);
    }                      // this scope is now over, and s is no longer valid

    // Below doesn't work and will raise "cannot find value `s` in this scope"
    // println!("{}", s);

    // String object allocates data on the heap, the memory is allocated
    // dynamically (so we have a pointer-like variable).
    // Instead of explicit memory management (like in C), or garbage collection,
    // Rust uses ownership rules to free the dynamic memory when it is not
    // needed anymore.
    // This is similar to Resource Acquisition Is Initialization (RAII)
    // memory management principle in C++.
    let s1 = String::from("Hello");
    // Here the s1 value is moved to s2 (it becomes a new owner),
    // after this, the s1 is not valid anymore.
    let s2 = s1;
    // The `println!("{}", s1);` raises an error.
    println!("s2: {}", s2);

    // We can copy the value into the new variable with `clone`:
    let s3 = s2.clone();
    // Here s2 is still valid:
    println!("s2, s3: {}, {}", s2, s3);

    // Note: integers, floats, booleans and chars (also tuples with
    // these types) are created on stack, so the value is always copied, 
    // here `x` is still valid after the assignment:
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // When we pass the value to the function, we also transfer the 
    // ownership to the function argument:
    let s = String::from("Hello");
    takes_ownership(s);
    // Here `s` is no longer valid, the ownership was transferred
    // to the function and destroyed when it went out of function
    // scope.

    // We can create the data on the heap inside the function and
    // get the ownership via the return value:
    let s = gives_ownership();
    println!("s from function: {}", s);

    // We can transfer the ownership to the function and then back,
    // via the return value:
    let s = String::from("Hello");
    let s = takes_and_gives_back(s);
    println!("s is back: {}", s);

    let mut s = String::from("Hello");
    change(&mut s);
    println!("s: {}", s);
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn gives_ownership() -> String {
    let string = String::from("Hello");
    string
}

fn takes_and_gives_back(string: String) -> String {
    println!("s inside {}", string);
    string
}

fn change(string: &mut String) {
    string.push_str(", world");
}
