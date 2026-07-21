use serde::Deserialize;
use truncus_core::dto::{IngestRequest, SessionMeta};
use worker::wasm_bindgen::JsValue;
use worker::{D1Database, Result};

const STUCK_THRESHOLD_MS: i64 = 10 * 60 * 1000;

pub struct Store {
    pub(crate) db: D1Database,
}

fn now_ms() -> i64 {
    worker::Date::now().as_millis() as i64
}

#[derive(Debug, Deserialize)]
pub struct SeqText {
    pub seq: i64,
    pub text: String,
}

pub(crate) fn s(value: &str) -> JsValue {
    JsValue::from(value)
}

pub(crate) fn n(value: i64) -> JsValue {
    JsValue::from(value as f64)
}

impl Store {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }

    pub async fn upsert_pending(&self, req: &IngestRequest) -> Result<()> {
        self.db
            .prepare(
                "INSERT INTO sessions (id, project, cwd, machine, started_at, ended_at, status, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending', ?7) \
                 ON CONFLICT(id) DO UPDATE SET project=?2, cwd=?3, machine=?4, \
                 started_at=?5, ended_at=?6, status='pending', error=NULL, updated_at=?7",
            )
            .bind(&[
                s(&req.session_id),
                s(&req.project),
                s(&req.cwd),
                s(&req.machine),
                n(req.started_at),
                n(req.ended_at),
                n(now_ms()),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn set_status(&self, id: &str, status: &str, error: Option<&str>) -> Result<()> {
        self.db
            .prepare("UPDATE sessions SET status=?2, error=?3, updated_at=?4 WHERE id=?1")
            .bind(&[
                s(id),
                s(status),
                error.map(s).unwrap_or(JsValue::NULL),
                n(now_ms()),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn unstick_sessions(&self) -> Result<()> {
        let cutoff = now_ms() - STUCK_THRESHOLD_MS;
        self.db
            .prepare(
                "UPDATE sessions SET status='failed', \
                 error='reset: processing exceeded time budget (likely a Workers AI capacity timeout) — reprocess to retry', \
                 updated_at=?1 WHERE status='processing' AND updated_at > 0 AND updated_at < ?2",
            )
            .bind(&[n(now_ms()), n(cutoff)])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn set_ready(&self, id: &str, summary: &str, chunk_count: i64) -> Result<()> {
        self.db
            .prepare(
                "UPDATE sessions SET status='ready', error=NULL, summary=?2, chunk_count=?3, \
                 updated_at=?4 WHERE id=?1",
            )
            .bind(&[s(id), s(summary), n(chunk_count), n(now_ms())])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn replace_chunks(&self, session_id: &str, chunks: &[String]) -> Result<()> {
        let delete = self
            .db
            .prepare("DELETE FROM chunks WHERE session_id=?1")
            .bind(&[s(session_id)])?;
        let mut statements = vec![delete];
        for (seq, text) in chunks.iter().enumerate() {
            let id = format!("{session_id}#c{seq}");
            statements.push(
                self.db
                    .prepare("INSERT INTO chunks (id, session_id, seq, text) VALUES (?1, ?2, ?3, ?4)")
                    .bind(&[s(&id), s(session_id), n(seq as i64), s(text)])?,
            );
        }
        self.db.batch(statements).await?;
        Ok(())
    }

    pub async fn get_session(&self, id: &str) -> Result<Option<SessionMeta>> {
        self.db
            .prepare(
                "SELECT id, project, machine, started_at, ended_at, status, summary, error, \
                 chunk_count FROM sessions WHERE id=?1",
            )
            .bind(&[s(id)])?
            .first::<SessionMeta>(None)
            .await
    }

    pub async fn chunk_seq_texts(&self, session_id: &str) -> Result<Vec<SeqText>> {
        self.db
            .prepare("SELECT seq, text FROM chunks WHERE session_id=?1")
            .bind(&[s(session_id)])?
            .all()
            .await?
            .results()
    }

    pub async fn delete_session(&self, id: &str) -> Result<()> {
        let statements = vec![
            self.db
                .prepare("DELETE FROM chunks WHERE session_id=?1")
                .bind(&[s(id)])?,
            self.db
                .prepare("DELETE FROM sessions WHERE id=?1")
                .bind(&[s(id)])?,
        ];
        self.db.batch(statements).await?;
        Ok(())
    }
}
