use std::collections::HashMap;

fn main() {
    // Note: By default, HashMap uses a cryptographically strong
    // hashing function that can provide resistance to DoS attacks.
    // This is not the fastest hashing algorithm available.
    // If you profile your code and find that the default hash
    // function is too slow for your purposes, you can switch to
    // another function by specifying a different hasher.

    // Create a HashMap and insert some data
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // Create from vector (or tuple) using 'collect' method:
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    // Access value by key (get returns Option enum):
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // Iterate over keys/values:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Insert overwrites the existing value:
    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    scores.insert(&team_name, 10);
    println!("Old value: {:?}", scores.get(&team_name));
    scores.insert(&team_name, 100);
    println!("New value: {:?}", scores.get(&team_name));

    // The `entry.or_insert` can be used to insert the value only
    // if it's not yet present:
    scores.entry(&team_name).or_insert(1000);
    println!("Value: {:?}", scores.get(&team_name));

    // Update the value based on previous value,
    // here we count how many times each word is present:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
