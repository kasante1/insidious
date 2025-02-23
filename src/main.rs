use std::fs;
use std::path::Path;
use regex::Regex;
use serde::{Serialize};
use walkdir::WalkDir;
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone)] 
struct Route {
    path: String,
    method: String,
    file_path: String,
    line_number: usize,
}

#[derive(Debug, Serialize, Clone)]
struct RouteConflict {
    route1: Route,
    route2: Route,
    similarity: f64,
    conflict_type: String,
}

#[derive(Debug, Serialize)]
struct AnalysisReport {
    conflicts: Vec<RouteConflict>,
    total_routes: usize,
    conflict_count: usize,
    similarity_matrix: HashMap<String, f64>,
}

fn extract_routes(file_path: &str) -> Vec<Route> {
    let content = fs::read_to_string(file_path).unwrap_or_default();
    let mut routes = Vec::new();
    
    let app_route_regex = Regex::new(r"app\.(get|post|put|delete|patch)\(['\](.*?)['\]\s*,").unwrap();
    let router_route_regex = Regex::new(r"router\.(get|post|put|delete|patch)\(['\](.*?)['\]\s*,").unwrap();
    
    for (line_number, line) in content.lines().enumerate() {
        for cap in app_route_regex.captures_iter(line) {
            routes.push(Route {
                method: cap[1].to_uppercase(),
                path: cap[2].to_string(),
                file_path: file_path.to_string(),
                line_number: line_number + 1,
            });
        }
        
        // Check for router routes
        for cap in router_route_regex.captures_iter(line) {
            routes.push(Route {
                method: cap[1].to_uppercase(),
                path: cap[2].to_string(),
                file_path: file_path.to_string(),
                line_number: line_number + 1,
            });
        }
    }
    
    routes
}

fn calculate_path_similarity(path1: &str, path2: &str) -> f64 {
    let diff = TextDiff::from_chars(path1, path2);
    let matching_chars: usize = diff
        .iter_all_changes()
        .filter(|change| change.tag() == ChangeTag::Equal)
        .map(|change| change.to_string().len())
        .sum();
    
    let max_len = path1.len().max(path2.len());
    if max_len == 0 {
        return 0.0;
    }
    
    (matching_chars as f64 / max_len as f64) * 100.0
}

fn check_route_conflicts(routes: &[Route]) -> Vec<RouteConflict> {
    let mut conflicts = Vec::new();
    
    for (i, route1) in routes.iter().enumerate() {
        for route2 in routes.iter().skip(i + 1) {
            if route1.method != route2.method {
                continue;
            }
            
            let similarity = calculate_path_similarity(&route1.path, &route2.path);
            
            if route1.path == route2.path {
                conflicts.push(RouteConflict {
                    route1: Route {
                        path: route1.path.clone(),
                        method: route1.method.clone(),
                        file_path: route1.file_path.clone(),
                        line_number: route1.line_number,
                    },
                    route2: Route {
                        path: route2.path.clone(),
                        method: route2.method.clone(),
                        file_path: route2.file_path.clone(),
                        line_number: route2.line_number,
                    },
                    similarity: 100.0,
                    conflict_type: "Exact Match".to_string(),
                });
                continue;
            }
            
            let path1_parts: Vec<&str> = route1.path.split('/').collect();
            let path2_parts: Vec<&str> = route2.path.split('/').collect();
            
            if path1_parts.len() == path2_parts.len() {
                let mut potential_conflict = false;
                for (p1, p2) in path1_parts.iter().zip(path2_parts.iter()) {
                    if (p1.starts_with(':') || p2.starts_with(':')) && p1 != p2 {
                        potential_conflict = true;
                        break;
                    }
                }
                
                if potential_conflict && similarity > 70.0 {
                    conflicts.push(RouteConflict {
                        route1: Route {
                            path: route1.path.clone(),
                            method: route1.method.clone(),
                            file_path: route1.file_path.clone(),
                            line_number: route1.line_number,
                        },
                        route2: Route {
                            path: route2.path.clone(),
                            method: route2.method.clone(),
                            file_path: route2.file_path.clone(),
                            line_number: route2.line_number,
                        },
                        similarity,
                        conflict_type: "Parameter Conflict".to_string(),
                    });
                }
            }
        }
    }
    
    conflicts
}

fn generate_report(project_path: &str) -> AnalysisReport {
    let mut all_routes = Vec::new();
    
    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false))
    {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if ext == "js" {
                    let file_path = entry.path().to_str().unwrap();
                    let routes = extract_routes(file_path);
                    all_routes.extend(routes);
                }
            }
        }
    }
    
    let conflicts = check_route_conflicts(&all_routes);
    
    let mut similarity_matrix = HashMap::new();
    for route1 in all_routes.iter() {
        for route2 in all_routes.iter() {
            if route1.path != route2.path {
                let key = format!("{} <-> {}", route1.path, route2.path);
                let similarity = calculate_path_similarity(&route1.path, &route2.path);
                similarity_matrix.insert(key, similarity);
            }
        }
    }
    
    AnalysisReport {
        conflicts: conflicts.clone(),
        total_routes: all_routes.len(),
        conflict_count: conflicts.len(),
        similarity_matrix,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <express-project-path>", args[0]);
        return;
    }
    
    let project_path = &args[1];
    if !Path::new(project_path).exists() {
        println!("Error: Project path does not exist");
        return;
    }
    
    println!("Analyzing Express.js routes in: {}", project_path);
    let report = generate_report(project_path);
    

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
    
    // Save report to JSON file
    let json = serde_json::to_string_pretty(&report).unwrap();
    fs::write("route_analysis_report.json", json).unwrap();
    println!("\nDetailed report saved to route_analysis_report.json");
}