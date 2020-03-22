// Structs
struct Example {
    field1: String, // arms (name: Type)
    field2: String,
    field3: i32,
    field4: bool,
}

fn structs() {
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

    // example struct building function usage
    init_example = build_example(String::from("example3"), String::from("example3"));
    println!("Example {}", init_example.field1); // accessing struct elements

    // struct method usage
    init_example.example_method();

    // associated function
    Example::example_associated_fn();
}

// returning struct from a fn
fn build_example(field1: String, field2: String) -> Example {
    Example {
        field1,
        field2,
        field4: true,
        field3: 22,
    }
}

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
