// removes dead code and unused variable warnings
#![allow(unused_variables)]
#![allow(dead_code)]

// bring something into the scope with use
use std::cmp::Ordering;
use std::io;

const DATA3: u32 = 2_0; // const variable with visual separator (_)

// Scalar Types
// Integers (i/u 8/16/32/64/128/size) [i = signed (-/+)]
// Floating-Point (f 32/64)
// Boolean (true/false)
// Characters (Single Unicode Chars)
/// decimal - 100_000
/// hex - 0xff
/// octal - 0o77
/// binary - 0b1111_0000
/// Byte (u8 only) - b'A'

// Compound Types
// Tuples (data, data, data)
// Arrays (data: i32, data: f32)

// Numeric Operations
// +, -, *, /, %

// Ownership Rules
/// Each value in Rust has a variable that's called its owner
/// There can only be one owner at a time
/// When the owner goes out of scope, the value will be dropped

// main function (starting place)
fn main() {
    let data = String::from("Data"); // mutable string slice
    println!("Random {}", data); // print data to console
    println!("Data 3: {}", DATA3);

    // Tuples
    let tup: (i32, f64, i32) = (500, 1.2, 1); // tuple example
    let (x, y, z) = tup; // destructing
    let first = tup.0; // accessing tuple elements

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type annotations are optional
    let a2 = [3; 5]; // same as [3,3,3,3,3]
    let first = a[0]; // accessing array elements
    println!("{}", &data[0..2]); // string slicing

    // * are dereferences
    // & are references

    // Structs
    struct Example {
        field1: String, // arms (name: Type)
        field2: String,
        field3: i32,
        field4: bool,
    }

    // Initiating a struct
    let mut init_example = Example {
        field4: false,
        field2: String::from("example"),
        field3: 32,
        field1: String::from("example"),
    };

    // Changing individual struct fields
    init_example.field3 = 22;
    init_example.field4 = true;
    init_example.field1 = String::from("example2");
    init_example.field2 = String::from("example2");

    // returning struct from a fn
    fn build_example(field1: String, field2: String) -> Example {
        Example {
            field1,
            field2,
            field4: true,
            field3: 22,
        }
    }

    // example struct building function usage
    init_example = build_example(String::from("example3"), String::from("example3"));
    println!("Example {}", init_example.field1); // accessing struct elements

    // example enum with multiple variants
    enum Example2 {
        Variant1(i32, i32, i32, i32),
        Variant2(String), // each variant has associated data value types
    }

    // enumerator variants initialization examples
    let init_example2 = Example2::Variant1(1, 1, 1, 1);
    let init_example3 = Example2::Variant2(String::from("example5"));

    // enumerator methods
    impl Example2 {
        fn example2_method(&self) {
            // method body
        }
    }

    // example enum method usage
    init_example2.example2_method();

    // null value (virtually a null value without the safety risks)
    let null_value: Option<i8> = None;

    // some option enum example (non-null)
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let example (checks a single pattern)
    if let Some(3) = some_u8_value {
        println!("three");
    } else if let Some(4) = some_u8_value {
        println!("four");
    } else {
        println!("some other number");
    }

    // Tuple structs
    struct TupStruct(i32, i32, i32); // no field names
    let init_tup_struct = TupStruct(0, 0, 0); // usage
    println!(
        "{} {} {}",
        init_tup_struct.0, init_tup_struct.1, init_tup_struct.2
    ); // accessing tuple struct data

    // implementation blocks on structs
    impl Example {
        fn example_method(&self) -> i32 {
            self.field3 * self.field3
        }
    }

    // associated function
    impl Example {
        fn example_associated_fn() -> Example {
            Example {
                field4: false,
                field2: String::from("example"),
                field3: 32,
                field1: String::from("example"),
            }
        }
    }

    // method usage
    init_example.example_method();

    // associated function
    Example::example_associated_fn();

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
            Ordering::Less => {} // arms
            Ordering::Greater => {}
            Ordering::Equal => {}
        }

        break; // breaking out of a loop
    }
}
