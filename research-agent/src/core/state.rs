use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Research session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchState {
    Initialized,
    AnalyzingQuery,
    SearchingCorpus { progress: f32 },
    AnalyzingEvidence,
    ExternalSearchRequired { queries: Vec<String> },
    Synthesizing,
    GeneratingDocument,
    Complete { artifacts: Vec<String> },
    Error { message: String },
}

/// Research session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSession {
    pub id: Uuid,
    pub question: String,
    pub state: ResearchState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResearchSession {
    pub fn new(question: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            question,
            state: ResearchState::Initialized,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, ResearchState::Complete { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(self.state, ResearchState::Error { .. })
    }
}
