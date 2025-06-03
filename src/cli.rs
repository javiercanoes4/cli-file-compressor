use clap::{Arg, Command};

const DEFAULT_ARCHIVE_NAME: &str = "archive.zip";
// CLI argument setup
pub fn build_cli() -> Command {
    Command::new("CLI File Compressor")
        .about("A simple file compressor by ME (Javier). Not entirely unlike Ark.")
        .arg(Arg::new("input-dir").required(true).long("input-dir").value_name("PATH"))
        .arg(Arg::new("output").long("output").value_name("FILE").default_value(DEFAULT_ARCHIVE_NAME))
        .arg(Arg::new("max-size").long("max-size").value_name("BYTES").value_parser(clap::value_parser!(u64)).default_value("10485760"))
        .arg(Arg::new("ext").long("ext").value_name("EXT1,EXT2,...").default_value("txt,json,csv"))
        .arg(Arg::new("format").long("format").value_name("FORMAT").value_parser(["zip", "tar.gz"]).default_value("zip"))
        .arg(Arg::new("report-format").long("report-format").value_name("FORMAT").value_parser(["json", "text"]).default_value("json"))
}

// hack to change default output to tar.gz if tar.gz is selected
pub fn default_output(format: &str, output: &str) -> String {
    match (output, format) { // match just in case default output was selected but format is not zip
        (DEFAULT_ARCHIVE_NAME, "tar.gz") => "archive.tar.gz".to_string(),
        (_, _) => output.to_string()
    }
}