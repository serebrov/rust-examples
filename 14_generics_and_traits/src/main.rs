use std::fmt::Display;
use std::cmp::PartialOrd;

// A generic function with function parameter
fn run_demo<F>(f: F) where F: Fn() {
    f();
    println!("\n\n");
}

// Previous examples of generics
// Option<T>
// Result<T, E>
// Vec<T>
// HashMap<K, V>

fn main() {
    run_demo(demo_largest);
    run_demo(demo_point_generics);
    run_demo(demo_summary_trait);
}

// Generic struct, both x and y should have the same type.
struct Point<T> {
    x: T,
    y: T,
}
// Generic method.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// f32-only method.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// A struct with different types for x and y.
struct Point2<T, U> {
    x: T,
    y: U,
}
// A method can use other types than defined for the struct (T and U),
// here the `mixup` method accepts V and W types:
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
// 
// A trait to show the object.
pub trait Show {
    fn show(&self) -> String;

    // Trait can have the default implementation for
    // the method.
    fn hello(&self) {
      println!("hello!");
    }
}

// Trait implementation for Point struct.
impl<T: Display> Show for Point<T> {
    fn show(&self) -> String {
        format!("{}, {}", self.x, self.y)
    }
}

fn demo_point_generics() {
    let pt = Point {x: 5, y: 7};
    println!("{}", pt.x());
    let pt = Point {x: 5.2, y: 7.2};
    println!("{}", pt.distance_from_origin());

    // Method implemented for Show trait:
    println!("{}", pt.show());
    // Call show via function
    printout(pt);

    let pt1 = Point2 {x: 5, y: 7.2};
    let pt2 = Point2 {x: "Hello", y: 'c'};
    let pt3 = pt1.mixup(pt2);
    println!("pt3.x = {}, pt3.y = {}", pt3.x, pt3.y);
}


// Trait bounds
// Trait bounds allow to speciy which types the generic type
// must implement, for example:
//
//   fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//
// Here we say that some function accepts a `t` parameter of type `T` and
// the type `T` must implement `Display` and `Clone` traits.
// Any type `U` for the `u` parameter must implement `Clone` and `Debug`.

// Function takes a reference to a list of type T 
// and returns an item of type T
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
// 
//     for &item in list.iter() {
//         // Without the trait specified, the following line produces
//         // a compiler error:
//         //    binary operation `>` cannot be applied to type `T`
//         //    ...
//         //    note: `T` might need a bound for `std::cmp::PartialOrd`
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }

// A function that works with types implementing Show
pub fn printout<T: Show>(item: T) {
    println!("{}", item.show());
}

// Regular function to find largest i32 in the list.
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

// Generic function to find largest from the list.
// The T, item type should be comparable, so we specify that
// it should implement the `std::cmp::PartialOrd` trait.
// Also, doing `largest = list[0]` requires copying and
// we also add the `Copy` trait to require T to be copyable.
fn largest_gen<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

// Alternative implementation that does not require Copy.
// Instead of T, it returns a reference to the item in the list:
fn largest_gen_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // Note: in the original code we have &item in list.iter() and
    // here the `&item` actually dereferences the `item` into value.
    //
    // From  https://users.rust-lang.org/t/how-did-a-reference-of-a-reference-become-value/57257/3
    // Patterns [from "pattern matching" concept] usually do kind-of
    // “the opposite” of expressions, so we have
    //    let x = &42;
    //    // x is `&i32`
    //    let &y = x;
    //    // y is `i32` and not `&&i32`
    // 
    // So below we just do `item in list.item()` and the `item` stays
    // reference which we can then assign to `largest` (also a reference)
    // and return.
    for item in list.iter() {
        // `if item > largest` also works due to the automatic dereferencing.
        if *item > *largest {
            largest = item
        }
    }
    largest
}

fn demo_largest() {
    println!("{}", largest(&vec![1,7,5]));
    println!("{}", largest_gen(&vec!['1','7','5']));
    println!("{}", largest_gen_ref(&vec!['1','7','5']));
}

// Trait bounds can be used to have conditional implementations
// where a struct method can be available for some types (matching
// trait bounds) and not avilable for others:
struct Pair<T> {
    x: T,
    y: T,
}
// No trait bounds here, method is available for any types:
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
// Display and PartialOrd trait bounds here, method is available only for
// types implementing these traits:
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can conditionally implement a trait for any type that implements
// another trait.
// For example, the standard rust library implements `ToString` trait for
// any type that implements `Display` trait this way:
// (this is called "blanket implementation")
//
//   impl<T: Display> ToString for T {
//       // --snip--
//   }


// Option<T> is a generic enum,
// any type T can be attached to the `Some` variant
//
//   enum Option<T> {
//       Some(T),
//       None,
//   }

// Result is also a generic enum with T attached to Ok
// and E attached to Err.
//
//    enum Result<T, E> {
//        Ok(T),
//        Err(E),
//    }


// Trait defines an interface
// Summary is a trait for types that have "summarize" method.
pub trait Summary {
    fn summarize(&self) -> String;
}

// Now we can implement the trait for different types
//
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// We can implement trait for standard types, for example, String:
//
// Note: we can implement a trait on a type only if either the trait
// or the type is local to our crate. 
//
// So, for example, if Summary was coming from the external crate,
// we wouldn't be able to implement it for String.
//
// This rule ensures that other people’s code can’t break your code and vice versa.
// Without the rule, two crates could implement the same trait for the same type, 
// and Rust wouldn’t know which implementation to use.
impl Summary for String {
    fn summarize(&self) -> String {
        format!("{}", self)
    }
}


fn demo_summary_trait() {
    let article = NewsArticle {
        headline: String::from("Title"),
        location: String::from("UA"),
        author: String::from("Me"),
        content: String::from("Article")
    };

    let tweet = Tweet {
        username: String::from("Me"),
        content: String::from("Text"),
        reply: false,
        retweet: false
    };

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    println!("String summary: {}", String::from("Test string").summarize());

}
