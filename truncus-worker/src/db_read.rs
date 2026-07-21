use crate::db::{n, s, Store};
use serde::Deserialize;
use truncus_core::dto::{SessionBrief, SessionMeta};
use worker::wasm_bindgen::JsValue;
use worker::Result;

#[derive(Debug, Deserialize)]
pub struct ChunkHydration {
    pub id: String,
    pub session_id: String,
    pub text: String,
    pub project: String,
    pub ended_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CountRow {
    pub n: i64,
}

impl Store {
    pub async fn list_sessions(
        &self,
        project: Option<&str>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<SessionMeta>> {
        let (sql, binds) = match project {
            Some(p) => (
                "SELECT id, project, machine, started_at, ended_at, status, summary, error, \
                 chunk_count FROM sessions WHERE project=?1 ORDER BY ended_at DESC \
                 LIMIT ?2 OFFSET ?3",
                vec![s(p), n(limit as i64), n(offset as i64)],
            ),
            None => (
                "SELECT id, project, machine, started_at, ended_at, status, summary, error, \
                 chunk_count FROM sessions ORDER BY ended_at DESC LIMIT ?1 OFFSET ?2",
                vec![n(limit as i64), n(offset as i64)],
            ),
        };
        self.db.prepare(sql).bind(&binds)?.all().await?.results()
    }

    pub async fn count_sessions(&self, project: Option<&str>) -> Result<i64> {
        let (sql, binds) = match project {
            Some(p) => (
                "SELECT COUNT(*) AS n FROM sessions WHERE project=?1",
                vec![s(p)],
            ),
            None => ("SELECT COUNT(*) AS n FROM sessions", Vec::new()),
        };
        Ok(self
            .db
            .prepare(sql)
            .bind(&binds)?
            .first::<CountRow>(None)
            .await?
            .map(|row| row.n)
            .unwrap_or(0))
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

pub(crate) fn placeholders(count: usize) -> String {
    (1..=count)
        .map(|i| format!("?{i}"))
        .collect::<Vec<_>>()
        .join(", ")
}
