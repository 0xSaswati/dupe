use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, BufReader};

pub fn hash_files(files: Vec<String>) -> Vec<(String, String)> {
    let mut results = Vec::new();
    for path in files {
        let file = File::open(&path).expect("Failed to open file");
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        
        loop {
            let n = reader.read(&mut buffer).expect("Read error");
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }
        
        let hash = format!("{:x}", hasher.finalize_reset());
        println!("Hashed file: {} -> {}", path, hash);  // Debugging line
        results.push((hash, path));
    }
    results
}
