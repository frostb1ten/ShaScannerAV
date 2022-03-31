extern crate core;
use std::fs::File;
use walkdir::WalkDir;
use std::env;
use std::io::{self, BufReader, BufRead};

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let directory = &args[1];
    search(directory.to_string());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn search(dir: String) {
    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) =>
                files(entry.path().display().to_string()),
            Err(err) => {
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::InvalidData => {
                            continue;
                        }
                        io::ErrorKind::PermissionDenied => {
                            continue;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }
    }
}


fn files(file: String) {
    for hash in sha256::digest_file(&file) {
        let definitions = "./hashes.txt";
        let files = File::open(definitions).unwrap();
        let reader = BufReader::new(files);
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line == hash {
                println!("FOUND:{} for file {}", hash, file);
            }
        }
    }
}
