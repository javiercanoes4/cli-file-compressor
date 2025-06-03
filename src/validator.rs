use std::fs;

// struct to hold file status - includes the Serialize trait to easily create the reports later
#[derive(serde::Serialize)]
pub struct FileStatus {
    pub path: String,
    pub valid: bool,
    pub reason: Option<String>,
}

// parsing valid files in the directory
pub fn validate_files(dir: &str, allowed_exts: &[&str], max_size: u64) -> Vec<FileStatus> {
    let mut statuses: Vec<FileStatus> = Vec::new();

    // leer files y procesar una a una
    if let Ok(entries) = fs::read_dir(dir) { // throw error if it doesn't exist
        for entry in entries.flatten() { // flatten y nos ahorramos el match de cada result
            let path = entry.path();
            // dropear directorios
            if path.is_file() {
                // insane fuckery to retrieve extension as &OsStr -> convert to string slice -> convert to bool if it matches wit allowed_exts -> set to false if any step fails (devuelve None en vez de Some)
                let ext_valid = path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| allowed_exts.contains(&e))
                    .unwrap_or(false);
                
                // get metadata and check size, unwrap as bool like eralier
                let metadata = fs::metadata(&path);
                let size_valid = metadata.as_ref().map(|m| m.len() <= max_size).unwrap_or(false);

                // assign validness and reason to reject
                let valid = ext_valid && size_valid;
                let reason = if !ext_valid {
                    Some("Invalid extension".to_string())
                } else if !size_valid {
                    Some("Exceeds max size".to_string())
                } else {
                    None
                };

                // add processed file to return array
                statuses.push(FileStatus {
                    path: path.to_string_lossy().to_string(),
                    valid,
                    reason,
                });
            }
        }
    }

    statuses
}