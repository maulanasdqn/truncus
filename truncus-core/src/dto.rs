use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Msg {
    pub role: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestRequest {
    pub session_id: String,
    pub project: String,
    pub cwd: String,
    pub machine: String,
    pub started_at: i64,
    pub ended_at: i64,
    pub messages: Vec<Msg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResponse {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMeta {
    pub id: String,
    pub project: String,
    pub machine: String,
    pub started_at: i64,
    pub ended_at: i64,
    pub status: String,
    pub summary: Option<String>,
    pub error: Option<String>,
    pub chunk_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionList {
    pub sessions: Vec<SessionMeta>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub session_id: String,
    pub kind: String,
    pub score: f64,
    pub text: String,
    pub project: String,
    pub ended_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub hits: Vec<SearchHit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBrief {
    pub id: String,
    pub project: String,
    pub ended_at: i64,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBundle {
    pub project_sessions: Vec<SessionBrief>,
    pub other_sessions: Vec<SessionBrief>,
}
