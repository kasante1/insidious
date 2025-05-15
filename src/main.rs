mod cli;
mod models;
mod route_extractor;
mod conflict_analyzer;
mod report;
mod utils;
mod config;

use std::path::Path;
use cli::Commands;

fn main() {
    let args = cli::get_args();
    
    // Handle subcommands
    match &args.command {
        Some(Commands::Init) => {
            // Initialize configuration file
            config::init_config();
            println!("Configuration file created: .express-analyzer.json");
            return;
        }
        Some(Commands::Report(report_args)) => {
            // Generate report with specified format
            let analysis_config = cli::get_analysis_config(&args);
            
            if !Path::new(&analysis_config.project_directory).exists() {
                println!("Error: Project path does not exist");
                return;
            }
            
            let routes = route_extractor::extract_all_routes(&analysis_config);
            let conflicts = conflict_analyzer::check_route_conflicts(&routes, &analysis_config);
            let analysis_report = report::create_analysis_report(routes, conflicts);
            
            match report_args.format.as_str() {
                "json" => report::save_json_report(&analysis_report, report_args.output.as_deref()),
                "html" => report::save_html_report(&analysis_report, report_args.output.as_deref()),
                "markdown" => report::save_markdown_report(&analysis_report, report_args.output.as_deref()),
                _ => {
                    println!("Unsupported format: {}. Using JSON format instead.", report_args.format);
                    report::save_json_report(&analysis_report, report_args.output.as_deref());
                }
            }
            return;
        }
        Some(Commands::Analyze(_)) | None => {
            // Standard analysis flow
            let analysis_config = cli::get_analysis_config(&args);
            
            if !Path::new(&analysis_config.project_directory).exists() {
                println!("Error: Project path does not exist");
                return;
            }
            
            println!("Analyzing Express.js routes in: {}", analysis_config.project_directory);
            println!("Excluded directories: {:?}", analysis_config.excluded_dirs);
            
            let routes = route_extractor::extract_all_routes(&analysis_config);
            let conflicts = conflict_analyzer::check_route_conflicts(&routes, &analysis_config);
            let report = report::create_analysis_report(routes, conflicts);
            
            report::print_report_summary(&report);
            report::save_json_report(&report, None);
        }
    }
}