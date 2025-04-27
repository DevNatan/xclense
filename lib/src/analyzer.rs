use std::vec;

pub struct AnalysisResult {
    pub used: Vec<String>,
    pub exported: Vec<String>,
    pub unused: Vec<String>,
}

pub fn analyze_project(
    swift_path: &str,
    xcframework_path: &str,
    platform: &str
) -> AnalysisResult {
    // TODO
    AnalysisResult {
        used: vec![],
        exported: vec![],
        unused: vec![]
    }
}
