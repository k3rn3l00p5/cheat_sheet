// example enum with multiple variants
enum Example {
    Variant1(i32, i32, i32, i32),
    Variant2(String), // each variant has associated data value types
}

fn enums() {
    // enumerator variants initialization examples
    let init_example = Example::Variant1(1, 1, 1, 1);
    let init_example2 = Example::Variant2(String::from("example5"));

    // example enum method usage
    init_example.example_method();

    // option enum example (null (virtually a null value without the safety risks))
    let null_value: Option<i8> = None;

    // option enum example (non-null)
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
}

// enumerator methods
impl Example {
    fn example_method(&self) {
        // method body
    }
}
