fn main() {
    // Ok
    let value = get_value();
    println!("{:?}", value);
    let result = return_value(&value);
    println!("{:?}", result);

    // Error
    // More info with `rustc --explain E0716`.
    let result = return_value(&get_value());
    println!("{:?}", result);
}

fn get_value() -> u8 {
    let x=3;
    x
}

fn return_value(value: &u8) -> &u8 {
    value
}
