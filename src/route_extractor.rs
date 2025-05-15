use std::fs;
use regex::Regex;
use walkdir::WalkDir;
use crate::models::Route;

pub fn extract_all_routes(project_path: &str) -> Vec<Route> {
    let mut all_routes = Vec::new();
    
    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false))
    {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if ext == "js" {
                    let file_path = entry.path().to_str().unwrap();
                    let routes = extract_routes_from_file(file_path);
                    all_routes.extend(routes);
                }
            }
        }
    }
    
    all_routes
}

fn extract_routes_from_file(file_path: &str) -> Vec<Route> {
    let content = fs::read_to_string(file_path).unwrap_or_default();
    let mut routes = Vec::new();
    
    let app_route_regex = Regex::new(r#"app\.(get|post|put|delete|patch)\(['"](.+?)['"],?"#).unwrap();
    let router_route_regex = Regex::new(r#"router\.(get|post|put|delete|patch)\(['"](.+?)['"],?"#).unwrap();
    
    for (line_number, line) in content.lines().enumerate() {
        for cap in app_route_regex.captures_iter(line) {
            routes.push(Route {
                method: cap[1].to_uppercase(),
                path: cap[2].to_string(),
                file_path: file_path.to_string(),
                line_number: line_number + 1,
            });
        }
        
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