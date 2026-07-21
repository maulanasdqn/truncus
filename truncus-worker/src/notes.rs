use crate::ai::AiService;
use crate::db::Store;
use crate::db_notes::NoteRef;
use crate::vectorize::{VectorIndex, VectorRecord};
use serde_json::json;
use std::collections::HashSet;
use truncus_core::chunk::chunk_text;
use truncus_core::dto::{NoteInput, NotesIngestResponse};
use worker::{Env, Result};

const UPSERT_BATCH: usize = 500;

pub fn note_id(project: &str, path: &str) -> String {
    fnv(&format!("{project}\n{path}"))
}

fn content_hash(content: &str) -> String {
    fnv(content)
}

fn fnv(input: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn vector_id(note_id: &str, seq: usize) -> String {
    format!("{note_id}#n{seq}")
}

pub async fn ingest(
    env: &Env,
    project: &str,
    notes: &[NoteInput],
    ts: i64,
) -> Result<NotesIngestResponse> {
    let store = Store::new(env.d1("DB")?);
    let ai = AiService::new(env)?;
    let index = VectorIndex::new(env)?;
    let mut ingested = 0;
    let mut skipped = 0;
    let mut chunk_total = 0;
    for note in notes {
        let id = note_id(project, &note.path);
        let hash = content_hash(&note.content);
        let state = store.note_state(&id).await?;
        if state.as_ref().map(|(h, _)| h.as_str()) == Some(hash.as_str()) {
            store
                .set_note_content(project, &note.path, &note.content)
                .await?;
            skipped += 1;
            continue;
        }
        let previous = state.map(|(_, count)| count).unwrap_or(0);
        let chunks = chunk_text(&note.content);
        if chunks.is_empty() {
            skipped += 1;
            continue;
        }
        let vectors = ai.embed(&chunks).await?;
        let records: Vec<VectorRecord> = vectors
            .into_iter()
            .enumerate()
            .map(|(seq, values)| VectorRecord {
                id: vector_id(&id, seq),
                values,
                metadata: json!({ "project": project, "kind": "note", "ts": ts }),
            })
            .collect();
        for batch in records.chunks(UPSERT_BATCH) {
            index.upsert(batch).await?;
        }
        delete_stale(&index, &id, previous, chunks.len()).await?;
        store.replace_note_chunks(&id, &chunks).await?;
        store
            .upsert_note(
                &id,
                project,
                &note.path,
                &note.title,
                &hash,
                chunks.len() as i64,
                ts,
                &note.content,
            )
            .await?;
        ingested += 1;
        chunk_total += chunks.len() as i64;
    }
    Ok(NotesIngestResponse {
        ingested,
        skipped,
        chunks: chunk_total,
    })
}

pub async fn prune(env: &Env, project: &str, keep: &[String]) -> Result<i64> {
    let store = Store::new(env.d1("DB")?);
    let index = VectorIndex::new(env)?;
    let keep_set: HashSet<&str> = keep.iter().map(String::as_str).collect();
    let mut removed = 0;
    for note in store.note_refs(project).await? {
        if keep_set.contains(note.path.as_str()) {
            continue;
        }
        remove_note(&store, &index, &note).await?;
        removed += 1;
    }
    Ok(removed)
}

pub async fn clear(env: &Env, project: &str) -> Result<i64> {
    let store = Store::new(env.d1("DB")?);
    let index = VectorIndex::new(env)?;
    let refs = store.note_refs(project).await?;
    let removed = refs.len() as i64;
    for note in &refs {
        remove_note(&store, &index, note).await?;
    }
    Ok(removed)
}

async fn remove_note(store: &Store, index: &VectorIndex, note: &NoteRef) -> Result<()> {
    let vectors: Vec<String> = (0..note.chunk_count.max(0) as usize)
        .map(|seq| vector_id(&note.id, seq))
        .collect();
    index.delete_by_ids(&vectors).await?;
    store.delete_note(&note.id).await?;
    Ok(())
}

async fn delete_stale(
    index: &VectorIndex,
    note_id: &str,
    previous: i64,
    current: usize,
) -> Result<()> {
    let stale: Vec<String> = (current..previous.max(0) as usize)
        .map(|seq| vector_id(note_id, seq))
        .collect();
    index.delete_by_ids(&stale).await
}
