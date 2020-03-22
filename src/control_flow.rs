fn control_flow() {
    // else  if statement
    if first < z {
    } else if x > z && x < z {
    } else {
    }

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // for loop
    for element in a2.iter() {
        println!("{}", element)
    }

    // infinite loop
    loop {
        // empty string slice with type annotation (:)
        let mut guess: String = String::new(); // mutable

        // terminal input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // variable shadowing and parsing
        let guess: u32 = match guess.trim().parse() {
            // error handling
            Ok(num) => num,
            Err(_) => {
                println!("That's not even a number idiot.");
                continue;
            }
        };

        // match expression with cmp method
        match guess.cmp(&DATA3) {
            std::cmp::Ordering::Less => {} // arms
            std::cmp::Ordering::Greater => {}
            std::cmp::Ordering::Equal => {}
        }

        break; // breaking out of a loop
    }
}
