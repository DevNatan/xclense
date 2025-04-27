use crate::analyzer::AnalysisResult;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Report<'a> {
    used_symbols: &'a Vec<String>,
    unused_symbols: &'a Vec<String>,
    total_exported: usize,
}

pub fn generate_report(result: &AnalysisResult, format: &str) {
    match format {
        "json" => {
            let report = Report {
                used_symbols: &result.used,
                unused_symbols: &result.unused,
                total_exported: result.exported.len(),
            };

            let json = serde_json::to_string_pretty(&report).unwrap();
            let mut file = File::create("xclense-report.json").unwrap();
            file.write_all(json.as_bytes()).unwrap();
            println!("✅ JSON report written to xclense-report.json");
        },
        _ => {
            println!("\n🧼 xclense summary:");
            println!("Total exported symbols: {}", result.exported.len());
            println!("Used: {}", result.used.len());
            println!("Unused: {}", result.unused.len());
            for symbol in &result.unused {
                println!("  ⚠️  Unused: {symbol}");
            }
        }
    }
}
