use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "lcp-sql-fetch")]
#[command(about = "Execute local SQL scripts on Liferay Cloud and fetch results", long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: AppCommands,

    /// Show verbose output from internal processes (lcp shell, expect, etc.)
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum DatabaseType {
    #[default]
    Psql,
    Mysql,
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
        #[arg(short = 'o', long, default_value = "output.txt")]
        output: std::path::PathBuf,

        /// Database username. Defaults to the project-environment ID (read-only user).
        #[arg(short = 'u', long)]
        user: Option<String>,

        /// Database password. If missing, you will be prompted securely.

        #[arg(short = 'P', long)]
        password: Option<String>,

        /// Bypass the safety warning for destructive SQL statements (UPDATE, DELETE, etc.).
        #[arg(long)]
        force: bool,

        /// Database type to use for execution.
        #[arg(short = 'd', long, value_enum, default_value_t = DatabaseType::Psql)]
        database_type: DatabaseType,
    },
}
