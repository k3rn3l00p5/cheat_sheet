fn err_handling() {
    // panic! macro
    panic!("Example Panic!");

    // unwrapping functions that return a Result<T, E>
    let f = std::fs::File::open("hello.txt").unwrap();

    // expect method on functions that return a Result<T, E>
    let f = std::fs::File::open("hello.txt").expect("Failed to open hello.txt");

    // pattern matching error kinds example
    let f = std::fs::File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// returning result with variant to calling code
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// using ? operator as a shortcut for propagating errors
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) // returns a Ok variant with the value inside it (even though the value was already taken out in the ?)
          // ? can only be used in functions that return result
}
