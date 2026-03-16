# Liferay Rust Tool Template

A modular template for building high-performance, cross-platform CLI tools for Liferay DXP, Liferay Cloud (LXC), and Client Extensions.

## New in v0.3.0

- **Automated Initialization:** New Gemini prompt (`setup-new-tool.md`) to initialize metadata, features, and CI/CD automatically.
- **Native Git Hooks:** Shared, cross-platform pre-commit hooks (`.githooks/`) for automated formatting and Clippy lints.
- **Quality Aliases:** Built-in Cargo aliases for project setup, linting, and formatting checks.
- **Robust Distribution:** Enhanced automated distribution with pre-flight URL validation and repository discovery.

## Features

- **Cross-Platform:** GitHub Actions pre-configured for Windows, Linux, and macOS (ARM/Intel).
- **Liferay Aware:** Logic for path resolution and product version detection from `gradle.properties`.
- **Modern CLI:** Built on `clap` for a professional command-line experience.

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
- **Git Hooks:** To ensure consistent code style, activate the shared pre-commit hooks:
  - Run: `git config core.hooksPath .githooks`.
  - On macOS/Linux: `chmod +x .githooks/pre-commit`.
  - This hook automatically runs `cargo fmt` and `cargo clippy` before each commit.

## Getting Started

1. Click **"Use this template"** on GitHub to create your new repository.
2. **Automated Initialization:** Once cloned, ask Gemini to set up your new tool:
   ```bash
   "Please execute .gemini/prompts/setup-new-tool.md to initialize my new project"
   ```
   Gemini will automatically gather your project metadata (name, description, features) and update all core configuration files (`Cargo.toml`, `.github/workflows/release.yml`, and this `README.md`) for you.
3. **Manual Customization:**
   - Customize subcommands in `src/cli.rs`.
   - Update `LICENSE` if necessary.
4. **First Release:** Push a tag (e.g., `v1.0.0`) to trigger an automated release across all major operating systems.

## Installation (End-Users)

Once a release is published and distribution channels are updated, users can install the tool using these commands:

### Homebrew (macOS / Linux)

```bash
brew tap [github-user]/homebrew-tap
brew install [tool-name]
```

### Scoop (Windows)

```bash
scoop bucket add [tool-name]-bucket https://github.com/[github-user]/scoop-bucket
scoop install [tool-name]
```

## Development

```bash
# Build locally
cargo build

# Run with arguments
cargo run -- --help
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
