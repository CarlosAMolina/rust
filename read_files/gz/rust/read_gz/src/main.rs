use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::fs::File;
use libflate::gzip::Decoder;

fn main() {
    let start = Instant::now();
    let path = "src/access.log.10.gz";
    process_file(path);
    println!("Time elapsed {:?}", start.elapsed());
}

fn process_file(path: &str) {
    // https://stackoverflow.com/questions/66250215/rust-and-gzipped-files
    let file = File::open(path).unwrap();
    let mut lines_number = 0;
    for line in BufReader::new(Decoder::new(file).unwrap())
        .lines() {
            println!("{:?}", line);
            lines_number += 1;
        }
    println!("Number of lines: {}", lines_number);
}
