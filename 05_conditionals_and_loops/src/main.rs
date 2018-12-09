fn main() {
    let number = 3;

    // if / else if / else structure:
    if number > 5 {
        println!("number is greater than 5");
    } else if number < 3 {
        println!("number is less than 3");
    } else {
        println!("number is between 3 and 5");
    }

    let condition = true;
    // Conditional statement:
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // Loop with break.
    let mut counter = 0;
    // Loop expression has a value:
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break defines the loop expression value
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // Conditional "while" loop.
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    // for-in loop.
    let a = [10, 20, 30];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Reverse for-in loop (from 1 to 3).
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
