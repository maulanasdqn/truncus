use crate::db::{n, s, Store};
use crate::db_read::{placeholders, CountRow};
use serde::Deserialize;
use truncus_core::dto::{NoteContent, NoteMeta, NoteProject};
use worker::wasm_bindgen::JsValue;
use worker::Result;

#[derive(Debug, Deserialize)]
pub struct NoteRef {
    pub id: String,
    pub path: String,
    pub chunk_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct NoteHit {
    pub chunk_id: String,
    pub path: String,
    pub title: String,
    pub project: String,
    pub updated_at: i64,
    pub text: String,
}

#[derive(Debug, Deserialize)]
struct NoteState {
    content_hash: String,
    chunk_count: i64,
}

impl Store {
    pub async fn note_state(&self, id: &str) -> Result<Option<(String, i64)>> {
        Ok(self
            .db
            .prepare("SELECT content_hash, chunk_count FROM notes WHERE id=?1")
            .bind(&[s(id)])?
            .first::<NoteState>(None)
            .await?
            .map(|row| (row.content_hash, row.chunk_count)))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn upsert_note(
        &self,
        id: &str,
        project: &str,
        path: &str,
        title: &str,
        content_hash: &str,
        chunk_count: i64,
        ts: i64,
        content: &str,
    ) -> Result<()> {
        self.db
            .prepare(
                "INSERT INTO notes (id, project, path, title, content_hash, chunk_count, updated_at, content) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) \
                 ON CONFLICT(id) DO UPDATE SET title=?4, content_hash=?5, chunk_count=?6, \
                 updated_at=?7, content=?8",
            )
            .bind(&[
                s(id),
                s(project),
                s(path),
                s(title),
                s(content_hash),
                n(chunk_count),
                n(ts),
                s(content),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn set_note_content(&self, project: &str, path: &str, content: &str) -> Result<()> {
        self.db
            .prepare("UPDATE notes SET content=?3 WHERE project=?1 AND path=?2")
            .bind(&[s(project), s(path), s(content)])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_note_content(
        &self,
        project: &str,
        path: &str,
    ) -> Result<Option<NoteContent>> {
        self.db
            .prepare("SELECT path, title, content FROM notes WHERE project=?1 AND path=?2")
            .bind(&[s(project), s(path)])?
            .first::<NoteContent>(None)
            .await
    }

    pub async fn replace_note_chunks(&self, note_id: &str, chunks: &[String]) -> Result<()> {
        let delete = self
            .db
            .prepare("DELETE FROM note_chunks WHERE note_id=?1")
            .bind(&[s(note_id)])?;
        let mut statements = vec![delete];
        for (seq, text) in chunks.iter().enumerate() {
            let id = format!("{note_id}#n{seq}");
            statements.push(
                self.db
                    .prepare(
                        "INSERT INTO note_chunks (id, note_id, seq, text) VALUES (?1, ?2, ?3, ?4)",
                    )
                    .bind(&[s(&id), s(note_id), n(seq as i64), s(text)])?,
            );
        }
        self.db.batch(statements).await?;
        Ok(())
    }

    pub async fn list_notes(&self, project: &str) -> Result<Vec<NoteMeta>> {
        self.db
            .prepare(
                "SELECT path, title, content_hash, chunk_count, updated_at FROM notes \
                 WHERE project=?1 ORDER BY path",
            )
            .bind(&[s(project)])?
            .all()
            .await?
            .results()
    }

    pub async fn note_projects(&self) -> Result<Vec<NoteProject>> {
        self.db
            .prepare(
                "SELECT project, COUNT(*) AS note_count FROM notes \
                 GROUP BY project ORDER BY project",
            )
            .bind(&[])?
            .all()
            .await?
            .results()
    }

    pub async fn note_count(&self, project: &str) -> Result<i64> {
        Ok(self
            .db
            .prepare("SELECT COUNT(*) AS n FROM notes WHERE project=?1")
            .bind(&[s(project)])?
            .first::<CountRow>(None)
            .await?
            .map(|row| row.n)
            .unwrap_or(0))
    }

    pub async fn note_refs(&self, project: &str) -> Result<Vec<NoteRef>> {
        self.db
            .prepare("SELECT id, path, chunk_count FROM notes WHERE project=?1")
            .bind(&[s(project)])?
            .all()
            .await?
            .results()
    }

    pub async fn delete_note(&self, id: &str) -> Result<()> {
        let statements = vec![
            self.db
                .prepare("DELETE FROM note_chunks WHERE note_id=?1")
                .bind(&[s(id)])?,
            self.db
                .prepare("DELETE FROM notes WHERE id=?1")
                .bind(&[s(id)])?,
        ];
        self.db.batch(statements).await?;
        Ok(())
    }

    pub async fn hydrate_note_chunks(&self, ids: &[String]) -> Result<Vec<NoteHit>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let sql = format!(
            "SELECT nc.id AS chunk_id, n.path AS path, n.title AS title, n.project AS project, \
             n.updated_at AS updated_at, nc.text AS text FROM note_chunks nc \
             JOIN notes n ON n.id = nc.note_id WHERE nc.id IN ({})",
            placeholders(ids.len())
        );
        let binds: Vec<JsValue> = ids.iter().map(|id| s(id)).collect();
        self.db.prepare(&sql).bind(&binds)?.all().await?.results()
    }
}
