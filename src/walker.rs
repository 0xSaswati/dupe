use walkdir::WalkDir;

pub fn scan_dir(root: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            files.push(entry.path().display().to_string());
        }
    }
    files
}
