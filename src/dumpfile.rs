use crate::error::*;
use crate::solver::{Operation, Operations};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

/// Dump operations intto file in JSON format
pub fn dump_to_file(prefix: String, operations: &[Operation]) -> Result<()> {
    let now = chrono::Local::now();

    let cwd = std::env::current_dir().map_err(|e| Error {
        kind: ErrorKind::Generic,
        value: Some(e.to_string()),
    })?;

    let dump = DumpFormat {
        date: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        directory: cwd.to_str().unwrap().to_string(),
        operations: operations.to_vec(),
    };

    // Create filename with the following syntax: "rnr-<DATE>.json"
    let filename = format!("{}{}{}", prefix, now.format("%Y-%m-%d_%H%M%S"), ".json");
    let path = home::home_dir()
        .map(|path: PathBuf| {
            let path = path.join(".rnr");
            if !path.exists() {
                std::fs::create_dir_all(&path)?;
            }
            Ok::<PathBuf, std::io::Error>(path)
        })
        .unwrap_or(Ok(cwd))
        .unwrap()
        .join(filename);

    // Dump info to a file
    let file = match File::create(path.to_str().unwrap()) {
        Ok(file) => file,
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::CreateFile,
                value: Some(path.to_str().unwrap().to_string()),
            })
        }
    };
    match serde_json::to_writer_pretty(file, &dump) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error {
            kind: ErrorKind::JsonParse,
            value: Some(path.to_str().unwrap().to_string()),
        }),
    }
}

/// Read operations from a dump file and generate a Operations vector
pub fn read_from_file(filepath: &Path) -> Result<Operations> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::ReadFile,
                value: Some(filepath.to_string_lossy().to_string()),
            })
        }
    };
    let dump: DumpFormat = match serde_json::from_reader(file) {
        Ok(dump) => dump,
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::JsonParse,
                value: Some(filepath.to_string_lossy().to_string()),
            })
        }
    };
    Ok(dump.operations)
}

#[derive(Serialize, Deserialize)]
struct DumpFormat {
    date: String,
    directory: String,
    operations: Operations,
}
