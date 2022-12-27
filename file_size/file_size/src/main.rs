use std::fs::File;

fn main() -> std::io::Result<()> {
    let file_path_name = "../file-to-get-size.txt";
    let f = File::open(file_path_name)?;
    let file_size = f.metadata().unwrap().len();
    assert_eq!(446, file_size);
    Ok(())
}

