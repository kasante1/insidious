use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
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

pub fn save_json_report(report: &AnalysisReport, output_path: Option<&Path>) {
    let json = serde_json::to_string_pretty(report).unwrap();
    let path = output_path.unwrap_or_else(|| Path::new("route_analysis_report.json"));
    
    fs::write(path, json).unwrap();
    println!("\nDetailed JSON report saved to {}", path.display());
}

pub fn save_html_report(report: &AnalysisReport, output_path: Option<&Path>) {
    let path = output_path.unwrap_or_else(|| Path::new("route_analysis_report.html"));
    
    // HTML report template
    let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <title>Express.js Route Analysis Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        tr:nth-child(even) { background-color: #f9f9f9; }
        .conflict { background-color: #ffecec; }
    </style>
</head>
<body>
    <h1>Express.js Route Analysis Report</h1>
    <p>Total Routes: "#);
    
    html.push_str(&report.total_routes.to_string());
    html.push_str("</p>\n<p>Conflicts Found: ");
    html.push_str(&report.conflict_count.to_string());
    html.push_str("</p>\n");
    
    if !report.conflicts.is_empty() {
        html.push_str(r#"<h2>Conflicts</h2>
    <table>
        <tr>
            <th>Conflict Type</th>
            <th>Similarity</th>
            <th>Route 1</th>
            <th>Location 1</th>
            <th>Route 2</th>
            <th>Location 2</th>
        </tr>
"#);
        
        for conflict in &report.conflicts {
            html.push_str("<tr class=\"conflict\">\n");
            html.push_str(&format!("<td>{}</td>\n", conflict.conflict_type));
            html.push_str(&format!("<td>{:.2}%</td>\n", conflict.similarity));
            html.push_str(&format!("<td>{} {}</td>\n", conflict.route1.method, conflict.route1.path));
            html.push_str(&format!("<td>{}:{}</td>\n", conflict.route1.file_path, conflict.route1.line_number));
            html.push_str(&format!("<td>{} {}</td>\n", conflict.route2.method, conflict.route2.path));
            html.push_str(&format!("<td>{}:{}</td>\n", conflict.route2.file_path, conflict.route2.line_number));
            html.push_str("</tr>\n");
        }
        
        html.push_str("</table>\n");
    }
    
    html.push_str("</body>\n</html>");
    
    fs::write(path, html).unwrap();
    println!("\nDetailed HTML report saved to {}", path.display());
}

pub fn save_markdown_report(report: &AnalysisReport, output_path: Option<&Path>) {
    let path = output_path.unwrap_or_else(|| Path::new("route_analysis_report.md"));
    
    let mut markdown = String::from("# Express.js Route Analysis Report\n\n");
    markdown.push_str(&format!("- **Total Routes:** {}\n", report.total_routes));
    markdown.push_str(&format!("- **Conflicts Found:** {}\n\n", report.conflict_count));
    
    if !report.conflicts.is_empty() {
        markdown.push_str("## Conflicts\n\n");
        markdown.push_str("| Conflict Type | Similarity | Route 1 | Location 1 | Route 2 | Location 2 |\n");
        markdown.push_str("|--------------|------------|---------|------------|---------|------------|\n");
        
        for conflict in &report.conflicts {
            markdown.push_str(&format!(
                "| {} | {:.2}% | {} {} | {}:{} | {} {} | {}:{} |\n",
                conflict.conflict_type,
                conflict.similarity,
                conflict.route1.method, conflict.route1.path,
                conflict.route1.file_path, conflict.route1.line_number,
                conflict.route2.method, conflict.route2.path,
                conflict.route2.file_path, conflict.route2.line_number
            ));
        }
    }
    
    fs::write(path, markdown).unwrap();
    println!("\nDetailed Markdown report saved to {}", path.display());
}