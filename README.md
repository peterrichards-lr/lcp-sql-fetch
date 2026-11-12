# Liferay Cloud SQL Fetch (`lcp-sql-fetch`)

[![GitHub Release](https://img.shields.io/github/v/release/peterrichards-lr/lcp-sql-fetch)](https://github.com/peterrichards-lr/lcp-sql-fetch/releases)
[![License](https://img.shields.io/github/license/peterrichards-lr/lcp-sql-fetch)](LICENSE)
[![Rust CI](https://github.com/peterrichards-lr/lcp-sql-fetch/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/lcp-sql-fetch/actions/workflows/rust.yml)

A Rust CLI that runs a local SQL script against the database of a Liferay Cloud (LCP) environment and downloads the result as a file. It finds a running service instance, uploads the script through `lcp shell`, runs it with `psql` or `mysql`, downloads the output with `lcp files download`, and cleans up the remote files afterwards.

## Features

- **Safe:** Fails fast if you aren't authenticated with `lcp` or the project doesn't exist. Scripts containing data-modifying keywords (`UPDATE`, `DELETE`, `DROP`, `TRUNCATE`, `ALTER`, `GRANT`, `REVOKE`, `INSERT`) need confirmation unless `--force` is used.
- **Smart:** Finds the first running instance of the service and detects whether your project ID already contains an environment suffix.
- **Secure:** Prompts for the database password with hidden input instead of keeping it in your shell history (unless passed via flag or environment variable).
- **Automated:** Uses `expect` to drive the interactive `lcp shell`, so execution is hands-free.
- **PostgreSQL and MySQL:** Choose the database client with `--database-type`.

## Prerequisites

- The Liferay Cloud CLI (`lcp`), logged in with `lcp login`.
- `expect`, available on your `PATH`. It ships with macOS and most Linux distributions; on Debian/Ubuntu install it with `sudo apt install expect`. The tool exits with an error if `expect` is missing, so on Windows run it from WSL.
- The database password for the user you connect as.

## Installation

### Homebrew (macOS / Linux)

```bash
brew tap peterrichards-lr/tap
brew install lcp-sql-fetch
```

The formula builds from source, so Homebrew installs Rust as a build dependency.

### Scoop (Windows)

```powershell
scoop bucket add peterrichards-lr https://github.com/peterrichards-lr/scoop-bucket
scoop install lcp-sql-fetch
```

The manifest builds from source and depends on `rustup`. You also need `expect` on your `PATH` (see [Prerequisites](#prerequisites)); WSL is the simpler option on Windows.

### Windows Subsystem for Linux (WSL)

Install [Homebrew on Linux](https://docs.brew.sh/Homebrew-on-Linux) inside your WSL distribution, make sure `expect` is installed, then use the Homebrew commands above:

```bash
# Ubuntu/Debian example
sudo apt update && sudo apt install expect -y
brew tap peterrichards-lr/tap
brew install lcp-sql-fetch
```

### Manual Download

Download the pre-compiled binary for your OS from the [GitHub Releases](https://github.com/peterrichards-lr/lcp-sql-fetch/releases) page.

### Build from Source

If you have Rust installed:

```bash
cargo install --path .
```

## Usage

```bash
lcp-sql-fetch --help
lcp-sql-fetch fetch --help
```

The project ID can be given in several formats:

```bash
# Option 1: Split project and environment
lcp-sql-fetch fetch -p my-project -e dev -f query.sql -o results.txt

# Option 2: Full LCP project ID (any project ID containing a hyphen is used as-is)
lcp-sql-fetch fetch -p my-project-dev -f query.sql

# Option 3: Interactive (prompts for the environment if missing)
lcp-sql-fetch fetch -p myproject -f query.sql
```

### Options

Global options:

- `-v, --verbose`: Show verbose output from internal processes (`lcp shell`, `expect`, etc.)
- `-h, --help`: Print help
- `-V, --version`: Print version

`fetch` options:

- `-p, --project <PROJECT>`: The project ID (e.g., `lfrprj`) or full ID (e.g., `lfrprj-dev`)
- `-e, --environment <ENVIRONMENT>`: The environment (e.g., `dev`, `uat`, `prd`)
- `-s, --service <SERVICE>`: The service name [default: `liferay`]
- `-f, --file <FILE>`: Path to the local `.sql` script to execute
- `-o, --output <OUTPUT>`: Local path for the result file [default: `output.txt`]
- `-u, --user <USER>`: Database username [default: the project-environment ID, e.g. `lfrprj-dev`, which is the read-only user]
- `-P, --password <PASSWORD>`: Database password (optional, prompted securely if missing) [env: `LCP_DB_PASSWORD`]
- `-d, --database-type <DATABASE_TYPE>`: Database type, `psql` or `mysql` [default: `psql`]
- `--force`: Skip the confirmation for scripts containing data-modifying SQL keywords

### Password Management

You can provide the database password in three ways (in order of precedence):

1. **Command Line Argument:** Use `-P` or `--password`.
2. **Environment Variable:** Set the `LCP_DB_PASSWORD` variable.
3. **Interactive Prompt:** If neither of the above is provided, the tool prompts you securely.

### Output

The script's combined output (including database errors) is written to the `--output` file. If the SQL fails, the tool still downloads the output so you can read the database error, then exits with an error. An empty output file usually means the query returned no results.

### Examples

The `examples/` directory contains PostgreSQL scripts:

- `user_security_audit.sql`: Complex join across 4 tables for user/site/role reporting.
- `storage_audit.sql`: Identifies the top 10 largest files in the document library.
- `list_tables.sql`: Lists all tables in the `public` schema.
- `list_columns.sql`: Lists the columns of a table (edit the table name first).
- `describe_tables.sql`: Lists the columns of the tables used by the audit scripts.

```bash
lcp-sql-fetch fetch -p lfrprj -e uat -f examples/storage_audit.sql -o storage.txt
```

## Releasing

1. Bump `version` in `Cargo.toml` and run `cargo build` to update `Cargo.lock`.
2. Move the `[Unreleased]` entries in `CHANGELOG.md` into a new `## [x.y.z] - YYYY-MM-DD` section.
3. Commit, then create an annotated tag and push it:
   ```bash
   python3 scripts/release.py --tag vX.Y.Z
   git push origin vX.Y.Z
   ```
   The release workflow fails if the tag isn't annotated, doesn't match `Cargo.toml`, or has no `CHANGELOG.md` section.
4. The Homebrew tap and Scoop bucket pick up the new release automatically within a couple of hours. `.gemini/prompts/update-distribution-channels.md` is only needed for a tool's first release.

## Disclaimer

**This tool is provided "as-is", without warranty of any kind. Executing SQL directly on a Liferay Cloud database, especially in production, can be dangerous. The keyword check is a basic safeguard, not a guarantee: you are responsible for the queries you run and for any data they change. Use with caution.**

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
