use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "xclense", about = "Analyze Kotlin XCFramework usage in Swift")]
pub struct Cli {
    #[arg(long)]
    pub swift_path: String,

    #[arg(long)]
    pub xcframework_path: String,

    #[arg(long)]
    pub platform: String,

    #[arg(long, default_value = "json")]
    pub report_format: String,
}
