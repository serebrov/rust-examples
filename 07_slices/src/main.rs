fn main() {
    let s = "Hello, world!";
    // We can get a slice with &s[n..m] syntax, here `n` is included and
    // `m` is not, to include both sides of the range, use [n..=m] range.
    let word = &s[0..5];
    println!("Word: {}", word);
    let word = &s[0..=4];  //same as above
    println!("Word: {}", word);
    let word = &s[..=4];  //same as above
    println!("Word: {}", word);
    let word = &s[5..];  //from 5 to the end
    println!("Word: {}", word);

    // The [..] means the slice for the whole string,
    // same as [0..len], where len = s.len().
    // Slice type is `&str`.
    let slice: &str = &s[..];
    let first = first_word(slice);
    println!("First word: {}", first);

    // The function that accepts the slice (&str), can also
    // accept the string literal:
    let s = "Hello, world!";
    let first = first_word(s);
    println!("First word from literal: {}", first);

    // The function that accepts the slice (&str), can also
    // accept the String reference:
    let string = String::from("Hello, world!");
    let first = first_word(&string);
    println!("First word from String ref: {}", first);

    // Similarly, it works for arrays:
    let arr = [1, 2, 3, 0, 4, 5];
    let first = first_number(&arr);
    for n in first.iter() {
        print!("{}", n);
    }
}

// If we want to pass the String object, we can use
// &string[..] or just String reference, &string, see
// the usage examples above.
// So it is better to use `&str` as argument type rather
// than &String, as it covers more use cases.
fn first_word(s: &str) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn first_number(arr: &[i32]) -> &[i32] {
    for (i, &n) in arr.iter().enumerate() {
        if n == 0 {
            return &arr[..i];
        }
    }
    return &arr[..];
}
