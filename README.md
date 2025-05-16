# Insidious

> Routes conflicts and precedent issues can really mess your day up in the most subtle way

A powerful CLI tool that analyzes Express.js projects to detect route conflicts, calculate route similarities, and identify potential precedence issues that could affect your application's routing behavior.

## Features

- 🔍 Detects exact route matches and conflicts
- 📊 Calculates route similarity percentages
- 🗺️ Supports complex route parameters
- 📝 Generates detailed reports in multiple formats (JSON, HTML, Markdown)
- 🚀 Fast and efficient analysis
- 📁 Recursive project directory scanning
- 💡 Smart conflict detection for parameterized routes

## Installation

### Prerequisites

- Rust (1.56.0 or later)
- Cargo package manager

### Installing

```bash
# Clone the repository
git clone https://github.com/dsasante1/insidious.git
cd insidious

# Build and install
cargo install --path .
```

## Usage

Insidious provides three main commands:

### Analyze Command

Analyzes Express.js routes in a project directory.

```bash
insidious analyze <PROJECT_DIRECTORY> [OPTIONS]
```

**Options:**

- `-e, --exclude <DIRS>`: Directories to exclude from analysis (comma-separated)
  - Default: `node_modules`
  - Example: `--exclude node_modules,dist,build`

- `-x, --ext <EXTENSIONS>`: File extensions to analyze (comma-separated)
  - Default: `js`
  - Example: `--ext js,ts,jsx`

- `-s, --similarity-threshold <PERCENTAGE>`: Minimum similarity percentage to flag as a conflict
  - Default: `70`
  - Example: `--similarity-threshold 80`

**Example:**

```bash
insidious analyze ./my-express-app --exclude node_modules,dist,build --ext js,ts,jsx
```

This will analyze all routes in the specified project directory, excluding the directories mentioned, and will look for files with the specified extensions. The analysis will identify route conflicts and routes with similarity above the specified threshold.

### Report Command

Generates a report in a specific format (JSON, HTML, or Markdown).

```bash
insidious report <PROJECT_DIRECTORY> [OPTIONS]
```

**Options:**

- `-f, --format <FORMAT>`: Output format
  - Options: `json`, `html`, `markdown`
  - Default: `json`
  - Example: `--format html`

- `-o, --output <PATH>`: Output file path
  - Example: `--output ./reports/routes.html`

- `-e, --exclude <DIRS>`: Directories to exclude from analysis (comma-separated)
  - Default: `node_modules`
  - Example: `--exclude node_modules,dist,build`

- `-x, --ext <EXTENSIONS>`: File extensions to analyze (comma-separated)
  - Default: `js`
  - Example: `--ext js,ts,jsx`

**Examples:**

```bash
# Generate a JSON report
insidious report ./my-express-app --format json --output routes.json

# Generate an HTML report
insidious report ./my-express-app --format html --output routes.html

# Generate a Markdown report
insidious report ./my-express-app --format markdown --output routes.md
```

If no output path is provided, the report will be saved to:
- `route_analysis_report.json` (for JSON format)
- `route_analysis_report.html` (for HTML format)
- `route_analysis_report.md` (for Markdown format)

### Init Command

Initializes a configuration file with default settings.

```bash
insidious init
```

This will create a `.express-analyzer.json` configuration file in the current directory with default settings for excluded directories, file extensions, and similarity threshold.

## Conflict Types

The analyzer detects several types of conflicts:

1. **Exact Match**: Identical routes with the same HTTP method
2. **Parameter Conflict**: Routes with different parameter names but same structure
3. **High Similarity**: Routes with similarity percentage above the configured threshold (default: 70%)

## Similarity Calculation

Route similarity is calculated using a character-based diff algorithm that:
- Compares route paths character by character
- Accounts for parameter names in routes
- Generates a similarity percentage
- Considers route structure and length

## Configuration

You can create a configuration file using the `init` command, which will generate default settings. Alternatively, you can specify options directly through command-line arguments.

## License

MIT License - see LICENSE file for details

## Author

dsasante1@gmail.com