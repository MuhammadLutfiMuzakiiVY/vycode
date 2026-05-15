// VyCode - Database Unified CLI Bridging Engine
use anyhow::Result;
use std::process::Command;

/// Performs optimized database queries securely leveraging native SQL command line client runners
pub fn execute_query(engine: &str, connection_uri: &str, sql: &str) -> Result<String> {
    let mut cmd = match engine {
        "sqlite" => {
            let mut c = Command::new("sqlite3");
            c.arg(connection_uri).arg(sql);
            c
        }
        "postgres" | "psql" => {
            let mut c = Command::new("psql");
            c.arg("-d").arg(connection_uri).arg("-c").arg(sql);
            c
        }
        "mysql" => {
            // Assumes uri matches `mysql://user:pass@host/db` or config
            let mut c = Command::new("mysql");
            c.arg("-e").arg(sql); // Simplified for CLI invocation
            c
        }
        _ => return Err(anyhow::anyhow!("Unsupported database engine `{}`", engine))
    };

    let output = cmd.output()?;
    if output.status.success() {
        Ok(format!("🗄️ [DB {}]:\n{}", engine, String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(anyhow::anyhow!("Database client failure: {}", String::from_utf8_lossy(&output.stderr)))
    }
}
