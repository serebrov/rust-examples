// Functions are declared with `fn` keyword.
fn main() {
    // Variables are immutable by default.
    // Below we don't specify the type, but Rust is statically typed,
    // the type is derived automatically from the function return value.
    let x = plus_one(4);
    println!("The value of x is: {}", x);

    // We can not modify the immutable variable,
    // for example x = x + 1 won't work,
    // but we can re-assign (shadow) the value with new one:
    let x = 10;
    println!("The new value of x is: {}", x);

    // The `mut` modifier is used to declare mutable variable.
    let mut y = 5;
    y = y + 1;
    println!("The value of y is: {}", y);

    // Constants are different from immutable variables:
    // - the value must be static (can not be function result for example)
    // - the type must be annotated
    // Below we have `u32` - unsigned 32-bit integer, signed type is `i32`.
    // Note: the numeric value can contain `_` for clarity, 
    // below the value is 100000:
    const X: u32 = 100_000;
    println!("The value of X is: {}", X);

    // Other integer types are 
    //  i8 / u8, i16 / u16, i32 / u32, i64 / u64, i128 / u128 
    //  and isize / usize (depend on the architecture - 32 or 64 bit)
    //
    // There are various integer literals:
    println!("Decimal: {}", 10_000);
    println!("Hex: {}", 0xff);
    println!("Octal: {}", 0o77);
    println!("Binary: {}", 0b1111_0000);
    println!("Byte (u8 only): {}", b'A');

    // Integer overflow: panic in debug build,
    // but "two's complement wrapping" in release
    // let mut i: u8 = 255;
    // i = i + 1;
    // println!("Overflow: {}", i);

    // Floating-point types are f32 and f64 (default).
    let f = 2.2;  // f64
    println!("Float: {}", f);

    // Boolean
    let b: bool = true;  // or false
    println!("Boolean: {}", b);

    // Char (unicode)
    let c = '😻';
    println!("Char: {}", c);

    // Array, type is derived from the initializer,
    // all elements have the same type.
    // Arrays are allocated on stack and can not be resized dynamically.
    let mut arr = [1, 2, 3];
    arr[0] = arr[0] + 1;
    println!("Array element: {}", arr[0]);

    // Tuple, elements can have different types:
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructure the tuple into variables:
    let (a, b, c) = tup;
    println!("Tuple values: {}, {}, {}", a, b, c);
    // Modify the tuple member:
    tup.0 = tup.0 + 55;
    // Tuple members can be accessed with `.`:
    println!("Tuple values: {}, {}, {}", tup.0, tup.1, tup.2);

    // Call inner_scope function.
    println!("Result {}", inner_scope(3))
}

// Argument types and return type of the function are annotated
// like this:
fn plus_one(x: i32) -> i32 {
    // Last expression is used as return value,
    // note that is should not have a `;` at the end:
    x + 1
}

fn inner_scope(x: i32) -> i32 {
    // Curly braces start the new scope that is also an expression
    // and can be used as a value:
    let y = {
        let z = x;
        z + 1
    };

    y + 1
}
