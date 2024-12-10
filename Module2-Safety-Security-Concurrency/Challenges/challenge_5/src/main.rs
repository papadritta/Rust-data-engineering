use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Result};
use std::path::Path;

fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha3_256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);
    Ok(format!("{:x}", hasher.finalize()))
}

fn main() -> Result<()> {
    let mut hashes = HashMap::new();

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let hash = hash_file(&path)?;
            hashes.entry(hash).or_insert_with(Vec::new).push(path);
        }
    }

    for (hash, paths) in hashes {
        if paths.len() > 1 {
            println!("Duplicate files with hash {}: {:?}", hash, paths);
        }
    }

    Ok(())
}
