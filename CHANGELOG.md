# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Label Dependabot pull requests `no-issue-needed`, alongside the default `dependencies` and ecosystem labels, so bot PRs are marked as intentionally having no linked issue.

## [1.0.2] - 2026-11-11

### Added

- Add `--version` (`-V`) to the CLI.
- Add `CHANGELOG.md`, with sections backfilled for earlier releases.
- Add `scripts/release.py` (changelog note extraction and annotated tagging) and `scripts/doctor.py` (placeholder check, run by the pre-commit hook).
- Add `.github/dependabot.yml` for weekly `cargo` and GitHub Actions updates.
- Add `formula.rb.example` and `scoop.json.example` distribution templates.
- Add `repository` and `license` to `Cargo.toml`.

### Changed

- Align `Cargo.toml` with the released versions: the version was still `1.0.0` after the `v1.0.1` tag.
- Replace `.github/workflows/release.yml` with the template version: names are derived from `Cargo.toml`, tags must be annotated and match the `Cargo.toml` version, release notes come from `CHANGELOG.md`, and Intel macOS binaries are built.
- Pin GitHub Actions in `.github/workflows/rust.yml` to commit SHAs.
- Update `.gemini/prompts/update-distribution-channels.md` from the template.
- Remove the `cargo setup-hooks` alias, which could never work because Cargo aliases only run Cargo subcommands. Use `git config core.hooksPath .githooks`.
- Restructure `README.md` and add a Releasing section.

### Fixed

- Bump `openssl` to 0.10.82, `rand` to 0.8.8 and `rpassword` to 7.5.5 to resolve open security advisories.
- Correct README details: the default database user is the project-environment ID, the interactive environment prompt example no longer uses a hyphenated project ID, `-v/--verbose` is now documented, all example scripts are listed, and the `expect` requirement is stated.

### Removed

- Remove the unreferenced screenshot `temp/commerce_seed.png`.

## [1.0.1] - 2026-03-16

### Added

- Read the database password from the `LCP_DB_PASSWORD` environment variable when `--password` is not given.

## [1.0.0] - 2026-03-16

### Added

- Initial release: the `fetch` command uploads a local SQL script to a running Liferay Cloud service instance through `lcp shell` (automated with `expect`), runs it with `psql` or `mysql`, and downloads the output with `lcp files download`.
- Project ID detection with optional environment suffix and interactive environment prompt.
- Project validation against the Liferay Cloud API using the local `lcp` token.
- Secure password prompt, confirmation before running scripts with data-modifying keywords (bypassed with `--force`), and cleanup of remote files.
- Example SQL scripts in `examples/`.
- Release workflow building Linux, macOS (Apple Silicon) and Windows binaries.

<!-- markdownlint-disable MD049 -->

---

_Last Updated: 2026-11-11_ | _Last Reviewed: 2026-11-11_
