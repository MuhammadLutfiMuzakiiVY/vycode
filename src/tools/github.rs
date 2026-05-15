// VyCode - GitHub API Specialized Automation Driver
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

/// Dispatches requests to the GitHub REST v3 API
pub async fn execute_github(token: &str, repo: &str, operation: &str, payload: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("VyCode-Agent")
        .build()?;

    let base_url = format!("https://api.github.com/repos/{}", repo);
    
    let response = match operation {
        "issues" => {
            // List active issues
            client.get(&format!("{}/issues", base_url))
                .bearer_auth(token)
                .send()
                .await?
        }
        "pulls" => {
            // List pull requests
            client.get(&format!("{}/pulls", base_url))
                .bearer_auth(token)
                .send()
                .await?
        }
        "create-issue" => {
            // Create new issue
            let body = json!({ "title": payload, "body": "Auto-generated via VyCode Autonomous Hub." });
            client.post(&format!("{}/issues", base_url))
                .bearer_auth(token)
                .json(&body)
                .send()
                .await?
        }
        "workflows" => {
            // List Actions workflows
            client.get(&format!("{}/actions/workflows", base_url))
                .bearer_auth(token)
                .send()
                .await?
        }
        _ => return Err(anyhow::anyhow!("Unsupported GitHub operation `{}`", operation))
    };

    let status = response.status();
    let txt = response.text().await.unwrap_or_default();
    
    if status.is_success() {
        Ok(format!("✅ GitHub [{}]: Execution successful!\n{}", operation, txt))
    } else {
        Err(anyhow::anyhow!("GitHub error (HTTP {}): {}", status, txt))
    }
}
