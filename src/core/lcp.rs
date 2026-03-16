use std::process::Command;

pub struct LcpClient {
    pub token: String,
    pub client: reqwest::blocking::Client,
}

impl LcpClient {
    pub fn new() -> anyhow::Result<Self> {
        let token = Self::get_token()?;
        let client = reqwest::blocking::Client::new();
        Ok(Self { token, client })
    }

    fn get_token() -> anyhow::Result<String> {
        let output = Command::new("lcp").args(["auth", "token"]).output()?;

        if !output.status.success() {
            anyhow::bail!("No Liferay Cloud token found. Please run 'lcp login' first.");
        }

        let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if token.is_empty() {
            anyhow::bail!("Liferay Cloud token is empty. Please run 'lcp login'.");
        }

        Ok(token)
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
