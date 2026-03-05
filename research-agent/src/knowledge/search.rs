use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::hub_reader::HubReader;

/// Search query parameters
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    pub text: String,
    pub max_results: usize,
    pub discipline: Option<String>,
    pub authors: Vec<String>,
    pub date_range: Option<(u16, u16)>,
}

/// Search result with relevance score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_name: String,
    pub score: f64,
    pub summary: Option<String>,
    pub matched_content: Option<Vec<String>>,
}

impl HubReader {
    /// Search the Hub corpus
    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // For now, use simple grep-based search
        // Future: Implement BM25, semantic search, etc.

        let pattern = &query.text;
        let grep_matches = self.grep_files(pattern, Some("*.pdf")).await?;

        // Group matches by file
        let mut file_matches: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for m in grep_matches {
            file_matches
                .entry(m.file_name.clone())
                .or_default()
                .push(m.content);
        }

        // Get all files for summaries
        let all_files = self.list_files().await?;
        let file_summaries: std::collections::HashMap<String, Option<String>> = all_files
            .into_iter()
            .map(|f| (f.file_name, f.summary))
            .collect();

        // Build results
        let mut results: Vec<SearchResult> = file_matches
            .into_iter()
            .map(|(file_name, matches)| {
                let score = calculate_simple_score(&matches, &query.text);
                SearchResult {
                    file_name: file_name.clone(),
                    score,
                    summary: file_summaries.get(&file_name).cloned().flatten(),
                    matched_content: Some(matches),
                }
            })
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        // Limit results
        results.truncate(query.max_results);

        Ok(results)
    }
}

/// Calculate a simple relevance score
fn calculate_simple_score(matches: &[String], query: &str) -> f64 {
    let num_matches = matches.len() as f64;
    let query_words: Vec<&str> = query.split_whitespace().collect();

    // Count how many query words appear in matches
    let mut word_hits = 0;
    for m in matches {
        let m_lower = m.to_lowercase();
        for word in &query_words {
            if m_lower.contains(&word.to_lowercase()) {
                word_hits += 1;
            }
        }
    }

    // Simple scoring: number of matches + word coverage
    let word_coverage = word_hits as f64 / (query_words.len() as f64).max(1.0);
    
    num_matches.log10() + word_coverage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_simple_score() {
        let matches = vec![
            "This is about game theory".to_string(),
            "Game theory applications".to_string(),
        ];

        let score = calculate_simple_score(&matches, "game theory");
        assert!(score > 0.0);
    }
}
