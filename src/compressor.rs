use std::fs::File;
use std::io::{Read, Write};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

// handle compression into zip
pub fn create_zip(output: &str, files: &[String]) -> std::io::Result<()> {
    let file = File::create(output)?; // create file or return error
    let mut zip = ZipWriter::new(file); // writer for the file

    // write each file one by one
    for path in files {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?; //hold file data in buffer
        zip.start_file(std::path::Path::new(path).file_name().unwrap().to_str().unwrap(), SimpleFileOptions::default())?; // start of file inside the archive
        zip.write_all(&buffer)?; // write file
    }

    zip.finish()?;
    Ok(()) // return a Result type to hold error reasons
}

// handle compression into tar
pub fn create_tar_gz(output: &str, files: &[String]) -> std::io::Result<()> {
    let tar_gz = File::create(output)?; // create file or return error
    let enc = GzEncoder::new(tar_gz, Compression::default()); // specify gzip encoder this time
    let mut tar = Builder::new(enc); // initialize builder of a file with the gzip encoder

    // appending the path of each file is enough to include it in the archive
    for path in files {
        tar.append_path(path)?;
    }

    tar.finish()?;
    Ok(())
}