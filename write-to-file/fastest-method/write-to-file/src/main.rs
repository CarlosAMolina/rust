use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub fn main() -> Result<(), Box<dyn Error>> {
    let line_to_write = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
    let lines_to_write = 1_000_000;

    let start = Instant::now();
    write_one_line_in_each_loop(line_to_write, lines_to_write)?;
    let duration = start.elapsed();
    println!("Time elapsed (write one line in each loop): {:?}", duration);

    let start = Instant::now();
    write_all_lines_at_once(line_to_write, lines_to_write)?;
    let duration = start.elapsed();
    println!("Time elapsed (write all lines at once): {:?}", duration);

    Ok(())
}

fn write_one_line_in_each_loop(
    line_to_write: &str,
    lines_to_write: u32,
) -> Result<(), Box<dyn Error>> {
    let file_path_name = "/tmp/file-1.txt";
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    for _ in 0..lines_to_write {
        file.write_all(line_to_write.as_bytes())?;
    }
    Ok(())
}

fn write_all_lines_at_once(line_to_write: &str, lines_to_write: u32) -> Result<(), Box<dyn Error>> {
    // Pros: faster.
    // Cons: data stored in memory.
    let file_path_name = "/tmp/file-2.txt";
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    let mut text_to_write_all = String::new();
    for _ in 0..lines_to_write {
        text_to_write_all.push_str(line_to_write);
    }
    file.write_all(text_to_write_all.as_bytes())?;
    Ok(())
}
