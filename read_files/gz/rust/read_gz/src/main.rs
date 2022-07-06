use flate2::read::GzDecoder;
use libflate::gzip::Decoder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    run_process(process_file_with_decoder_and_lines, "decoder and lines");
    run_process(process_file_with_gz_decoder_and_lines, "gz_decoder and lines");
    run_process(process_file_with_gz_decoder_and_read_to_string, "gz_decoder and read_to_string");
}

fn run_process(function: fn(&str), parse_description: &str) {
    let path = "src/access.log.10.gz";
    let start = Instant::now();
    function(path);
    println!("Time elapsed with {:?}: {:?}", parse_description, start.elapsed());
}

fn process_file_with_decoder_and_lines(path: &str) {
    // https://stackoverflow.com/questions/66250215/rust-and-gzipped-files
    let file = File::open(path).unwrap();
    let mut lines_number = 0;
    for line in BufReader::new(Decoder::new(file).unwrap())
        .lines() {
            let _result = line.unwrap();
            //println!("{}", _result);
            lines_number += 1;
        }
    println!("Number of lines: {}", lines_number);
}

fn process_file_with_gz_decoder_and_lines(path: &str) {
    // https://stackoverflow.com/questions/66250215/rust-and-gzipped-files
    let file = File::open(path).unwrap();
    let mut lines_number = 0;
    for line in BufReader::new(GzDecoder::new(file))
        .lines() {
            let _result = line.unwrap();
            //println!("{}", _result);
            lines_number += 1;
        }
    println!("Number of lines: {}", lines_number);
}

fn process_file_with_gz_decoder_and_read_to_string(path: &str) {
    // https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html#decompress-a-tarball
    let file = File::open(&path).unwrap();
    let mut d = GzDecoder::new(file);
    let mut s = String::new();

    d.read_to_string(&mut s).unwrap();
    //println!("{}", s);
}

