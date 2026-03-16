mod cli;
mod core;
mod utils;

use crate::cli::{App, AppCommands};
use crate::core::LcpClient;
use crate::utils::lcp_utils;
use clap::Parser;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    match args.command {
        AppCommands::Fetch {
            project,
            environment,
            service,
            file,
            output,
            password,
        } => {
            // 0. Runtime Dependency Check
            lcp_utils::check_expect_available()?;

            // 1. Project ID Resolution (Smart Detection)
            let project_id = if project.contains('-') {
                project.clone()
            } else if let Some(env) = environment {
                format!("{}-{}", project, env)
            } else {
                print!(
                    "Project ID '{}' does not contain an environment suffix. Please enter environment (e.g., prd, uat): ",
                    project
                );
                io::stdout().flush()?;
                let mut env_input = String::new();
                io::stdin().read_line(&mut env_input)?;
                format!("{}-{}", project, env_input.trim())
            };

            // 2. Auth & Project Validation
            let lcp = LcpClient::new()?;
            lcp.validate_project(&project_id)?;

            // 3. Password Secure Prompt
            let pwd = match password {
                Some(p) => p,
                None => lcp_utils::prompt_password()?,
            };

            // 4. Instance Discovery
            let instance_id = lcp_utils::get_running_instance(&project_id, &service)?;

            // 5. Remote Path Generation (Unique to avoid collisions)
            let ts = chrono::Utc::now().timestamp();
            let remote_sql = format!("/tmp/query_{}.sql", ts);
            let remote_output = format!("/tmp/output_{}.txt", ts);

            // 6. File Upload
            lcp_utils::upload_file(&project_id, &service, &file, &remote_sql)?;

            // 7. Automated Execution (Expect)
            lcp_utils::run_sql_via_expect(
                &project_id,
                &service,
                &instance_id,
                &pwd,
                &remote_sql,
                &remote_output,
            )?;

            // 8. File Download
            lcp_utils::download_file(&project_id, &service, &remote_output, &output)?;

            // 9. Cleanup
            lcp_utils::cleanup_remote_files(
                &project_id,
                &service,
                &instance_id,
                &[&remote_sql, &remote_output],
            )?;

            println!("\n✨ Success! SQL results saved to: {}", output.display());
        }
    }

    Ok(())
}
