fn main() {
    // New string:
    let mut s = String::new();
    // Update string:
    s.push_str("content");
    s.push_str(", more");
    println!("{}", s);

    // Concatenate with literal,
    // Note: the 's' value is moved here,
    // so 's' is not available after that:
    let s2 = s + ", even more";
    println!("{}", s2);

    // Concatenate String objects:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
    println!("{}", s3);

    // Concatenate using format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Convert from the string literal,
    // same as doing s = String::from("initial contents"):
    let s = "initial contents".to_string();
    println!("{}", s);

    // Strings can not be indexed directly, like below,
    // because the content is multi-byte unicode
    //    let s = String::from("hello");
    //    let h = s[0];  // Error: String can not be indexed by integer.
    let len = String::from("Здравствуйте").len();
    // The length is 24 (bytes), although we only have 12 letters.
    println!("Length: {}", len);

    // We can index by range to get a string slice:
    let s = String::from("hello");
    let h = &s[0..1];
    println!("Element: {}", h);

    // But it still won't work for unicode data:
    // Panic: byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
    //    let s = String::from("Здравствуйте");
    //    let h = &s[0..1];

    // Hindi word “नमस्ते” written in the Devanagari script,
    // it is stored as a vector of u8 values that looks like this: 
    //     [224, 164, 168, ..., 165, 135] (18 bytes)
    // If we look at them as Unicode scalar values (Rust char), we get:
    //     ['न', 'म', 'स', '्', 'त', 'े'] (6 chars)
    // but the fourth and sixth are not letters: they’re diacritics 
    // that don’t make sense on their own. 
    // If we look at them as grapheme clusters, we’d get what a person would call the four letters:
    //     ["न", "म", "स्", "ते"]
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // Grapheme clusters (the closest thing to what we would call letters).
    // Getting grapheme clusters from strings is complex, 
    // so this functionality is not provided by the standard library. 
    // Crates are available on crates.io if this is the functionality you need.

}
