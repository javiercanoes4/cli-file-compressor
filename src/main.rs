mod cli;
mod validator;
mod compressor;
mod report;

use validator::*;
use std::path::Path;

fn main() {
    // process cli stuff (args)
    let matches = cli::build_cli().get_matches();

    // check si el path existe and panic
    let input_dir = matches.get_one::<String>("input-dir").unwrap();
    if !Path::new(input_dir).exists() {
        eprintln!("Error: input directory does not exist.");
        std::process::exit(1);
    }

    // parse args, panic if fail (con unwrap)
    let output = matches.get_one::<String>("output").unwrap();
    let max_size = *matches.get_one::<u64>("max-size").unwrap();
    let exts: Vec<&str> = matches.get_one::<String>("ext").unwrap().split(',').collect();
    let format = matches.get_one::<String>("format").unwrap().as_str();
    let output = &cli::default_output(format, output); // correct the default archive name
    let report_format = matches.get_one::<String>("report-format").unwrap().as_str();


    println!("Args looking good. Starting...");

    // statuses to hold status of all files and valid_paths to filter files to be compressed
    let statuses: Vec<FileStatus> = validate_files(input_dir, &exts, max_size);
    let (valid, rejected): (Vec<&FileStatus>, Vec<&FileStatus>) = statuses.iter().partition(|f| f.valid); // tupla de arrays de FileStatus
    let valid_paths: Vec<String> = valid.iter().map(|f| f.path.clone()).collect(); // collect los paths en un array de strings

    // write report or indicate error but DON'T PANIC
    let report_path = format!("validation_report.{}", report_format);
    if let Err(e) = report::write_report(&report_path, &statuses, report_format) {
        eprintln!("Error writing report: {}", e);
        //std::process::exit(1);
    }

    // match and use the corresponding compression - unreachable panics as it shouldnt be reached
    let archive_result = match format {
        "zip" => compressor::create_zip(output, &valid_paths),
        "tar.gz" => compressor::create_tar_gz(output, &valid_paths),
        _ => unreachable!(),
    };

    match archive_result {
        Ok(_) => {
            println!("Job done.");
            println!("Files scanned: {}", statuses.len());
            println!("Accepted: {}", valid.len());
            println!("Rejected: {}", rejected.len());
            println!("Archive: {}", output);
            println!("Report: {}", report_path);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Compression failed: {}", e);
            std::process::exit(1);
        }
    }
}