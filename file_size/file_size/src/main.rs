use std::fs::File;

fn main() {
    let file_path_name = "../file-to-get-size.txt";
    let file_size = get_file_size(file_path_name).unwrap();
    assert_eq!(446, file_size);
}


fn get_file_size(file_path_name: &str) -> std::io::Result<u64> {
    let f = File::open(file_path_name)?;
    let file_size = f.metadata().unwrap().len();
    Ok(file_size)
}

