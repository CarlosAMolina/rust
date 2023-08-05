use std::env;

// https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
fn main() {
    println!("Hello, world!");
    println!("My OS is {}", env::consts::OS); // Prints the current OS.
}
