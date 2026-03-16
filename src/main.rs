mod cli;
mod core;
mod utils;

use crate::cli::{App, AppCommands};
use crate::core::LcpClient;
use crate::utils::lcp_utils;
use crate::utils::lcp_utils::SqlExecutionContext;
use clap::Parser;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    match args.command {
        AppCommands::Fetch {
            project,
            environment,
            service,
            file,
            output,
            user,
            password,
            force,
            database_type,
        } => {
            // 0. Runtime Dependency Check
            lcp_utils::check_expect_available()?;

            // 1. Read Local SQL Content (Need this early for validation)
            let sql_content = fs::read_to_string(&file)?;

            // 2. Safety Validation
            lcp_utils::check_destructive_sql(&sql_content, force)?;

            // 3. Project ID Resolution (Smart Detection & Robust Prompting)
            let project_id = if let Some(env) = environment {
                if project.ends_with(&format!("-{}", env)) {
                    project
                } else {
                    format!("{}-{}", project, env)
                }
            } else if project.contains('-') {
                project
            } else {
                let env = lcp_utils::prompt_environment(&project)?;
                format!("{}-{}", project, env)
            };

            // 4. Username Resolution (Default to project_id for read-only user)
            let db_user = match user {
                Some(u) => u,
                None => project_id.clone(),
            };

            // 5. Auth & Project Validation
            let lcp = LcpClient::new()?;
            lcp.validate_project(&project_id)?;

            // 6. Password Secure Prompt (Include user in prompt)
            let pwd = match password {
                Some(p) => p,
                None => lcp_utils::prompt_password(&project_id, &db_user)?,
            };

            // 7. Instance Discovery
            let instance_id = lcp_utils::get_running_instance(&project_id, &service)?;

            // 8. Remote Path Generation
            let ts = chrono::Utc::now().timestamp();
            let remote_output_filename = format!("lcp_sql_output_{}.txt", ts);
            let remote_output_path = format!("/mnt/persistent-storage/{}", remote_output_filename);

            // 9. Automated Execution (Expect with injection)
            let sql_exec_res = lcp_utils::run_sql_via_expect(SqlExecutionContext {
                project_id: &project_id,
                service: &service,
                instance_id: &instance_id,
                password: &pwd,
                sql_content: &sql_content,
                output_filename: &remote_output_filename,
                database_type: &database_type,
                user: &db_user,
                verbose: args.verbose,
            });

            // 10. File Download (Always attempt download, as it may contain stderr on failure)
            let _ =
                lcp_utils::download_file(&project_id, &service, &remote_output_filename, &output);

            // 11. Cleanup
            let cleanup_res = lcp_utils::cleanup_remote_files(
                &project_id,
                &service,
                &instance_id,
                &["/tmp/query.sql", &remote_output_path],
                args.verbose,
            );

            // 12. Handle Execution Results
            if let Err(e) = sql_exec_res {
                println!("\n❌ SQL Execution Failed!");
                println!("Error: {}", e);

                if fs::metadata(&output).map(|m| m.len() > 0).unwrap_or(false) {
                    println!(
                        "\nThe database returned an error. Check: {} for details.",
                        output.display()
                    );
                } else {
                    println!("\nNo output was generated. Please check your SQL query and database connectivity.");
                }

                cleanup_res?;
                anyhow::bail!("Fetch failed due to SQL error.");
            }

            cleanup_res?;

            // 13. Result Verification (for successful execution with empty result set)
            if fs::metadata(&output)?.len() == 0 {
                println!("\n⚠️  Warning: The output file is empty. Your query may have returned no results.");
            } else {
                println!("\n✨ Success! SQL results saved to: {}", output.display());
            }
        }
    }

    Ok(())
}
