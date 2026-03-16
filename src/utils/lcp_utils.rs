use dialoguer::{Confirm, Input, Password};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;
use walkdir::WalkDir;

pub struct SqlExecutionContext<'a> {
    pub project_id: &'a str,
    pub service: &'a str,
    pub instance_id: &'a str,
    pub password: &'a str,
    pub sql_content: &'a str,
    pub output_filename: &'a str,
    pub database_type: &'a crate::cli::DatabaseType,
    pub user: &'a str,
    pub verbose: bool,
}

pub fn check_expect_available() -> anyhow::Result<()> {
    if Command::new("expect").arg("-v").output().is_err() {
        anyhow::bail!(
            "'expect' is not installed or not in PATH. Please install it to use this tool."
        );
    }
    Ok(())
}

pub fn prompt_password(project_id: &str, user: &str) -> anyhow::Result<String> {
    let password = Password::new()
        .with_prompt(format!(
            "Enter database password for project: {} (User: {})",
            project_id, user
        ))
        .interact()?;
    Ok(password)
}

pub fn prompt_environment(project: &str) -> anyhow::Result<String> {
    let env: String = Input::new()
        .with_prompt(format!(
            "Enter environment for project '{}' (e.g. dev, uat, prd)",
            project
        ))
        .interact_text()?;
    Ok(env)
}

pub fn check_destructive_sql(sql: &str, force: bool) -> anyhow::Result<()> {
    if force {
        return Ok(());
    }

    let destructive_keywords = [
        "UPDATE", "DELETE", "DROP", "TRUNCATE", "ALTER", "GRANT", "REVOKE", "INSERT",
    ];

    let mut found_keywords = Vec::new();
    let upper_sql = sql.to_uppercase();

    for &kw in &destructive_keywords {
        if upper_sql.contains(kw) {
            found_keywords.push(kw);
        }
    }

    if !found_keywords.is_empty() {
        println!("\n⚠️  WARNING: Potentially destructive or data-modifying keywords found:");
        println!("   {}", found_keywords.join(", "));
        println!("\nThis tool is primarily intended for FETCHING data.");

        if !Confirm::new()
            .with_prompt("Are you sure you want to execute this script?")
            .default(false)
            .interact()?
        {
            anyhow::bail!("Operation cancelled by user.");
        }
    }

    Ok(())
}

pub fn get_running_instance(project_id: &str, service: &str) -> anyhow::Result<String> {
    let mut child = Command::new("lcp")
        .args(["shell", "-p", project_id, "-s", service])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "exit");
    }

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    if let Some(id) = parse_instance_id_from_list(&stdout, service) {
        return Ok(id);
    }

    anyhow::bail!(
        "Could not find a running instance ID for service: {} in project: {}.",
        service,
        project_id
    )
}

fn parse_instance_id_from_list(input: &str, service: &str) -> Option<String> {
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let instance = parts[1];
            let status = parts[2];
            if instance.starts_with(service) && status.to_lowercase() == "running" {
                return Some(instance.to_string());
            }
        }
    }
    None
}

pub fn download_file(
    project_id: &str,
    service: &str,
    remote_filename: &str, // Relative to /mnt/persistent-storage
    local_path: &Path,
) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid progress bar template"),
    );
    pb.set_message(format!("Downloading result to {}...", local_path.display()));
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    let temp_download_dir = tempfile::tempdir()?;
    let temp_download_path = temp_download_dir.path();

    let status = Command::new("lcp")
        .args([
            "files",
            "download",
            "-p",
            project_id,
            "-s",
            service,
            "--prefix",
            remote_filename,
            "--dest",
            temp_download_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid temp download path"))?,
        ])
        .status()?;

    if !status.success() {
        pb.finish_with_message("Download failed!");
        anyhow::bail!("Failed to download file via 'lcp files download'.");
    }

    let mut found_file = None;
    for entry in WalkDir::new(temp_download_path) {
        let entry = entry?;
        if entry.file_name().to_string_lossy() == remote_filename {
            found_file = Some(entry.path().to_path_buf());
            break;
        }
    }

    if let Some(src_path) = found_file {
        std::fs::copy(src_path, local_path)?;
        pb.finish_with_message("Download complete.");
        Ok(())
    } else {
        pb.finish_with_message("File not found!");
        // If the file is not found, it might be because the SQL failed and didn't create it.
        anyhow::bail!(
            "Downloaded file not found. The SQL query likely failed to create an output."
        );
    }
}

pub fn run_sql_via_expect(ctx: SqlExecutionContext) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid progress bar template"),
    );
    pb.set_message("Executing SQL script via lcp shell injection...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    let escaped_password = ctx.password.replace("'", "'\\''");
    let remote_output_path = format!("/mnt/persistent-storage/{}", ctx.output_filename);

    let db_cmd = match ctx.database_type {
        crate::cli::DatabaseType::Psql => format!(
            "export PGPASSWORD='{}' && psql -v ON_ERROR_STOP=1 -U {} -d lportal -f /tmp/query.sql > {} 2>&1",
            escaped_password, ctx.user, remote_output_path
        ),
        crate::cli::DatabaseType::Mysql => format!(
            "export MYSQL_PWD='{}' && mysql --abort-on-error -u {} lportal < /tmp/query.sql > {} 2>&1",
            escaped_password, ctx.user, remote_output_path
        ),
    };

    let escaped_content = ctx
        .sql_content
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("$", "\\$")
        .replace("[", "\\[")
        .replace("]", "\\]");

    let expect_script = format!(
        r#"#!/usr/bin/expect -f
set timeout 600
spawn lcp shell -p {} -s {} --instance {}
expect "$ "
send -- "cat <<'LCP_SQL_EOF' > /tmp/query.sql\r"
send -- "{}\r"
send -- "LCP_SQL_EOF\r"
expect "$ "
send -- "{}; echo LCP_EXIT_CODE:\$? \r"
expect -re "LCP_EXIT_CODE:(\[0-9\]+)"
set exit_code $expect_out(1,string)
expect "$ "
send "exit\r"
expect eof
exit $exit_code
"#,
        ctx.project_id, ctx.service, ctx.instance_id, escaped_content, db_cmd
    );

    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", expect_script)?;
    let path = temp_file
        .path()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid temp file path"))?;

    let mut cmd = Command::new("expect");
    cmd.arg(path);

    if !ctx.verbose {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = cmd.status()?;

    if !status.success() {
        pb.finish_with_message("SQL execution failed!");
        anyhow::bail!("SQL command failed with exit code: {:?}", status.code());
    }

    pb.finish_with_message("SQL execution complete.");
    Ok(())
}

pub fn cleanup_remote_files(
    project_id: &str,
    service: &str,
    instance_id: &str,
    files: &[&str],
    verbose: bool,
) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid progress bar template"),
    );
    pb.set_message("Cleaning up remote temporary files...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    let files_str = files.join(" ");
    let expect_script = format!(
        r#"#!/usr/bin/expect -f
set timeout 30
spawn lcp shell -p {} -s {} --instance {}
expect "$ "
send "rm {}\r"
expect "$ "
send "exit\r"
expect eof
"#,
        project_id, service, instance_id, files_str
    );

    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", expect_script)?;
    let path = temp_file
        .path()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid temp file path"))?;

    let mut cmd = Command::new("expect");
    cmd.arg(path);

    if !verbose {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let _ = cmd.status();

    pb.finish_with_message("Cleanup complete.");
    Ok(())
}
