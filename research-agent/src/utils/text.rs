use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

/// Normalize text for processing
pub fn normalize_text(text: &str) -> String {
    // Remove excessive whitespace
    let re = Regex::new(r"\s+").unwrap();
    let normalized = re.replace_all(text, " ");
    normalized.trim().to_string()
}

/// Extract sentences from text
pub fn extract_sentences(text: &str) -> Vec<String> {
    // Simple sentence splitter
    // Future: Use more sophisticated NLP
    let re = Regex::new(r"[.!?]+\s+").unwrap();
    
    re.split(text)
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

/// Count words in text
pub fn word_count(text: &str) -> usize {
    text.unicode_words().count()
}

/// Extract keywords (simple version)
pub fn extract_keywords(text: &str, top_n: usize) -> Vec<String> {
    use std::collections::HashMap;

    let words: Vec<&str> = text.unicode_words().collect();
    let mut word_freq: HashMap<String, usize> = HashMap::new();

    // Count word frequencies (case-insensitive)
    for word in words {
        let word_lower = word.to_lowercase();
        
        // Filter out common stop words and short words
        if word_lower.len() < 3 || is_stop_word(&word_lower) {
            continue;
        }
        
        *word_freq.entry(word_lower).or_insert(0) += 1;
    }

    // Sort by frequency
    let mut freq_vec: Vec<(String, usize)> = word_freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Return top N
    freq_vec
        .into_iter()
        .take(top_n)
        .map(|(word, _)| word)
        .collect()
}

/// Simple stop word check (English)
fn is_stop_word(word: &str) -> bool {
    const STOP_WORDS: &[&str] = &[
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
        "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
        "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
        "or", "an", "will", "my", "one", "all", "would", "there", "their",
    ];

    STOP_WORDS.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        let text = "  This   has   extra    spaces  ";
        let normalized = normalize_text(text);
        assert_eq!(normalized, "This has extra spaces");
    }

    #[test]
    fn test_extract_sentences() {
        let text = "First sentence. Second sentence! Third sentence?";
        let sentences = extract_sentences(text);
        assert_eq!(sentences.len(), 3);
    }

    #[test]
    fn test_word_count() {
        let text = "This is a test";
        assert_eq!(word_count(text), 4);
    }

    #[test]
    fn test_extract_keywords() {
        let text = "game theory game theory applications theory practice game";
        let keywords = extract_keywords(text, 3);
        assert!(keywords.contains(&"game".to_string()));
        assert!(keywords.contains(&"theory".to_string()));
    }
}
