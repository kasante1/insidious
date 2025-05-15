use std::fs;
use regex::Regex;
use walkdir::WalkDir;
use crate::models::Route;
use crate::cli::AnalysisConfig;

pub fn extract_all_routes(config: &AnalysisConfig) -> Vec<Route> {
    let mut all_routes = Vec::new();
    
    for entry in WalkDir::new(&config.project_directory)
        .into_iter()
        .filter_entry(|e| {
            // Filter out hidden files/dirs
            if e.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false) {
                return false;
            }
            
            // Filter out excluded directories
            let path = e.path();
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                return !config.excluded_dirs.iter().any(|excluded| excluded == dir_name);
            }
            
            true
        })
    {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if let Some(ext_str) = ext.to_str() {
                    if config.file_extensions.iter().any(|e| ext_str == e) {
                        let file_path = entry.path().to_str().unwrap();
                        let routes = extract_routes_from_file(file_path);
                        all_routes.extend(routes);
                    }
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