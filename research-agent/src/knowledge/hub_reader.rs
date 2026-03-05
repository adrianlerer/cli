use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use tracing::{debug, warn};

/// Represents a file in the Hub
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubFile {
    pub file_name: String,
    pub summary: Option<String>,
}

/// Interface to Hub files
pub struct HubReader {
    // Future: Add caching, rate limiting, etc.
}

impl HubReader {
    pub fn new() -> Self {
        Self {}
    }

    /// List all files in the Hub
    pub async fn list_files(&self) -> Result<Vec<HubFile>> {
        debug!("Listing files from Hub");

        let output = Command::new("hub_files_tool")
            .arg("ls")
            .stdin(Stdio::null())
            .output()
            .context("Failed to execute hub_files_tool")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("hub_files_tool failed: {}", stderr));
        }

        let stdout = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 from hub_files_tool")?;

        self.parse_ls_output(&stdout)
    }

    /// Read a file from the Hub
    pub async fn read_file(
        &self,
        file_name: &str,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<String> {
        debug!("Reading file from Hub: {}", file_name);

        let mut args = vec!["read".to_string(), "--file-name".to_string(), file_name.to_string()];

        if let Some(off) = offset {
            args.push("--offset".to_string());
            args.push(off.to_string());
        }

        if let Some(lim) = limit {
            args.push("--limit".to_string());
            args.push(lim.to_string());
        }

        let output = Command::new("hub_files_tool")
            .args(&args)
            .stdin(Stdio::null())
            .output()
            .context("Failed to execute hub_files_tool")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("hub_files_tool read failed: {}", stderr));
        }

        let content = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 from hub_files_tool")?;

        Ok(content)
    }

    /// Search files using grep
    pub async fn grep_files(&self, pattern: &str, include: Option<&str>) -> Result<Vec<GrepMatch>> {
        debug!("Grepping Hub files with pattern: {}", pattern);

        let mut args = vec!["grep".to_string(), "--pattern".to_string(), pattern.to_string()];

        if let Some(inc) = include {
            args.push("--include".to_string());
            args.push(inc.to_string());
        }

        let output = Command::new("hub_files_tool")
            .args(&args)
            .stdin(Stdio::null())
            .output()
            .context("Failed to execute hub_files_tool")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("hub_files_tool grep failed: {}", stderr);
            return Ok(vec![]); // Return empty on grep failure
        }

        let stdout = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 from hub_files_tool")?;

        self.parse_grep_output(&stdout)
    }

    /// Parse the output of 'hub_files_tool ls'
    fn parse_ls_output(&self, output: &str) -> Result<Vec<HubFile>> {
        let mut files = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        // Skip header lines
        let mut data_started = false;
        let mut current_file: Option<String> = None;
        let mut current_summary: Option<String> = None;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            // Detect header separator (---)
            if trimmed.starts_with("---") {
                data_started = true;
                continue;
            }

            if !data_started {
                continue;
            }

            // Parse file entries (format: "filename | summary")
            if let Some(pos) = trimmed.find('|') {
                // Save previous file if exists
                if let Some(file_name) = current_file.take() {
                    files.push(HubFile {
                        file_name,
                        summary: current_summary.take(),
                    });
                }

                let file_name = trimmed[..pos].trim().to_string();
                let summary = trimmed[pos + 1..].trim();
                
                current_file = Some(file_name);
                current_summary = if summary.is_empty() {
                    None
                } else {
                    Some(summary.to_string())
                };
            }
        }

        // Don't forget last file
        if let Some(file_name) = current_file {
            files.push(HubFile {
                file_name,
                summary: current_summary,
            });
        }

        debug!("Parsed {} files from Hub", files.len());
        Ok(files)
    }

    /// Parse grep output
    fn parse_grep_output(&self, output: &str) -> Result<Vec<GrepMatch>> {
        let mut matches = Vec::new();

        for line in output.lines() {
            // Expected format: "filename:line_number:content"
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() == 3 {
                if let Ok(line_num) = parts[1].parse::<usize>() {
                    matches.push(GrepMatch {
                        file_name: parts[0].to_string(),
                        line_number: line_num,
                        content: parts[2].to_string(),
                    });
                }
            }
        }

        Ok(matches)
    }
}

impl Default for HubReader {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a grep search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepMatch {
    pub file_name: String,
    pub line_number: usize,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ls_output() {
        let output = r#"
File Name | Summary
--- | ---
test.pdf | Test document
another.pdf | Another test
"#;

        let reader = HubReader::new();
        let files = reader.parse_ls_output(output).unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].file_name, "test.pdf");
        assert_eq!(files[0].summary, Some("Test document".to_string()));
    }
}
