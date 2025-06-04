# CLI File Compressor

A simple file compressor written in Rust.

## Description

This is a CLI tool that creates a compressed archive of all the files inside a directory (non-recursively), according to a set of rules. It can filter files by:
- File extension.
- Maximum size.

Example output:
```
Files scanned: 7
Accepted: 2
Rejected: 5
Archive: archive.zip
Report: validation_report.json
```

Additionally, it creates a report of the processed files and their rejection reason in either `.txt` or `.json`.
```json
[
  {
    "path": "./files/a.txt",
    "valid": true,
    "reason": null
  },
  {
    "path": "./files/b.txt",
    "valid": true,
    "reason": null
  },
  {
    "path": "./files/program.AppImage",
    "valid": false,
    "reason": "Exceeds max size"
  },
  {
    "path": "./files/document.pdf",
    "valid": false,
    "reason": "Invalid extension"
  }
]
```

```
./files/a.txt - Valid
./files/b.txt - Valid
./files/program.AppImage - Exceeds max size
./files/document.pdf - Invalid extension
```



## Building

### Natively

* [Install Rust and Cargo.](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* Clone this repository:
```
git clone https://github.com/javiercanoes4/cli-file-compressor.git
```
* Build with Cargo:
```
cd ./cli-file-compressor
cargo build --release
```
The executable should have been built inside `target/release/`.

### With Docker

* Clone this repository:
```
git clone https://github.com/javiercanoes4/cli-file-compressor.git
```

* Run docker compose to build with Docker:
```
cd ./cli-file-compressor
docker-compose up
```
The executable `cli-data-compressor` should be available in the root directory of this project.

This pulls the `rust:1.87` Docker image which is somewhat big. Remove the image and container if desired after building.

### Debug version

* [Install Rust and Cargo.](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* Clone this repository:
```
git clone https://github.com/javiercanoes4/cli-file-compressor.git
```

* Run with Cargo as if you were running the executable:
```
cd ./cli-file-compressor
cargo run -- --help
```


## Usage

Print the usage using the `--help` argument:
```
Usage: cli-file-compressor [OPTIONS] --input-dir <PATH>

Options:
      --input-dir <PATH>        
      --output <FILE>           [default: archive.zip]
      --max-size <BYTES>        [default: 10485760]
      --ext <EXT1,EXT2,...>     [default: txt,json,csv]
      --format <FORMAT>         [default: zip] [possible values: zip, tar.gz]
      --report-format <FORMAT>  [default: json] [possible values: json, text]
  -h, --help                    Print help
```

* `--input-dir`: REQUIRED. Indicates the directory that contains the files to be compressed.
* `--output`: The path to the output archive.
* `--max-size`: Filters out files exceeding this value (in bytes).
* `--ext`: Filters out files that are not of these extensions. Is a comma separated list.
* `--format`: Format used for the output archive. Either `zip` or `tar.gz`.
* `--report-format`: The desired format of the report. Either `json` or `text`.


## Author

[javiercanoes4](https://github.com/javiercanoes4)


## Acknowledgments

Project idea by [pablomendez95](https://hackmd.io/@pablomendez95) - https://hackmd.io/ZUUopEnGQVSWc8Fs0yDWDg?view