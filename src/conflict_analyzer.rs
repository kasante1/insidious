use crate::models::{Route, RouteConflict};
use crate::utils::calculate_path_similarity;
use crate::cli::AnalysisConfig;

pub fn check_route_conflicts(routes: &[Route], config: &AnalysisConfig) -> Vec<RouteConflict> {
    let mut conflicts = Vec::new();
    
    for (i, route1) in routes.iter().enumerate() {
        for route2 in routes.iter().skip(i + 1) {
            if route1.method != route2.method {
                continue;
            }
            
            let similarity = calculate_path_similarity(&route1.path, &route2.path);
            
            if route1.path == route2.path {
                conflicts.push(create_conflict(
                    route1.clone(), 
                    route2.clone(), 
                    100.0, 
                    "Exact Match"
                ));
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
                
                if potential_conflict && similarity > config.similarity_threshold {
                    conflicts.push(create_conflict(
                        route1.clone(), 
                        route2.clone(), 
                        similarity, 
                        "Parameter Conflict"
                    ));
                }
            }
        }
    }
    
    conflicts
}

fn create_conflict(route1: Route, route2: Route, similarity: f64, conflict_type: &str) -> RouteConflict {
    RouteConflict {
        route1,
        route2,
        similarity,
        conflict_type: conflict_type.to_string(),
    }
}