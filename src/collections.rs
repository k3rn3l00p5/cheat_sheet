fn collections() {
    // Tuples
    let tup: (i32, f64, i32) = (500, 1.2, 1); // tuple example
    let (x, y, z) = tup; // destructing
    let first = tup.0; // accessing tuple elements

    // Tuple structs
    struct TupStruct(i32, i32, i32); // no field names
    let init_tup_struct = TupStruct(0, 0, 0); // usage
    println!(
        "{} {} {}",
        init_tup_struct.0, init_tup_struct.1, init_tup_struct.2
    ); // accessing tuple struct data

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type annotations are optional
    let a2 = [3; 5]; // same as [3,3,3,3,3]
    let first = a[0]; // accessing array elements
    println!("{}", &data[0..2]); // string slicing

    // Vectors
    // snip //

    // Strings
    // snip //

    // HashMaps
    // snip //
}
