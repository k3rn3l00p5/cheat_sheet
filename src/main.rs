// removes dead code and unused variable/import warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// bring a path into the scope with use
// re-exporting "use" with "pub"
// path name changing with "as"
pub use other_file::change_me as change;

// load contents of a file with the same name
mod other_file;

// bring all public paths in with glob operator (*)
use crate::other_file::*;

const DATA3: u32 = 2_0; // const variable with visual separator (_)

// Numeric Operations
// +, -, *, /, %

// main function (starting place)
fn main() {
    let data = String::from("Data"); // mutable string slice
    println!("Data 3: {}", DATA3); // print data to console
}
