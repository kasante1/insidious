use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "express-route-analyzer",
    author = "verbsgh <verbsgh@gmail.com>",
    version = "1.0.0",
    about = "Analyzes Express.js routes to detect conflicts and potential issues",
    long_about = None
)]
pub struct Args {
    /// Path to the Express.js project directory
    pub project_directory: String
}

pub fn get_args() -> Args {
    Args::parse()
}