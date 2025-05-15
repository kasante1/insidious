mod cli;
mod models;
mod route_extractor;
mod conflict_analyzer;
mod report;
mod utils;
mod config;

fn main() {
    let args = cli::get_args();
    
    match &args.command {
        cli::Commands::Init => {
            // Initialize configuration file
            config::init_config();
            println!("Configuration file created: .express-analyzer.json");
        },
        
        cli::Commands::Report { 
            project_directory, 
            format, 
            output,
            .. // We don't need to extract the other parameters here as they're handled in get_analysis_config
        } => {
            // Validate project directory
            if let Err(err) = cli::validate_project_path(project_directory) {
                println!("{}", err);
                return;
            }
            
            // Get analysis configuration
            let config = cli::get_analysis_config(&args);
            
            println!("Analyzing Express.js routes in: {}", project_directory);
            println!("Generating {} report...", format);
            
            // Extract routes and analyze conflicts
            let routes = route_extractor::extract_all_routes(&config);
            let conflicts = conflict_analyzer::check_route_conflicts(&routes, &config);
            let report = report::create_analysis_report(routes, conflicts);
            
            // Generate report in the specified format
            match format.as_str() {
                "json" => report::save_json_report(&report, output.as_deref()),
                "html" => report::save_html_report(&report, output.as_deref()),
                "markdown" => report::save_markdown_report(&report, output.as_deref()),
                _ => {
                    println!("Unsupported format: {}. Using JSON format instead.", format);
                    report::save_json_report(&report, output.as_deref());
                }
            }
            
            println!("Report generation complete.");
        },
        
        cli::Commands::Analyze { 
            project_directory,
            exclude,
            extensions,
            similarity_threshold
        } => {
            // Validate project directory
            if let Err(err) = cli::validate_project_path(project_directory) {
                println!("{}", err);
                return;
            }
            
            println!("Analyzing Express.js routes in: {}", project_directory);
            println!("Excluded directories: {:?}", exclude);
            println!("File extensions: {:?}", extensions);
            println!("Similarity threshold: {}%", similarity_threshold);
            
            // Get analysis configuration
            let config = cli::get_analysis_config(&args);
            
            // Extract routes and analyze conflicts
            let routes = route_extractor::extract_all_routes(&config);
            println!("Found {} routes in project", routes.len());
            
            let conflicts = conflict_analyzer::check_route_conflicts(&routes, &config);
            let report = report::create_analysis_report(routes, conflicts);
            
            // Print summary report
            report::print_report_summary(&report);
            
            // Save detailed JSON report
            report::save_json_report(&report, None);
            println!("Detailed report saved to route_analysis_report.json");
        }
    }
}