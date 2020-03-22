fn strings() {
    // Creating a new string
    let s = String::new();

    // to load some initial data into a string use the to_string method
    let s = "initial contents".to_string();

    // we can make a string from anything supported by UTF-8 encoding
    let mut s = String::from("नमस्ते");
    let s2 = String::from("world!");

    // Updating a string
    // you can append to a string with push_str and push
    s.push_str("bar"); // takes a string slice because we don't want to take ownership of the parameter so it's still available for use after the method is called
    s.push('.'); // push method takes a single character and appends it to the string

    // String concatenation using format! macro
    let s = format!("{}-{}-", s, s2); // format macro is better because it doesn't take ownership of any parameters

    // String concatenation using +
    let s = s2 + &s; // note s2 has been moved here and can no longer be used

    // string slicing
    let hello = "Hi";
    let s = &hello[0..1]; // create string slices with caution because if they go out of char boundaries it will panic rust code

    // perform operations on individual unicode scalar values
    for c in "sad".chars() {
        println!("{}", c);
    }
    for b in "happy".bytes() {
        println!("{}", b);
    }
}
