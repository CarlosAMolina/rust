use std::error::Error;
use csv::WriterBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path("foo.csv")?;
    wtr.write_record(&["a", "b,asf ", "c"])?;
    wtr.write_record(&["x", "y", "z"])?;
    wtr.flush()?;
    Ok(())
}
