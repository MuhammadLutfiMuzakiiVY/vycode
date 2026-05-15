// VyCode - Lightweight Documentation Scraping & Reading Tool
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

/// Fetches public URLs, strips HTML tags, and returns clean textual context
pub async fn fetch_documentation(url: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Compatible; VyCodeBot/2.0; +https://github.com/MuhammadLutfiMuzakiiVY/vycode)")
        .build()?;

    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("HTTP Request Failed with status: {}", response.status()));
    }

    let html = response.text().await?;
    let cleaned = extract_text_from_html(&html);
    
    // Restrict size for efficient LLM context window processing
    let final_text = if cleaned.len() > 8000 {
        let mut shortened = cleaned[0..8000].to_string();
        shortened.push_str("\n\n[...Content truncated for brevity...]");
        shortened
    } else {
        cleaned
    };

    Ok(format!(
        "🌐 [TOOL: DOCS] Pulled from: {}\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n{}",
        url,
        final_text
    ))
}

/// Super lightweight HTML text stripper to keep binary sizes ultra compact (<100MB)
/// Removes script tags, style tags, and returns plain textual contexts
fn extract_text_from_html(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    let mut skip_mode = false;
    let mut current_tag = String::new();
    
    let mut chars = html.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '<' {
            in_tag = true;
            current_tag.clear();
            continue;
        }
        if c == '>' {
            in_tag = false;
            let tag_lower = current_tag.to_lowercase();
            if tag_lower.starts_with("script") || tag_lower.starts_with("style") || tag_lower.starts_with("head") {
                skip_mode = true;
            }
            if tag_lower.starts_with("/script") || tag_lower.starts_with("/style") || tag_lower.starts_with("/head") {
                skip_mode = false;
            }
            continue;
        }
        
        if in_tag {
            current_tag.push(c);
        } else if !skip_mode {
            text.push(c);
        }
    }

    // Clean up excess whitespace
    let mut output = String::new();
    
    for c in text.lines() {
        let trimmed = c.trim();
        if !trimmed.is_empty() {
            output.push_str(trimmed);
            output.push('\n');
        }
    }

    output
}
