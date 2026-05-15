// VyCode - Model Context Protocol (MCP) Async Integration Client
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

/// Executes direct JSON-RPC standard requests to Model Context Protocol (MCP) compatible API servers
pub async fn call_mcp_server(server_url: &str, method: &str, params: serde_json::Value) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()?;

    let rpc_payload = json!({
        "jsonrpc": "2.0",
        "id": "vycode-mcp-query",
        "method": method,
        "params": params
    });

    let response = client.post(server_url)
        .json(&rpc_payload)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        Ok(format!("🧩 [MCP Client] Raw response from server:\n{}", body))
    } else {
        Err(anyhow::anyhow!("MCP Server failed (HTTP {}): {}", status, body))
    }
}

/// Wrapper helper to easily list MCP tools
pub async fn list_mcp_tools(server_url: &str) -> Result<String> {
    call_mcp_server(server_url, "tools/list", json!({})).await
}

/// Wrapper helper to invoke a specific tool
pub async fn execute_mcp_tool(server_url: &str, name: &str, arguments: serde_json::Value) -> Result<String> {
    call_mcp_server(server_url, "tools/call", json!({
        "name": name,
        "arguments": arguments
    })).await
}
