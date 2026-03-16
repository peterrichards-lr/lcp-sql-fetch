use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lcp-sql-fetch")]
#[command(about = "Execute local SQL scripts on Liferay Cloud and fetch results", long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: AppCommands,
}

#[derive(Subcommand)]
pub enum AppCommands {
    /// Executes a local SQL script on a Liferay Cloud database and downloads the result.
    Fetch {
        /// Project ID or full ID (e.g., 'acme' or 'acme-prd').
        #[arg(short, long)]
        project: String,

        /// Optional environment suffix (e.g., 'prd', 'uat').
        #[arg(short, long)]
        environment: Option<String>,

        /// Liferay Cloud service name (default: liferay).
        #[arg(short, long, default_value = "liferay")]
        service: String,

        /// Path to the local .sql script to execute.
        #[arg(short, long)]
        file: std::path::PathBuf,

        /// Local path for the result file.
        #[arg(short, long, default_value = "output.txt")]
        output: std::path::PathBuf,

        /// Database password. If missing, you will be prompted securely.
        #[arg(short = 'P', long)]
        password: Option<String>,
    },
}
