use regex::Regex;
use walkdir::WalkDir;

pub fn extract_symbols(path: &str, platform: &str) -> Vec<String> {
    let mut symbols = vec![];
    let header_pattern = Regex::new(r#"\b(\w+)\s*[(:;{]"#).unwrap();

    for entry in WalkDir::new(path).into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "h").unwrap_or(false)) {
        if entry.path().to_string_lossy().contains(platform) {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                for cap in header_pattern.captures_iter(content.as_str()) {
                    if let Some(matched) = cap.get(1) {
                        symbols.push(matched.as_str().to_string());
                    }
                }
            }
        }
    }

    symbols
}