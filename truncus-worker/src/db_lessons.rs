use crate::db::{n, s, Store};
use truncus_core::dto::Lesson;
use worker::Result;

const LESSON_COLUMNS: &str = "id, project, category, title, insight, evidence, confidence, \
     times_seen, created_at, updated_at";

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
    ) -> Result<()> {
        self.db
            .prepare(
                "INSERT INTO lessons (id, project, category, title, insight, evidence, \
                 confidence, times_seen, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0.5, 1, ?7, ?7) \
                 ON CONFLICT(id) DO UPDATE SET \
                 times_seen = times_seen + 1, \
                 confidence = MIN(1.0, confidence + 0.15), \
                 category = excluded.category, \
                 title = excluded.title, \
                 insight = excluded.insight, \
                 evidence = CASE WHEN evidence = '' THEN excluded.evidence \
                 WHEN instr(evidence, excluded.evidence) > 0 THEN evidence \
                 ELSE evidence || ',' || excluded.evidence END, \
                 updated_at = excluded.updated_at",
            )
            .bind(&[
                s(id),
                s(project),
                s(category),
                s(title),
                s(insight),
                s(session_id),
                n(ts),
            ])?
            .run()
            .await?;
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
