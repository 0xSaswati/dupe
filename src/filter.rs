use regex::Regex;

pub fn apply_filter(files: Vec<String>, pattern: Option<&str>) -> Vec<String> {
    if let Some(pat) = pattern {
        let re = Regex::new(pat).expect("Invalid regex pattern");
        files.into_iter().filter(|f| re.is_match(f)).collect()
    } else {
        files
    }
}
