use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub fn main() -> Result<(), Box<dyn Error>> {
    let line_to_write = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
    let lines_to_write = 500_000;
    let final_file_bytes: usize = 223_000_000;

    let start = Instant::now();
    write_one_line_in_each_loop(line_to_write, lines_to_write)?;
    let duration = start.elapsed();
    println!("Time elapsed (write one line in each loop): {:?}", duration);

    let start = Instant::now();
    write_one_line_in_each_loop_checking_size(line_to_write, final_file_bytes)?;
    let duration = start.elapsed();
    println!(
        "Time elapsed (write one line in each loop checking file size): {:?}",
        duration
    );

    let start = Instant::now();
    write_all_lines_at_once(line_to_write, lines_to_write)?;
    let duration = start.elapsed();
    println!("Time elapsed (write all lines at once): {:?}", duration);

    let start = Instant::now();
    write_all_lines_at_once_checking_capacity(line_to_write, final_file_bytes)?;
    let duration = start.elapsed();
    println!(
        "Time elapsed (write all lines at once checking capacity): {:?}",
        duration
    );

    let start = Instant::now();
    write_all_lines_at_once_checking_len(line_to_write, final_file_bytes)?;
    let duration = start.elapsed();
    println!(
        "Time elapsed (write all lines at once checking len): {:?}",
        duration
    );

    Ok(())
}

fn write_one_line_in_each_loop(
    line_to_write: &str,
    lines_to_write: u32,
) -> Result<(), Box<dyn Error>> {
    let file_path_name = "/tmp/file-one-line-each-loop.txt";
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    for _ in 0..lines_to_write {
        file.write_all(line_to_write.as_bytes())?;
    }
    Ok(())
}

fn write_one_line_in_each_loop_checking_size(
    line_to_write: &str,
    final_file_bytes: usize,
) -> Result<(), Box<dyn Error>> {
    let file_path_name = "/tmp/file-one-line-each-loop-checking-file.txt";
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    let final_file_bytes_u: u64 = final_file_bytes.try_into()?;
    while file.metadata().unwrap().len() < final_file_bytes_u {
        file.write_all(line_to_write.as_bytes())?;
    }
    Ok(())
}

fn write_all_lines_at_once(line_to_write: &str, lines_to_write: u32) -> Result<(), Box<dyn Error>> {
    // Pros: faster.
    // Cons: data stored in memory.
    let file_path_name = "/tmp/file-all-lines-at-once.txt";
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    let mut text_to_write_all = String::new();
    for _ in 0..lines_to_write {
        text_to_write_all.push_str(line_to_write);
    }
    file.write_all(text_to_write_all.as_bytes())?;
    Ok(())
}

fn write_all_lines_at_once_checking_capacity(
    line_to_write: &str,
    final_file_bytes: usize,
) -> Result<(), Box<dyn Error>> {
    let file_path_name = "/tmp/file-all-lines-at-once-checking-capacity.txt";
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    let mut text_to_write_all = String::new();
    while text_to_write_all.capacity() < final_file_bytes {
        text_to_write_all.push_str(line_to_write);
    }
    file.write_all(text_to_write_all.as_bytes())?;
    Ok(())
}

fn write_all_lines_at_once_checking_len(
    line_to_write: &str,
    final_file_bytes: usize,
) -> Result<(), Box<dyn Error>> {
    let file_path_name = "/tmp/file-all-lines-at-once-checking-len.txt";
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    let mut text_to_write_all = String::new();
    while text_to_write_all.len() < final_file_bytes {
        text_to_write_all.push_str(line_to_write);
    }
    file.write_all(text_to_write_all.as_bytes())?;
    Ok(())
}
