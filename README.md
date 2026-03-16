# Liferay Cloud SQL Fetch Tool (`lcp-sql-fetch`)

[![Rust CI](https://github.com/peterrichards-lr/lcp-sql-fetch/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/lcp-sql-fetch/actions/workflows/rust.yml)
[![Latest Release](https://img.shields.io/github/v/tag/peterrichards-lr/lcp-sql-fetch?label=version)](https://github.com/peterrichards-lr/lcp-sql-fetch/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, cross-platform Rust CLI utility to securely execute local SQL scripts on your Liferay Cloud (LCP) environments and download the results as a file. It automates finding instance IDs, uploading scripts, and executing commands inside the `lcp shell`.

## Features

- **Safe:** Fails fast if you aren't authenticated with `lcp`. Includes built-in protection against destructive SQL (UPDATE, DELETE, DROP) unless the `--force` flag is used.
- **Smart:** Automatically discovers running Liferay instances and intelligently detects if your project ID already contains an environment suffix.
- **Secure:** Uses hidden prompts for your database password instead of keeping it in your shell history (unless passed via flag or environment variable).
- **Automated:** Uses `expect` to "type" commands into the interactive `lcp shell` for 100% hands-free execution on macOS/Linux.
- **Cross-Platform:** Native binaries for macOS (Intel/ARM), Linux, and Windows.

## Installation

### macOS / Linux (Homebrew)

```bash
brew tap peterrichards-lr/homebrew-tap
brew install lcp-sql-fetch
```

_Note: For full automation, ensure `expect` is installed (default on most macOS/Linux systems)._

### Windows (Scoop)

```powershell
scoop bucket add peterrichards-lr https://github.com/peterrichards-lr/scoop-bucket
scoop install lcp-sql-fetch
```

### Windows Subsystem for Linux (WSL)

The tool works perfectly in WSL! For the best experience (100% automation), ensure `expect` is installed in your WSL distribution:

```bash
# Ubuntu/Debian example
sudo apt update && sudo apt install expect -y
```

### Manual Download

Download the pre-compiled executable for your OS from the [GitHub Releases](https://github.com/peterrichards-lr/lcp-sql-fetch/releases) page.

### From Source

If you have Rust installed, you can build from source:

```bash
cargo install --path .
```

## Usage

The tool is smart enough to handle project IDs in multiple formats and will guide you through the process:

```bash
# Option 1: Split project and environment
lcp-sql-fetch fetch -p my-project -e dev -f query.sql -o results.txt

# Option 2: Full LCP project ID
lcp-sql-fetch fetch -p my-project-dev -f query.sql

# Option 3: Interactive (will prompt for environment if missing)
lcp-sql-fetch fetch -p my-project -f query.sql
```

### Options:

- `-p, --project <PROJECT>`: The project ID (e.g., `lfrprj`) or full ID (e.g., `lfrprj-dev`)
- `-e, --environment <ENVIRONMENT>`: The environment (e.g., `dev`, `uat`, `prd`)
- `-s, --service <SERVICE>`: The service name [default: `liferay`]
- `-f, --file <FILE>`: Path to the local `.sql` script to execute
- `-o, --output <OUTPUT>`: Local path for the result file [default: `output.txt`]
- `-u, --user <USER>`: Database username (defaults to project ID for read-only access)
- `-P, --password <PASSWORD>`: Database password (optional, prompted securely if missing)
- `-d, --database-type <TYPE>`: Database type (`psql` or `mysql`) [default: `psql`]
- `--force`: Bypass safety warning for destructive SQL statements

## Password Management

You can provide the database password in three ways (in order of precedence):

1. **Command Line Argument:** Use `-P` or `--password`.
2. **Environment Variable:** Set the `LCP_DB_PASSWORD` variable.
3. **Interactive Prompt:** If neither of the above is provided, the tool will prompt you securely.

## Examples

Check the `examples/` directory for useful scripts:

- `user_security_audit.sql`: Complex join across 4 tables for user/site/role reporting.
- `storage_audit.sql`: Identifies top 10 largest files in the document library.
- `list_tables.sql`: Lists all tables in the `public` schema.

## Disclaimer

Executing SQL directly on a production database can be dangerous. While this tool includes basic safety checks, you are responsible for the queries you run. Use with caution.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
