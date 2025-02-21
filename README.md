# Isidious - routes confilicts can really mess your day.

This is a command-line tool written in Rust.
It analyzes Express.js projects for route conflicts and generates similarity reports.
This tool helps developers identify potential routing issues and precedence issues.

## Features

- 🔍 Detects exact route matches and conflicts
- 📊 Calculates route similarity percentages
- 🗺️ Supports complex route parameters
- 📝 Generates detailed JSON reports
- 🚀 Fast and efficient analysis
- 📁 Recursive project directory scanning
- 💡 Smart conflict detection for parameterized routes

## Installation

### Prerequisites

- Rust (1.56.0 or later)
- Cargo package manager


## Conflict Types

The analyzer detects several types of conflicts:

1. **Exact Match**: Identical routes with the same HTTP method
2. **Parameter Conflict**: Routes with different parameter names but same structure
3. **High Similarity**: Routes with similarity percentage above 70%

## Similarity Calculation

Route similarity is calculated using a character-based diff algorithm that:
- Compares route paths character by character
- Accounts for parameter names in routes
- Generates a similarity percentage
- Considers route structure and length



## License

MIT License - see LICENSE file for details

## Author
dsasante1@gmail.com
