fn main() {
    enum_definition();
    option_enum();
    enum_match();
    enum_match_pattern_bind();
    option_match();
    default_match();
    if_let_match();
}

fn enum_definition() {
    // Define Enum with two values: V4 and V6:
    enum IpAddrKind {
        V4,
        V6,
    }
    // Assign enum values to variables:
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // Define Enum with attachable String value:
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    // We can have different types attached to different
    // Enum values:
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));

    // We can put any kind of data inside an enum variant:
    // strings, numeric types, or structs, or even another enum.
    #[derive(Debug)]
    enum Message {
        Quit,                       // no associated data
        Move { x: i32, y: i32 },    // anonymous struct
        Write(String),              // String
        ChangeColor(i32, i32, i32), // three i32 values
    }

    // We can define methods for Enums:
    impl Message {
        fn call(&self) {
            // method body would be defined here
            println!("{:#?}", self);
        }
    }
    let message = Message::Write(String::from("hello"));
    message.call();
    let message = Message::Move{x: 10, y: 100};
    message.call();
    let message = Message::ChangeColor(0xff, 0xff, 0xff);
    message.call();
    let message = Message::Quit;
    message.call();
}

fn option_enum() {
    // The `Option` enum from the standard library represents
    // value - or - null scenario.
    // Note: Option is defined in the standard library, as well as
    // Some and None options; also Some and None can be used without
    // the `Option::` prefix.
    // The `Option` enum is defined approximately like this:
    enum MyOption<T> {
        Some(T),
        None,
    }
    let _maybe_number = MyOption::Some(5);
    let _maybe_string = MyOption::Some("String");
    let _no_number: MyOption<i32> = MyOption::None;

    // To use the `Option` value, it needs to be converged to the T type:
    let x: i32 = 1;
    let y: Option<i32> = Some(1);
    // We need to 'unwrap' Some value to use it,
    // see https://doc.rust-lang.org/std/option/enum.Option.html for other
    // methods.
    let sum = x + y.unwrap();
    println!("Sum: {}", sum);
    // This will result in panic and crash:
    //    let z: Option<i32> = None;
    //    let sum = x + z.unwrap();
    //    println!("Sum: {}", sum);
}

fn enum_match() {
    // The `match` operator can be used to perform different actions
    // depending on the Enum value.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    println!("Penny: {}", value_in_cents(Coin::Penny));
    println!("Nickel: {}", value_in_cents(Coin::Nickel));
    println!("Dime: {}", value_in_cents(Coin::Dime));
    println!("Quarter: {}", value_in_cents(Coin::Quarter));

    // We can also have blocks of code in the match arms
    // (arm is the same as "case" in switch-case operator):
    fn print_value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Penny");
                1
            },
            Coin::Nickel => {
                println!("Nickel");
                5
            },
            Coin::Dime => {
                println!("Dime");
                10
            },
            Coin::Quarter => {
                println!("Quarter");
                25
            },
        }
    }

    println!("{}", print_value_in_cents(Coin::Penny));
    println!("{}", print_value_in_cents(Coin::Nickel));
    println!("{}", print_value_in_cents(Coin::Dime));
    println!("{}", print_value_in_cents(Coin::Quarter));
}

fn enum_match_pattern_bind() {
    // Patterns that bind to values.
    #[derive(Debug)] // So we can inspect the state
    enum UsState {
        Alabama,
        Alaska,
    }
    // Here the Quarter variant has associated UsState value:
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    // In the `match` below we use `Coin::Quarter(state)` syntax,
    // and the associated `state` value becomes available in the
    // arm code:
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // Here we can use `state`
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(
        Coin::Quarter(UsState::Alaska)));
    println!("{}", value_in_cents(
        Coin::Quarter(UsState::Alabama)));
}

fn option_match() {
    // The `match` operator is useful with Option<T> enum.
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Six: {}", six.unwrap());
    println!("Is None: {}", none.is_none());
}

fn default_match() {
    // The `_` placeholder can be used as a default match:
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),  // The `()` value is a unit value, so we do nothing.
    }
}

fn if_let_match() {
    // The if-let construct is useful when we only want to do something
    // for one specific match:
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(0) => println!("zero"),
        _ => (),
    }

    // Instead of the above, we can use if-let:
    if let Some(0) = some_u8_value {
        println!("zero");
    }

    // We can use else in the if-let too:
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}
