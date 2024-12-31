use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // get return option value
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // all the keys and value must have the same type

    println!("score is {:?}", scores);
    // split_shiteshpacd

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");


}
