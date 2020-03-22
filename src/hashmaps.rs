fn hashmaps() {
    // creating a hashmap
    let mut scores = std::collections::HashMap::new();

    // inserting new values into a hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); // keys and values must be of the same type

    // creating a hashmap with vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // ownership example
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // hashmap took ownership of parameters

    // accessing values in a hash map
    let score = scores.get(&String::from("Blue")); // get returns an Option enum with either a some or none value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting a value in a hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // insert a value if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
