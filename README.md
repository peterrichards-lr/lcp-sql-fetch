# lcp-sql-fetch

A tool that executes local SQL scripts on a Liferay Cloud database via lcp shell and downloads the results.

## Features

- **Automated Execution:** Uses `expect` to automate the `lcp shell` login and script execution.
- **File Management:** Handles uploading local SQL scripts and downloading result files via `lcp files`.
- **Liferay Cloud Aware:** Designed specifically for Liferay Cloud (LXC) database environments.
- **Cross-Platform:** GitHub Actions pre-configured for Windows, Linux, and macOS (ARM/Intel).

## Project Structure

```plaintext
.
├── .cargo/config.toml        # Cargo aliases (setup, lint)
├── .gemini/prompts/          # Automated Gemini CLI workflows
├── .github/workflows/        # Multi-OS CI/CD (Release, Rust)
├── .githooks/pre-commit      # Shared cross-platform git hook
├── src/
│   ├── main.rs               # Command routing
│   ├── core/
│   │   ├── mod.rs            # Core traits
│   │   └── env.rs            # Project discovery logic
│   ├── utils/
│   │   ├── mod.rs            # Utility re-exports
│   │   ├── git.rs            # Git wrappers
│   │   └── xml.rs            # Recursive XML logic
│   └── cli.rs                # Command definitions
├── formula.rb.example        # Homebrew template
├── scoop.json.example        # Scoop template
├── .gitignore                # Tracks Cargo.lock for reliable CI
├── Cargo.toml                # Feature-based dependencies
└── LICENSE (MIT)
```

## Prerequisites

- **Rust:** `cargo`, `rustc`, `rustfmt`, `clippy`.
- **Liferay Cloud CLI (lcp):** Must be installed and authenticated.
- **expect:** Must be installed on the system for shell automation.
- **Git Hooks:** To ensure consistent code style, activate the shared pre-commit hooks:
  - Run: `git config core.hooksPath .githooks`.
  - On macOS/Linux: `chmod +x .githooks/pre-commit`.
  - This hook automatically runs `cargo fmt` and `cargo clippy` before each commit.

## Installation (End-Users)

Once a release is published and distribution channels are updated, users can install the tool using these commands:

### Homebrew (macOS / Linux)

```bash
brew tap peterrichards-lr/homebrew-tap
brew install lcp-sql-fetch
```

### Scoop (Windows)

```bash
scoop bucket add lcp-sql-fetch-bucket https://github.com/peterrichards-lr/scoop-bucket
scoop install lcp-sql-fetch
```

## Development

```bash
# Build locally
cargo build

# Run with arguments
cargo run -- fetch -p [project-id] -f query.sql
```

## Distribution (macOS, Linux, Windows)

To avoid "Unidentified Developer" warnings on macOS and ensure a secure, user-level installation on Windows, we recommend building from source via **Homebrew** or **Scoop**.

### Repository Visibility & Authentication

By default, Homebrew assumes your **homebrew-tap** and the tool's source repository are **public**.

If you wish to keep your distribution repositories **private**:

1. Users must have a **GitHub Personal Access Token (PAT)** with `repo` scope.
2. Users should export this token in their environment:
   ```bash
   export HOMEBREW_GITHUB_API_TOKEN=your_token_here
   ```
3. Without a token, `brew tap` and `brew install` will fail for private repositories.

### Automated Distribution via Gemini

This template includes an automated prompt for Gemini CLI to handle updating your Homebrew tap and Scoop bucket repositories with new releases.

When you create a new GitHub release, you can simply ask Gemini:

```bash
"Please execute the steps in .gemini/prompts/update-distribution-channels.md to update my distribution repositories"
```

Gemini will automatically:

1. Extract metadata from `Cargo.toml`.
2. Calculate the SHA256 hash of the release tarball.
3. Generate and write the Homebrew formula (`formula.rb.example`) and Scoop manifest (`scoop.json.example`).
4. Commit and push the updates to your local `homebrew-tap` and `scoop-bucket` repositories.
