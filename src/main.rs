mod cli;
mod models;
mod route_extractor;
mod conflict_analyzer;
mod report;
mod utils;

use std::path::Path;

fn main() {
    let args = cli::get_args();
    
    let project_path = &args.project_directory;
    if !Path::new(project_path).exists() {
        println!("Error: Project path does not exist");
        return;
    }
    
    println!("Analyzing Express.js routes in: {}", project_path);
    
    let routes = route_extractor::extract_all_routes(project_path);
    let conflicts = conflict_analyzer::check_route_conflicts(&routes);
    let report = report::create_analysis_report(routes, conflicts);
    
    report::print_report_summary(&report);
    report::save_json_report(&report, "route_analysis_report.json");
}