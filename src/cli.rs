use clap::{Parser, Subcommand, Args as ClapArgs};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "express-route-analyzer",
    author = "verbsgh <verbsgh@gmail.com>",
    version = "1.0.0",
    about = "Analyzes Express.js routes to detect conflicts and potential issues",
    long_about = None
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,


    #[arg(required_unless_present = "command")]
    pub project_directory: Option<String>,
    
    /// Directories to exclude from analysis (comma-separated)
    #[arg(short, long, value_delimiter = ',', default_value = "node_modules")]
    pub exclude: Vec<String>,
    
    /// File extensions to analyze (comma-separated)
    #[arg(short, long, value_delimiter = ',', default_value = "js")]
    pub extensions: Vec<String>,
    
    #[arg(short, long, default_value = "70")]
    pub similarity_threshold: f64,
}

#[derive(Subcommand)]
pub enum Commands {
    Analyze(AnalyzeArgs),
    
    Init,
    
    Report(ReportArgs),
}

#[derive(ClapArgs)]
pub struct AnalyzeArgs {

    pub project_directory: String,
    
    /// Directories to exclude from analysis (comma-separated)
    #[arg(short, long, value_delimiter = ',', default_value = "node_modules")]
    pub exclude: Vec<String>,
    
    /// File extensions to analyze (comma-separated)
    #[arg(short, long, value_delimiter = ',', default_value = "js")]
    pub extensions: Vec<String>,
    
    #[arg(short, long, default_value = "70")]
    pub similarity_threshold: f64,
}

#[derive(ClapArgs)]
pub struct ReportArgs {
    pub project_directory: String,
    
    /// Output format (json, html, markdown)
    #[arg(short, long, default_value = "json")]
    pub format: String,
    
    /// Output file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub struct AnalysisConfig {
    pub project_directory: String,
    pub excluded_dirs: Vec<String>,
    pub file_extensions: Vec<String>,
    pub similarity_threshold: f64,
}

pub fn get_args() -> CliArgs {
    CliArgs::parse()
}

/// Extract the analysis configuration from command line arguments
pub fn get_analysis_config(args: &CliArgs) -> AnalysisConfig {
    match &args.command {
        Some(Commands::Analyze(analyze_args)) => {
            AnalysisConfig {
                project_directory: analyze_args.project_directory.clone(),
                excluded_dirs: analyze_args.exclude.clone(),
                file_extensions: analyze_args.extensions.clone(),
                similarity_threshold: analyze_args.similarity_threshold,
            }
        },
        Some(Commands::Report(report_args)) => {

            AnalysisConfig {
                project_directory: report_args.project_directory.clone(),
                excluded_dirs: vec!["node_modules".to_string()],
                file_extensions: vec!["js".to_string()],
                similarity_threshold: 70.0,
            }
        },

        _ => {
            AnalysisConfig {
                project_directory: args.project_directory.clone().unwrap_or_default(),
                excluded_dirs: args.exclude.clone(),
                file_extensions: args.extensions.clone(),
                similarity_threshold: args.similarity_threshold,
            }
        }
    }
}