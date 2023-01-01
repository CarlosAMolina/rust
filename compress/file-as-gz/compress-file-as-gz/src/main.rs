use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let path_name_compressed_file = "/tmp/foo.txt.gz";
    let path_name_file_to_compress = "../files/foo.txt";
    compress_file_as_gz(path_name_file_to_compress, path_name_compressed_file)?;
    Ok(())
}

fn compress_file_as_gz(
    path_name_file_to_compress: &str,
    path_name_compressed_file: &str,
) -> Result<(), std::io::Error> {
    let file_gz = File::create(path_name_compressed_file)?;
    let mut enc = GzEncoder::new(file_gz, Compression::default());
    let contents = fs::read_to_string(path_name_file_to_compress)?;
    enc.write_all(contents.as_bytes())?;
    enc.finish()?;
    Ok(())
}
