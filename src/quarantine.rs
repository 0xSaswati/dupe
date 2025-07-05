use std::fs;
use std::path::Path;
use crate::report::Report;

pub fn clean_duplicates(report: &Report) {
    let quarantine_dir = Path::new("quarantine");
    fs::create_dir_all(&quarantine_dir).expect("Could not create quarantine folder");

    println!("Found {} duplicate groups.", report.duplicates.len());  // Debugging line

    for group in &report.duplicates {
        if let Some((_, rest)) = group.split_first() {
            for file in rest {
                let filename = Path::new(file).file_name().unwrap();
                let dest = quarantine_dir.join(filename);
                println!("Moving file {} to quarantine.", file);  // Debugging line
                fs::rename(file, dest).expect("Failed to move file");
            }
        }
    }
}