use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, val) in &scores {
        println!("{}, {}", key, val);
    }

    // zip
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for (key, val) in &scores {
        println!("{}, {}", key, val);
    }

    // get value from hash
    let k = String::from("key");
    let v = String::from("val");
    let mut map = HashMap::new();
    map.insert(k, v);
    // println!("{}, {}", k, v); // throw error : value used here after move

    let key = String::from("key");
    let foo = String::from("foo");
    // has key
    println!("{}", map.get(&key).unwrap());
    // does not have key
    println!("{:?}", map.get(&foo));

    // update hash
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // update score with Yellow, 50
    scores.entry(String::from("Yellow")).or_insert(50);
    // does not update score because score has Blue key already
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let txt = "hello world wonderful world";
    let mut map = HashMap::new();

    for w in txt.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}
