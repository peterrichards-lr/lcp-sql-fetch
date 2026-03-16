# Gemini Prompt: Setup New Tool Repository

**Goal:** Transform the cloned template repo into a production-ready repository for a new Liferay Rust CLI tool.

**Instructions for Gemini:**
When a user asks you to "setup the new tool," "convert this template," or "run the initialization," follow these steps strictly:

---

### 1. Information Gathering
Before making any changes, ask the user the following questions:
- **Tool Name:** (e.g., `lfr-local`, `dxp-env`)
- **Short Description:** (e.g., "A tool for managing local DXP instances")
- **Author:** (e.g., "Your Name <email@example.com>")
- **GitHub Username/Org:** (For repository and distribution URLs)
- **Optional Features:** Which should be enabled by default? (e.g., `web` for scraping, `yaml` for YAML config)

### 2. Update Cargo.toml
- Set `package.name` to the **Tool Name**.
- Set `package.version` to `0.1.0`.
- Set `package.description` to the **Short Description**.
- Update `package.authors`.
- Update the `default` features list in the `[features]` section based on user preference.

### 3. Update GitHub Release Workflow
In `.github/workflows/release.yml`:
- Replace all occurrences of `REPLACE_ME` with the **Tool Name**.
- Ensure the `asset_name` patterns (e.g., `{{TOOL_NAME}}-linux-x86_64.tar.gz`) use the new name correctly.

### 4. Update README.md
- Replace the main title with the **Tool Name**.
- Replace the description with the **Short Description**.
- Update the "Getting Started" section to reflect that the tool has already been initialized.
- **Update Installation commands:** Replace `[github-user]`, `[tool-name]`, and `[bucket-name]` placeholders with the user's actual information.
- Update any other command examples (e.g., `cargo run -- --help`) to use the new tool's context.

### 5. Final Setup & Reminders
Finalize the environment:
- **Initialize Hooks:** Run the command: `git config core.hooksPath .githooks`
- **Set Permissions:** (Unix only) If on macOS/Linux, run: `chmod +x .githooks/pre-commit`
- **Verify:** Remind the user they can now run `cargo lint` and `cargo fmt-check`.
- Update `LICENSE` if necessary.
- Review `src/cli.rs` to define their tool's specific subcommands.
- Push the changes and a new tag (e.g., `v0.1.0`) to verify the release workflow.

---

**To the User:** 
To execute this, ask Gemini: *"Please execute .gemini/prompts/setup-new-tool.md to initialize my new project"*
