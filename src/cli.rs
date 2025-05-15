use clap::{Parser, Subcommand};
use std::path::Path;

/// Configuration for route extraction
pub struct AnalysisConfig {
    pub project_directory: String,
    pub excluded_dirs: Vec<String>,
    pub file_extensions: Vec<String>,
    pub similarity_threshold: f64,
}

#[derive(Parser)]
#[command(
    name = "express-route-analyzer",
    author = "verbsgh@gmail.com",
    version,
    about = "Analyzes Express.js routes in Express.js projects to detect routes conflicts",
    long_about = None
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze Express.js routes in a project
    Analyze {
        /// Path to the Express.js project directory
        project_directory: String,
        
        /// Directories to exclude from analysis (comma-separated)
        #[arg(short, long, value_delimiter = ',', default_value = "node_modules")]
        exclude: Vec<String>,
        
        /// File extensions to analyze (comma-separated)
        #[arg(short, long, value_delimiter = ',', default_value = "js")]
        extensions: Vec<String>,
        
        /// Minimum similarity percentage to flag as a conflict
        #[arg(short, long, default_value = "70")]
        similarity_threshold: f64,
    },
    
    /// Initialize a configuration file
    Init,
    
    /// Generate a report in a specific format
    Report {
        /// Path to the Express.js project directory
        project_directory: String,
        
        /// Output format (json, html, markdown)
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        
        /// Directories to exclude from analysis (comma-separated)
        #[arg(short, long, value_delimiter = ',', default_value = "node_modules")]
        exclude: Vec<String>,
        
        /// File extensions to analyze (comma-separated)
        #[arg(short, long, value_delimiter = ',', default_value = "js")]
        extensions: Vec<String>,
    },
}

pub fn get_args() -> CliArgs {
    CliArgs::parse()
}

/// Create an analysis configuration from command arguments
pub fn create_analysis_config_from_analyze(
    project_directory: &str,
    exclude: &[String],
    extensions: &[String],
    similarity_threshold: f64,
) -> AnalysisConfig {
    AnalysisConfig {
        project_directory: project_directory.to_string(),
        excluded_dirs: exclude.to_vec(),
        file_extensions: extensions.to_vec(),
        similarity_threshold,
    }
}

/// Create an analysis configuration from report arguments
pub fn create_analysis_config_from_report(
    project_directory: &str,
    exclude: &[String],
    extensions: &[String],
) -> AnalysisConfig {
    AnalysisConfig {
        project_directory: project_directory.to_string(),
        excluded_dirs: exclude.to_vec(),
        file_extensions: extensions.to_vec(),
        similarity_threshold: 70.0, // Default for reports
    }
}

/// Extract the analysis configuration from command line arguments
pub fn get_analysis_config(args: &CliArgs) -> AnalysisConfig {
    match &args.command {
        Commands::Analyze { 
            project_directory, 
            exclude, 
            extensions, 
            similarity_threshold 
        } => {
            create_analysis_config_from_analyze(
                project_directory,
                exclude,
                extensions,
                *similarity_threshold,
            )
        },
        Commands::Report { 
            project_directory, 
            exclude,
            extensions,
            .. 
        } => {
            create_analysis_config_from_report(
                project_directory,
                exclude,
                extensions,
            )
        },
        Commands::Init => {
            panic!("Analysis config should not be needed for Init command");
        }
    }
}

/// Validate that the project directory exists
pub fn validate_project_path(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("Error: Project path '{}' does not exist", path));
    }
    Ok(())
}