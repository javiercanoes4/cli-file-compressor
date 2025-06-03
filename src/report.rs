use crate::validator::FileStatus;
use std::fs::File;
use std::io::Write;

// write report
pub fn write_report(path: &str, statuses: &[FileStatus], format: &str) -> std::io::Result<()> {
    match format { // match depending on format
        "json" => {
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, &statuses)?; // serde saves the day
        },
        "text" => {
            let mut file = File::create(path)?;
            for status in statuses {
                writeln!(file, "{} - {}", status.path, if status.valid { "Valid" } else { status.reason.as_deref().unwrap_or("Invalid") })?;
            }
        },
        _ => unreachable!(),
    }
    Ok(())
}