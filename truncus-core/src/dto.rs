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
    #[serde(default)]
    pub lessons: Vec<Lesson>,
    #[serde(default)]
    pub note_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteInput {
    pub path: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMeta {
    pub path: String,
    pub title: String,
    pub content_hash: String,
    pub chunk_count: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteList {
    pub notes: Vec<NoteMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteContent {
    pub path: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotesIngest {
    pub project: String,
    pub notes: Vec<NoteInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotesIngestResponse {
    pub ingested: i64,
    pub skipped: i64,
    pub chunks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotesPrune {
    pub project: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotesRemoved {
    pub removed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteProject {
    pub project: String,
    pub note_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteProjectList {
    pub projects: Vec<NoteProject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub project: String,
    pub category: String,
    pub title: String,
    pub insight: String,
    #[serde(default)]
    pub evidence: String,
    pub confidence: f64,
    pub times_seen: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonList {
    pub lessons: Vec<Lesson>,
}
