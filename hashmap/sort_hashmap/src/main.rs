use std::collections::HashMap;

// https://doc.rust-lang.org/std/collections/struct.HashMap.html
fn main() {
    let map = HashMap::from([
        (1, "access.log.1.gz"),
        (0, "access.log"),
        (3, "access.log.3"),
    ]);
    let mut keys = Vec::from_iter(map.keys());
    // The `IntoValues` iterator produces values in arbitrary order, so
    // the values must be sorted to test them against a sorted array.
    keys.sort_unstable();
    println!("{:?}", keys);
    let mut values = Vec::new();
    for key in keys {
        values.push(map.get(key).unwrap());
    } 
    println!("{:?}", values);
}
