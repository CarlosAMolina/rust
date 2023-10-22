use std::error::Error;
use std::fs::OpenOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/tmp/foo.csv";
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&["a", "b,asf ", "c"])?;
    wtr.write_record(&["x", "y", "z"])?;
    wtr.flush()?;
    Ok(())
}
