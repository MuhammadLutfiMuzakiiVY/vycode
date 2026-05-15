// VyCode - Headless Chrome Remote Driver
use anyhow::Result;
use std::process::Command;
use std::path::Path;

/// Drives the local installation of Google Chrome/Chromium via headless terminal hooks
pub fn drive_browser(operation: &str, url: &str, output_target: &str) -> Result<String> {
    // Find standard Chrome path for Windows systems
    let possible_paths = [
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        "chrome" // Fallback to PATH
    ];

    let mut chrome_exe = "chrome";
    for path in &possible_paths {
        if Path::new(path).exists() {
            chrome_exe = path;
            break;
        }
    }

    let mut cmd = Command::new(chrome_exe);
    cmd.arg("--headless")
       .arg("--disable-gpu")
       .arg("--no-sandbox");

    match operation {
        "dom" => {
            // Dumps the completely executed DOM
            let output = cmd.arg("--dump-dom")
                .arg(url)
                .output()?;
            
            if output.status.success() {
                let html = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(format!("🌐 [Browser] Successfully dumped executed DOM ({} bytes).", html.len()))
            } else {
                Err(anyhow::anyhow!("Chrome driver failure: {}", String::from_utf8_lossy(&output.stderr)))
            }
        }
        "screenshot" => {
            // Captures a live PNG screenshot of the URL
            let target = if output_target.is_empty() { "screenshot.png" } else { output_target };
            cmd.arg(format!("--screenshot={}", target))
                .arg(url)
                .output()?;
            
            Ok(format!("📸 [Browser] Live screenshot rendered successfully to `{}`!", target))
        }
        "pdf" => {
            // Prints the target URL as a PDF document
            let target = if output_target.is_empty() { "page.pdf" } else { output_target };
            cmd.arg(format!("--print-to-pdf={}", target))
                .arg(url)
                .output()?;
            
            Ok(format!("📄 [Browser] Target printed successfully to PDF at `{}`!", target))
        }
        _ => Err(anyhow::anyhow!("Unsupported Chrome operation `{}`", operation))
    }
}
