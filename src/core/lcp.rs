use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

pub struct LcpClient {
    pub token: String,
    pub client: reqwest::blocking::Client,
}

#[derive(Serialize)]
struct LoginRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

impl LcpClient {
    pub fn new() -> anyhow::Result<Self> {
        let token = Self::get_token()?;
        let client = reqwest::blocking::Client::new();
        Ok(Self { token, client })
    }

    fn get_token() -> anyhow::Result<String> {
        // 1. Try environment variable
        if let Ok(token) = env::var("LCP_TOKEN") {
            return Ok(token);
        }

        // 2. Try local file (~/.lcp/token)
        let home =
            home::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let token_path = home.join(".lcp").join("token");
        if token_path.exists() {
            let token = fs::read_to_string(token_path)?.trim().to_string();
            if !token.is_empty() {
                return Ok(token);
            }
        }

        // 3. Fallback to login if credentials are provided
        if let (Ok(email), Ok(password)) = (env::var("LCP_EMAIL"), env::var("LCP_PASSWORD")) {
            let client = reqwest::blocking::Client::new();
            let res = client
                .post("https://api.liferay.cloud/login")
                .json(&LoginRequest {
                    email: &email,
                    password: &password,
                })
                .send()?
                .error_for_status()?;
            let login_res: LoginResponse = res.json()?;
            return Ok(login_res.token);
        }

        anyhow::bail!("No Liferay Cloud token found. Please run 'lcp login' or set LCP_TOKEN/LCP_EMAIL+LCP_PASSWORD.")
    }

    pub fn validate_project(&self, project_id: &str) -> anyhow::Result<()> {
        let query = r#"
            query ProjectsQuery {
                projects {
                    projectId
                }
            }
        "#;

        let res = self
            .client
            .post("https://api.liferay.cloud/graphql")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&serde_json::json!({
                "query": query
            }))
            .send()?
            .error_for_status()?;

        let json: serde_json::Value = res.json()?;
        let projects = json["data"]["projects"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse projects from API response"))?;

        let exists = projects
            .iter()
            .any(|p| p["projectId"].as_str() == Some(project_id));

        if !exists {
            anyhow::bail!("Project '{}' not found in Liferay Cloud.", project_id);
        }

        Ok(())
    }
}
