// Structure is a collection of fields of various types.
// The #[derive(Debug)] is to be able to print the struct.
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u64,
}

// Define a method of the structure.
impl User {
    fn display_name(&self) -> String {
        self.username.clone()
    }

    fn display_status(&self) -> String {
        format!("Status: {}, login count: {}", self.active, self.login_count)
    }
}

// Constructor-like function with defaults.
fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email,  // shorthand syntax, field name same as parameter name
        active: true,
        login_count: 0,
    }
}

// Tuple struct, without named fields.
// Note: It is almost always better to use a struct rather than a tuple struct.
struct Point(i32, i32);

// Function that works with the structure.
fn move_point(pt: &Point, dx: i32, dy: i32) -> Point {
    Point(pt.0 + dx, pt.1 + dy)
}

// There is one case when a tuple struct is very useful, though, 
// and that’s a tuple struct with only one element.
// We call this the ‘newtype’ pattern, because it allows you 
// to create a new type, distinct from that of its contained 
// value and expressing its own semantic meaning:
struct Inches(i32);

// Unit struct, without fields.
// It behaves similarly to the `()`, the unit type.
#[derive(Debug)]
struct MyUnit();


fn main() {
    // Create struct instance directly and make it mutable.
    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("tester"),
        active: true,
        login_count: 0,
    };

    user1.email = String::from("test2@example.com");
    // Printing here is handled by #[derive(Debug)]
    // The {:?} prints the debug one-line representation of the struct,
    // and {:#?} outputs the formatted (multi-line) debug representation.
    println!("User: {:#?}", user1);
    println!("User: {:#?}", user1.display_name());
    println!("User: {:#?}", user1.display_status());

    // Create using constructor-like function:
    let user2 = build_user(
        String::from("Joe"),
        String::from("joe@example.com"),
    );
    println!("User: {:?}", user2);

    // Create using struct update syntax:
    let joe2 = User {
        username: String::from("Joe2"),
        ..user2
    };
    println!("User: {:?}", joe2);

    // Create tuple struct instance:
    let pt = Point(0, 0);
    println!("{}, {}", pt.0, pt.1);
    let pt2 = move_point(&pt, 5, 5);
    println!("{}, {}", pt2.0, pt2.1);

    // Create a regular tuple:
    let pt3 = (10, 10);
    println!("{}, {}", pt3.0, pt3.1);

    // Create unit struct instance:
    let unit = MyUnit();
    // Print with {:?} works because we have #[derive(Debug)] for MyUnit
    println!("My Unit: {:?}", unit);

    // Create 'newtype' structure instance:
    let length = Inches(10);
    // Extract the inner integer type through a destructuring let,
    // just as with regular tuples:
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}
