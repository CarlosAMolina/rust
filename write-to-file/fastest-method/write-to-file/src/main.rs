use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub fn main() -> Result<(), Box<dyn Error>> {
    let line_to_write = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
    let start = Instant::now();
    write_one_line_in_each_loop(line_to_write)?;
    let duration = start.elapsed();
    println!("Time elapsed (write one line in each loop): {:?}", duration);
    let start = Instant::now();
    write_all_lines_at_once(line_to_write)?;
    let duration = start.elapsed();
    println!("Time elapsed (write all lines at once): {:?}", duration);
    Ok(())
}

fn write_one_line_in_each_loop(line_to_write: &str) -> Result<(), Box<dyn Error>> {
    let file_path_name = "/tmp/file-1.txt";
    let path = Path::new(&file_path_name);
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => {
            let error_msg = format!("couldn't create {}: {}", display, why);
            return Err(error_msg.into());
        }
        Ok(file) => file,
    };
    for _ in 0..1_000_000 {
        if let Err(why) = file.write_all(line_to_write.as_bytes()) {
            let error_msg = format!("couldn't write to {}: {}", display, why);
            return Err(error_msg.into());
        }
    }
    Ok(())
}

fn write_all_lines_at_once(line_to_write: &str) -> Result<(), Box<dyn Error>> {
    let file_path_name = "/tmp/file-2.txt";
    let path = Path::new(&file_path_name);
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => {
            let error_msg = format!("couldn't create {}: {}", display, why);
            return Err(error_msg.into());
        }
        Ok(file) => file,
    };
    let mut text_to_write_all = String::new();
    for _ in 0..1_000_000 {
        text_to_write_all.push_str(line_to_write);
    }
    if let Err(why) = file.write_all(text_to_write_all.as_bytes()) {
        let error_msg = format!("couldn't write to {}: {}", display, why);
        return Err(error_msg.into());
    }
    Ok(())
}
