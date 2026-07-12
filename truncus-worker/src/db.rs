use serde::Deserialize;
use truncus_core::dto::{IngestRequest, SessionBrief, SessionMeta};
use worker::wasm_bindgen::JsValue;
use worker::{D1Database, Result};

pub struct Store {
    db: D1Database,
}

#[derive(Debug, Deserialize)]
pub struct ChunkHydration {
    pub id: String,
    pub session_id: String,
    pub text: String,
    pub project: String,
    pub ended_at: i64,
}

fn s(value: &str) -> JsValue {
    JsValue::from(value)
}

fn n(value: i64) -> JsValue {
    JsValue::from(value as f64)
}

impl Store {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }

    pub async fn upsert_pending(&self, req: &IngestRequest) -> Result<()> {
        self.db
            .prepare(
                "INSERT INTO sessions (id, project, cwd, machine, started_at, ended_at, status) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending') \
                 ON CONFLICT(id) DO UPDATE SET project=?2, cwd=?3, machine=?4, \
                 started_at=?5, ended_at=?6, status='pending', error=NULL",
            )
            .bind(&[
                s(&req.session_id),
                s(&req.project),
                s(&req.cwd),
                s(&req.machine),
                n(req.started_at),
                n(req.ended_at),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn set_status(&self, id: &str, status: &str, error: Option<&str>) -> Result<()> {
        self.db
            .prepare("UPDATE sessions SET status=?2, error=?3 WHERE id=?1")
            .bind(&[s(id), s(status), error.map(s).unwrap_or(JsValue::NULL)])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn set_ready(&self, id: &str, summary: &str, chunk_count: i64) -> Result<()> {
        self.db
            .prepare(
                "UPDATE sessions SET status='ready', error=NULL, summary=?2, chunk_count=?3 \
                 WHERE id=?1",
            )
            .bind(&[s(id), s(summary), n(chunk_count)])?
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

    pub async fn list_sessions(
        &self,
        project: Option<&str>,
        limit: usize,
    ) -> Result<Vec<SessionMeta>> {
        let (sql, binds) = match project {
            Some(p) => (
                "SELECT id, project, machine, started_at, ended_at, status, summary, error, \
                 chunk_count FROM sessions WHERE project=?1 ORDER BY ended_at DESC LIMIT ?2",
                vec![s(p), n(limit as i64)],
            ),
            None => (
                "SELECT id, project, machine, started_at, ended_at, status, summary, error, \
                 chunk_count FROM sessions ORDER BY ended_at DESC LIMIT ?1",
                vec![n(limit as i64)],
            ),
        };
        self.db.prepare(sql).bind(&binds)?.all().await?.results()
    }

    pub async fn recent_briefs(
        &self,
        project: &str,
        include: bool,
        limit: usize,
    ) -> Result<Vec<SessionBrief>> {
        let comparator = if include { "=" } else { "!=" };
        let sql = format!(
            "SELECT id, project, ended_at, summary FROM sessions \
             WHERE status='ready' AND summary IS NOT NULL AND project {comparator} ?1 \
             ORDER BY ended_at DESC LIMIT ?2"
        );
        self.db
            .prepare(&sql)
            .bind(&[s(project), n(limit as i64)])?
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

    pub async fn hydrate_chunks(&self, ids: &[String]) -> Result<Vec<ChunkHydration>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let placeholders = placeholders(ids.len());
        let sql = format!(
            "SELECT c.id, c.session_id, c.text, s.project, s.ended_at FROM chunks c \
             JOIN sessions s ON s.id = c.session_id WHERE c.id IN ({placeholders})"
        );
        let binds: Vec<JsValue> = ids.iter().map(|id| s(id)).collect();
        self.db.prepare(&sql).bind(&binds)?.all().await?.results()
    }

    pub async fn hydrate_summaries(&self, ids: &[String]) -> Result<Vec<SessionBrief>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let placeholders = placeholders(ids.len());
        let sql = format!(
            "SELECT id, project, ended_at, summary FROM sessions \
             WHERE summary IS NOT NULL AND id IN ({placeholders})"
        );
        let binds: Vec<JsValue> = ids.iter().map(|id| s(id)).collect();
        self.db.prepare(&sql).bind(&binds)?.all().await?.results()
    }
}

fn placeholders(count: usize) -> String {
    (1..=count)
        .map(|i| format!("?{i}"))
        .collect::<Vec<_>>()
        .join(", ")
}
