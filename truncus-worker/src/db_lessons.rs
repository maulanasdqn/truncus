use crate::db::{n, s, Store};
use serde::Deserialize;
use truncus_core::dto::Lesson;
use worker::Result;

const LESSON_COLUMNS: &str = "id, project, category, title, insight, evidence, confidence, \
     times_seen, created_at, updated_at";

const REINFORCE_SET: &str = "times_seen = times_seen + \
     (CASE WHEN instr(evidence, ?SESSION) > 0 THEN 0 ELSE 1 END), \
     confidence = CASE WHEN instr(evidence, ?SESSION) > 0 THEN confidence \
     ELSE MIN(1.0, confidence + 0.15) END, \
     evidence = CASE WHEN evidence = '' THEN ?SESSION \
     WHEN instr(evidence, ?SESSION) > 0 THEN evidence ELSE evidence || ',' || ?SESSION END";

const DECAY_GRACE_MS: i64 = 3 * 24 * 60 * 60 * 1000;
const DECAY_STEP: f64 = 0.05;
const PRUNE_FLOOR: f64 = 0.2;

#[derive(Debug, Deserialize)]
struct VectorRow {
    id: String,
    embedding: String,
}

impl Store {
    pub async fn upsert_lesson(
        &self,
        id: &str,
        project: &str,
        category: &str,
        title: &str,
        insight: &str,
        session_id: &str,
        ts: i64,
        embedding: &str,
    ) -> Result<()> {
        let set = REINFORCE_SET.replace("?SESSION", "?6");
        let sql = format!(
            "INSERT INTO lessons (id, project, category, title, insight, evidence, confidence, \
             times_seen, created_at, updated_at, embedding) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0.5, 1, ?7, ?7, ?8) \
             ON CONFLICT(id) DO UPDATE SET {set}, updated_at = ?7"
        );
        self.db
            .prepare(&sql)
            .bind(&[
                s(id),
                s(project),
                s(category),
                s(title),
                s(insight),
                s(session_id),
                n(ts),
                s(embedding),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn reinforce_lesson(&self, id: &str, session_id: &str, ts: i64) -> Result<()> {
        let set = REINFORCE_SET.replace("?SESSION", "?2");
        let sql = format!("UPDATE lessons SET {set}, updated_at = ?3 WHERE id = ?1");
        self.db
            .prepare(&sql)
            .bind(&[s(id), s(session_id), n(ts)])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn lesson_vectors(&self, project: &str) -> Result<Vec<(String, Vec<f32>)>> {
        let rows: Vec<VectorRow> = self
            .db
            .prepare("SELECT id, embedding FROM lessons WHERE project=?1 AND embedding <> ''")
            .bind(&[s(project)])?
            .all()
            .await?
            .results()?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                serde_json::from_str::<Vec<f32>>(&row.embedding)
                    .ok()
                    .map(|values| (row.id, values))
            })
            .collect())
    }

    pub async fn decay_lessons(&self, now: i64) -> Result<()> {
        let cutoff = now - DECAY_GRACE_MS;
        let decay = self
            .db
            .prepare(&format!(
                "UPDATE lessons SET confidence = MAX(0.0, confidence - {DECAY_STEP}) \
                 WHERE updated_at < ?1"
            ))
            .bind(&[n(cutoff)])?;
        let prune = self
            .db
            .prepare(&format!("DELETE FROM lessons WHERE confidence < {PRUNE_FLOOR}"))
            .bind(&[])?;
        self.db.batch(vec![decay, prune]).await?;
        Ok(())
    }

    pub async fn list_lessons(&self, project: Option<&str>, limit: usize) -> Result<Vec<Lesson>> {
        let (sql, binds) = match project {
            Some(p) => (
                format!(
                    "SELECT {LESSON_COLUMNS} FROM lessons WHERE project=?1 \
                     ORDER BY confidence DESC, updated_at DESC LIMIT ?2"
                ),
                vec![s(p), n(limit as i64)],
            ),
            None => (
                format!(
                    "SELECT {LESSON_COLUMNS} FROM lessons \
                     ORDER BY confidence DESC, updated_at DESC LIMIT ?1"
                ),
                vec![n(limit as i64)],
            ),
        };
        self.db.prepare(&sql).bind(&binds)?.all().await?.results()
    }

    pub async fn delete_lesson(&self, id: &str) -> Result<()> {
        self.db
            .prepare("DELETE FROM lessons WHERE id=?1")
            .bind(&[s(id)])?
            .run()
            .await?;
        Ok(())
    }
}
