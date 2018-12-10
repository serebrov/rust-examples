fn main() {
    // Create the vector, specify the type:
    let v: Vec<i32> = Vec::new();
    println!("\nNew vector: ");
    for el in v.iter() {
        print!("{} ", el);
    }

    // Create with macro:
    let v = vec![1, 2, 3];
    println!("\nMacro vector: ");
    for el in v {
        print!("{} ", el);
    }

    // Infer the type from the 'push' operation:
    let mut v = Vec::new();
    println!("\nAuto vector: ");
    // The type is inferred when we push an item.
    v.push(1);
    for idx in 0..v.len() {
        // Access the element by index:
        print!("{} ", v[idx]);
    }

    println!("\nOut-of-bounds access: ");
    // Access via `get` method - this will not panic 
    // if there is no such element:
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(value) => println!("Have the element: {}", value),
        None => println!("No element")
    }
    match v.get(0) {
        Some(value) => println!("Have the element: {}", value),
        None => println!("No element")
    }

    // Modify in a loop
    let mut v = vec![1, 2, 3];
    println!("\nMutable elements: ");
    for el in &mut v {
        *el += 10;
    }
    for el in v {
        print!("{} ", el);
    }
    println!();

    // Store values of different types in vector via enum:
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for el in row {
        print!("{:#?} ", el);
    }
    println!();
}
