use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let path_name_compressed_file = "/tmp/test/error-new.log.9.gz";
    let tar_gz = File::create(path_name_compressed_file)?;
    let mut enc = GzEncoder::new(tar_gz, Compression::default());
    let path_name_file_to_compress = "/tmp/test/error-original.log.9";
    let contents = fs::read_to_string(path_name_file_to_compress)?;
    enc.write_all(contents.as_bytes())?;
    enc.finish()?;
    Ok(())
}
