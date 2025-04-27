use analyzer::analyze_project;
use clap::Parser;
use cli::Cli;
use report::generate_report;

mod cli;
mod analyzer;
mod report;
mod swift;
mod xcframework;

fn main() {
    let cli_args = Cli::parse();
    let analysis_result = analyze_project(
        &cli_args.swift_path,
        &cli_args.xcframework_path,
        &cli_args.platform,
    );

    generate_report(&analysis_result, &cli_args.report_format);
}