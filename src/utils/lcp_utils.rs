use indicatif::{ProgressBar, ProgressStyle};
use rpassword::read_password;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn check_expect_available() -> anyhow::Result<()> {
    if Command::new("expect").arg("-v").output().is_err() {
        anyhow::bail!(
            "'expect' is not installed or not in PATH. Please install it to use this tool."
        );
    }
    Ok(())
}

pub fn prompt_password() -> anyhow::Result<String> {
    print!("Database Password: ");
    io::stdout().flush()?;
    let password = read_password()?;
    Ok(password)
}

pub fn get_running_instance(project_id: &str, service: &str) -> anyhow::Result<String> {
    let output = Command::new("lcp")
        .args(["status", "-p", project_id, "-s", service])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to get status for project '{}' and service '{}': {}",
            project_id,
            service,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Skip the header and find the first 'Running' instance
    for line in stdout.lines().skip(1) {
        if line.contains("Running") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                return Ok(parts[0].to_string());
            }
        }
    }

    anyhow::bail!(
        "No running instance found for project '{}' and service '{}'.",
        project_id,
        service
    )
}

pub fn upload_file(
    project_id: &str,
    service: &str,
    local_path: &Path,
    remote_path: &str,
) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid progress bar template"),
    );
    pb.set_message(format!("Uploading {}...", local_path.display()));
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    let status = Command::new("lcp")
        .args([
            "files",
            "upload",
            "-p",
            project_id,
            "-s",
            service,
            local_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid local path"))?,
            remote_path,
        ])
        .status()?;

    if !status.success() {
        pb.finish_with_message("Upload failed!");
        anyhow::bail!("Failed to upload file via 'lcp files upload'.");
    }

    pb.finish_with_message("Upload complete.");
    Ok(())
}

pub fn download_file(
    project_id: &str,
    service: &str,
    remote_path: &str,
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

    let status = Command::new("lcp")
        .args([
            "files",
            "download",
            "-p",
            project_id,
            "-s",
            service,
            remote_path,
            local_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid local path"))?,
        ])
        .status()?;

    if !status.success() {
        pb.finish_with_message("Download failed!");
        anyhow::bail!("Failed to download file via 'lcp files download'.");
    }

    pb.finish_with_message("Download complete.");
    Ok(())
}

pub fn run_sql_via_expect(
    project_id: &str,
    service: &str,
    instance_id: &str,
    password: &str,
    sql_remote_path: &str,
    output_remote_path: &str,
) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid progress bar template"),
    );
    pb.set_message("Executing SQL script via lcp shell...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    let expect_script = format!(
        r#"
set timeout 600
spawn lcp shell -p {project_id} -s {service} --instance {instance_id}
expect -re "[$#] "
send "psql -U dxpcloud -d lportal -f {sql_remote_path} > {output_remote_path}\r"
expect {{
    "Password for user dxpcloud:" {{
        send "{password}\r"
        exp_continue
    }}
    -re "[$#] " {{
        send "exit\r"
    }}
}}
expect eof
"#,
        project_id = project_id,
        service = service,
        instance_id = instance_id,
        password = password,
        sql_remote_path = sql_remote_path,
        output_remote_path = output_remote_path
    );

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(expect_script.as_bytes())?;
    let path = temp_file
        .path()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid temp file path"))?;

    let status = Command::new("expect").arg(path).status()?;

    if !status.success() {
        pb.finish_with_message("SQL execution failed!");
        anyhow::bail!("Expect script failed.");
    }

    pb.finish_with_message("SQL execution complete.");
    Ok(())
}

pub fn cleanup_remote_files(
    project_id: &str,
    service: &str,
    instance_id: &str,
    files: &[&str],
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
        r#"
set timeout 30
spawn lcp shell -p {project_id} -s {service} --instance {instance_id}
expect -re "[$#] "
send "rm {files_str}\r"
expect -re "[$#] "
send "exit\r"
expect eof
"#,
        project_id = project_id,
        service = service,
        instance_id = instance_id,
        files_str = files_str
    );

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(expect_script.as_bytes())?;
    let path = temp_file
        .path()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid temp file path"))?;

    let _ = Command::new("expect").arg(path).status();

    pb.finish_with_message("Cleanup complete.");
    Ok(())
}
