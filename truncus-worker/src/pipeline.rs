use crate::ai::AiService;
use crate::db::Store;
use crate::vectorize::{VectorIndex, VectorRecord};
use serde_json::json;
use std::collections::HashMap;
use truncus_core::chunk::chunk_messages;
use truncus_core::dto::{IngestRequest, Msg};
use worker::{Env, Result};

const SUMMARY_INPUT_HEAD: usize = 6000;
const SUMMARY_INPUT_TAIL: usize = 14000;
const UPSERT_BATCH: usize = 500;

pub fn summary_vector_id(session_id: &str) -> String {
    format!("{session_id}#s")
}

pub fn chunk_vector_id(session_id: &str, seq: usize) -> String {
    format!("{session_id}#c{seq}")
}

pub async fn run(env: Env, session_id: String) {
    if let Err(error) = process(&env, &session_id).await {
        if let Ok(db) = env.d1("DB") {
            let _ = Store::new(db)
                .set_status(&session_id, "failed", Some(&error.to_string()))
                .await;
        }
    }
}

async fn process(env: &Env, session_id: &str) -> Result<()> {
    let store = Store::new(env.d1("DB")?);
    store.set_status(session_id, "processing", None).await?;
    let previous_chunks = store
        .get_session(session_id)
        .await?
        .map(|meta| meta.chunk_count)
        .unwrap_or(0);

    let raw = load_raw(env, session_id).await?;
    let ai = AiService::new(env)?;
    let index = VectorIndex::new(env)?;

    let chunks = chunk_messages(&raw.messages);
    let summary = ai.summarize(&render_conversation(&raw.messages)).await?;
    let changed = changed_seqs(&store, session_id, &chunks).await?;

    let mut texts = vec![summary.clone()];
    texts.extend(changed.iter().map(|&seq| chunks[seq].clone()));
    let vectors = ai.embed(&texts).await?;

    let records = build_records(session_id, &raw, &changed, vectors);
    for batch in records.chunks(UPSERT_BATCH) {
        index.upsert(batch).await?;
    }
    delete_stale_vectors(&index, session_id, previous_chunks, chunks.len()).await?;

    store.replace_chunks(session_id, &chunks).await?;
    store
        .set_ready(session_id, &summary, chunks.len() as i64)
        .await?;
    let _ = crate::reflect::reflect_session(
        &store,
        &ai,
        &raw.project,
        session_id,
        &summary,
        raw.ended_at,
    )
    .await;
    Ok(())
}

async fn changed_seqs(
    store: &Store,
    session_id: &str,
    chunks: &[String],
) -> Result<Vec<usize>> {
    let existing: HashMap<i64, String> = store
        .chunk_seq_texts(session_id)
        .await?
        .into_iter()
        .map(|row| (row.seq, row.text))
        .collect();
    Ok((0..chunks.len())
        .filter(|&seq| existing.get(&(seq as i64)).map(String::as_str) != Some(chunks[seq].as_str()))
        .collect())
}

fn build_records(
    session_id: &str,
    raw: &IngestRequest,
    changed: &[usize],
    mut vectors: Vec<Vec<f32>>,
) -> Vec<VectorRecord> {
    let metadata = |kind: &str| {
        json!({ "project": raw.project, "kind": kind, "ts": raw.ended_at })
    };
    let mut records = Vec::with_capacity(changed.len() + 1);
    records.push(VectorRecord {
        id: summary_vector_id(session_id),
        values: vectors.remove(0),
        metadata: metadata("summary"),
    });
    for (&seq, values) in changed.iter().zip(vectors) {
        records.push(VectorRecord {
            id: chunk_vector_id(session_id, seq),
            values,
            metadata: metadata("chunk"),
        });
    }
    records
}

async fn delete_stale_vectors(
    index: &VectorIndex,
    session_id: &str,
    previous: i64,
    current: usize,
) -> Result<()> {
    let stale: Vec<String> = (current..previous.max(0) as usize)
        .map(|seq| chunk_vector_id(session_id, seq))
        .collect();
    index.delete_by_ids(&stale).await
}

async fn load_raw(env: &Env, session_id: &str) -> Result<IngestRequest> {
    let object = env
        .bucket("RAW")?
        .get(raw_key(session_id))
        .execute()
        .await?
        .ok_or_else(|| worker::Error::RustError("raw payload missing".into()))?;
    let body = object
        .body()
        .ok_or_else(|| worker::Error::RustError("raw payload empty".into()))?
        .text()
        .await?;
    serde_json::from_str(&body)
        .map_err(|e| worker::Error::RustError(format!("raw payload parse: {e}")))
}

pub fn raw_key(session_id: &str) -> String {
    format!("raw/{session_id}.json")
}

fn render_conversation(messages: &[Msg]) -> String {
    let full = messages
        .iter()
        .map(|m| format!("{}: {}", m.role, m.text))
        .collect::<Vec<_>>()
        .join("\n\n");
    if full.len() <= SUMMARY_INPUT_HEAD + SUMMARY_INPUT_TAIL {
        return full;
    }
    let head_end = floor_boundary(&full, SUMMARY_INPUT_HEAD);
    let tail_start = ceil_boundary(&full, full.len() - SUMMARY_INPUT_TAIL);
    format!("{}\n[...]\n{}", &full[..head_end], &full[tail_start..])
}

fn floor_boundary(text: &str, mut idx: usize) -> usize {
    while idx > 0 && !text.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

fn ceil_boundary(text: &str, mut idx: usize) -> usize {
    while idx < text.len() && !text.is_char_boundary(idx) {
        idx += 1;
    }
    idx
}
