use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key,value) in &scores {
        println!("{}: {}",key, value);
    }
    println!();
    // Only inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(7000);

    for (key,value) in &scores {
        println!("{}: {}",key, value);
    }
}