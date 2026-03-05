use anyhow::Result;
use tracing::info;

use super::state::ResearchSession;

/// Main research agent orchestrator
pub struct ResearchAgent {
    // Future: Add configuration, dependencies, etc.
}

impl ResearchAgent {
    /// Create a new research agent instance
    pub async fn new() -> Result<Self> {
        info!("Initializing Research Agent");
        Ok(Self {})
    }

    /// Start a new research session
    pub async fn start_research(&self, _question: &str) -> Result<ResearchSession> {
        todo!("Full research pipeline not yet implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        let agent = ResearchAgent::new().await;
        assert!(agent.is_ok());
    }
}
