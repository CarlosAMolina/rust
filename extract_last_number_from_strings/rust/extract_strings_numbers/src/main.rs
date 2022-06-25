use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let filenames = vec![
        "foo",
        "foo.txt",
        "access.log",
        "access.log.5",
        "access.log.2",
        "access.log.10",
        "access.log.1",
    ];
    let result_expected = vec![5, 2, 10, 1];
    let mut start = Instant::now();
    assert_eq!(
        result_expected,
        get_filenames_numbers_with_regex_captures(&filenames)
    );
    println!("Time elapsed with regex captures: {:?}", start.elapsed());
    start = Instant::now();
    get_filenames_numbers_with_regex_match(&filenames);
    println!("Time elapsed with regex match: {:?}", start.elapsed());
    start = Instant::now();
    assert_eq!(
        result_expected,
        get_filenames_numbers_with_split(&filenames)
    );
    println!("Time elapsed with split: {:?}", start.elapsed());
}

fn get_filenames_numbers_with_regex_match(filenames: &[&str]) -> Vec<u8> {
    lazy_static! {
        static ref FILE_NUMBER: Regex = Regex::new(r"^access\.log\.(?P<file_number>\d+)").unwrap();
    }
    let mut numbers = Vec::<u8>::new();
    for filename in filenames.iter() {
        if FILE_NUMBER.is_match(filename) {
             numbers.push(1)
        }
    }
    numbers
}


fn get_filenames_numbers_with_regex_captures(filenames: &[&str]) -> Vec<u8> {
    lazy_static! {
        static ref FILE_NUMBER: Regex = Regex::new(r"^access\.log\.(?P<file_number>\d+)").unwrap();
    }
    let mut numbers = Vec::<u8>::new();
    for filename in filenames.iter() {
        FILE_NUMBER.captures(filename).and_then(|cap| {
            cap.name("file_number")
                .map(|number| numbers.push(number.as_str().parse::<u8>().unwrap()))
        });
    }
    numbers
}

fn get_filenames_numbers_with_split(filenames: &[&str]) -> Vec<u8> {
    let mut numbers = Vec::<u8>::new();
    for filename in filenames.iter() {
        let last_part = filename.split('.').last();
        if let Ok(number) = last_part.unwrap().parse::<u8>() {
            numbers.push(number)
        }
    }
    numbers
}
