use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use tar;

fn main() -> Result<(), std::io::Error> {
    let path_name_compressed_file = "/tmp/file.gz";
    let tar_gz = File::create(path_name_compressed_file)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    let path_name_file_to_compress = "../files/foo.txt";
    let path_name_file_to_create_in_compressed_file = "result/files/foo.txt";
    let mut f = File::open(path_name_file_to_compress)?;
    tar.append_file(path_name_file_to_create_in_compressed_file, &mut f)?;
    Ok(())
}
