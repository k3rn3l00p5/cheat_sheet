fn vectors() {
    // Initializing vectors
    let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(5); // add a value to a vector

    // using get (returns Option<&T>) to access vector values
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let third: &i32 = &v[2]; // reference to modify content directly

    // iterating through a vector
    for i in &mut v {
        *i += 50;
    }

    // using an enumerator for multiple types in one vector
    enum DifferentTypes {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // storing multiple types in one vector
    let mut row = vec![
        DifferentTypes::Int(3),
        DifferentTypes::Text(String::from("blue")),
        DifferentTypes::Float(10.12),
    ];

    // Pattern Matching to access vector values
    match vec_value {
        DifferentTypes::Int(value) => {
            *value += 20;
            println!("Changed Value: {}", value)
        }
        _ => println!("Something else"),
    }

    // accessing a vector value with mutable references
    let vec_value: &mut DifferentTypes = &mut row[0];

    // if let statement version for more concise code
    if let DifferentTypes::Int(value) = vec_value {
        println!("Value: {}", value);
    }
}
