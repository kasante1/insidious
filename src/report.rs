use std::collections::HashMap;
use std::fs;
use crate::models::{Route, RouteConflict, AnalysisReport};
use crate::utils::calculate_path_similarity;

pub fn create_analysis_report(routes: Vec<Route>, conflicts: Vec<RouteConflict>) -> AnalysisReport {
    let mut similarity_matrix = HashMap::new();
    
    for route1 in routes.iter() {
        for route2 in routes.iter() {
            if route1.path != route2.path {
                let key = format!("{} <-> {}", route1.path, route2.path);
                let similarity = calculate_path_similarity(&route1.path, &route2.path);
                similarity_matrix.insert(key, similarity);
            }
        }
    }
    
    AnalysisReport {
        conflicts: conflicts.clone(),
        total_routes: routes.len(),
        conflict_count: conflicts.len(),
        similarity_matrix,
    }
}

pub fn print_report_summary(report: &AnalysisReport) {
    println!("\nRoute Analysis Report");
    println!("====================");
    println!("Total Routes: {}", report.total_routes);
    println!("Conflicts Found: {}", report.conflict_count);
    
    if !report.conflicts.is_empty() {
        println!("\nConflicts:");
        for conflict in &report.conflicts {
            println!("\nConflict Type: {}", conflict.conflict_type);
            println!("Similarity: {:.2}%", conflict.similarity);
            println!("Route 1: {} {} ({}:{})",
                conflict.route1.method,
                conflict.route1.path,
                conflict.route1.file_path,
                conflict.route1.line_number
            );
            println!("Route 2: {} {} ({}:{})",
                conflict.route2.method,
                conflict.route2.path,
                conflict.route2.file_path,
                conflict.route2.line_number
            );
        }
    }
}

pub fn save_json_report(report: &AnalysisReport, filename: &str) {
    let json = serde_json::to_string_pretty(&report).unwrap();
    fs::write(filename, json).unwrap();
    println!("\nDetailed report saved to {}", filename);
}