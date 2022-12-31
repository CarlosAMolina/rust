use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use tar;

fn main() -> Result<(), std::io::Error> {
    let path_name_compressed_file = "/tmp/archive.tar.gz";
    let tar_gz = File::create(path_name_compressed_file)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    let path_name_dir_to_compress = "../files";
    let path_name_dir_to_create_in_compressed_file = "result/files";
    tar.append_dir_all(
        path_name_dir_to_create_in_compressed_file,
        path_name_dir_to_compress,
    )?;
    Ok(())
}
