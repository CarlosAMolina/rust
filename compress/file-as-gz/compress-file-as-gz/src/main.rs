use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::fs;
use tar;
use std::io::prelude::*;
use flate2::write::ZlibEncoder;


fn main() -> Result<(), std::io::Error> {
    let path_name_compressed_file = "/tmp/test/error-new.log.9.gz";
    let tar_gz = File::create(path_name_compressed_file)?;
    let mut enc = GzEncoder::new(tar_gz, Compression::default());

    enc.write_all(b"Hello World")?;
    enc.finish()?;

    /*
    let compressed = enc.finish()?;
    println!("{contents}");

    
    let mut tar = tar::Builder::new(enc);
    let path_name_file_to_compress = "/tmp/test/error-original.log.9";
    let path_name_file_to_create_in_compressed_file = "error-new.log.9";
    let mut f = File::open(path_name_file_to_compress)?;
    tar.append_file(path_name_file_to_create_in_compressed_file, &mut f)?;

    match file.write_all(e) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let contents = fs::read_to_string(path_name_file_to_compress).expect("Should have been able to read the file");
    println!("{contents}");
    */


    Ok(())
}
