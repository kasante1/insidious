use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)] 
pub struct Route {
    pub path: String,
    pub method: String,
    pub file_path: String,
    pub line_number: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct RouteConflict {
    pub route1: Route,
    pub route2: Route,
    pub similarity: f64,
    pub conflict_type: String,
}

#[derive(Debug, Serialize)]
pub struct AnalysisReport {
    pub conflicts: Vec<RouteConflict>,
    pub total_routes: usize,
    pub conflict_count: usize,
    pub similarity_matrix: HashMap<String, f64>,
}