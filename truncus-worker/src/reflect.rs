use crate::ai::AiService;
use crate::db::Store;
use worker::{console_log, Env, Result};

const MAX_LESSONS: usize = 5;
const DEDUP_THRESHOLD: f32 = 0.85;

struct LessonDraft {
    category: String,
    title: String,
    insight: String,
}

pub async fn reflect_session(
    store: &Store,
    ai: &AiService,
    project: &str,
    session_id: &str,
    summary: &str,
    ts: i64,
) -> Result<usize> {
    if summary.trim().is_empty() {
        return Ok(0);
    }
    let raw = ai.reflect(project, summary).await?;
    let mut drafts = parse_drafts(&raw);
    drafts.truncate(MAX_LESSONS);
    if drafts.is_empty() {
        return Ok(0);
    }

    let texts: Vec<String> = drafts
        .iter()
        .map(|draft| format!("{}\n{}", draft.title, draft.insight))
        .collect();
    let embeddings = ai.embed(&texts).await?;
    let mut vectors = store.lesson_vectors(project).await?;

    let mut new_count = 0;
    let mut reinforced = 0;
    for (draft, embedding) in drafts.into_iter().zip(embeddings.into_iter()) {
        match best_match(&vectors, &embedding) {
            Some((id, score)) if score >= DEDUP_THRESHOLD => {
                store.reinforce_lesson(&id, session_id, ts).await?;
                reinforced += 1;
            }
            _ => {
                let id = lesson_id(project, &draft.title);
                let embedding_json = serde_json::to_string(&embedding).unwrap_or_default();
                store
                    .upsert_lesson(
                        &id,
                        project,
                        &draft.category,
                        &draft.title,
                        &draft.insight,
                        session_id,
                        ts,
                        &embedding_json,
                    )
                    .await?;
                vectors.push((id, embedding));
                new_count += 1;
            }
        }
    }
    console_log!(
        "reflect {session_id}: {new_count} new, {reinforced} reinforced"
    );
    Ok(new_count)
}

pub async fn backfill(env: Env, session: Option<String>, project: Option<String>, limit: usize) {
    let Ok(db) = env.d1("DB") else {
        return;
    };
    let store = Store::new(db);
    let Ok(ai) = AiService::new(&env) else {
        return;
    };
    let metas = match &session {
        Some(id) => store
            .get_session(id)
            .await
            .ok()
            .flatten()
            .into_iter()
            .collect::<Vec<_>>(),
        None => store
            .list_sessions(project.as_deref(), limit, 0)
            .await
            .unwrap_or_default(),
    };
    for meta in metas {
        if meta.status != "ready" {
            continue;
        }
        if let Some(summary) = meta.summary.as_deref() {
            if let Err(error) =
                reflect_session(&store, &ai, &meta.project, &meta.id, summary, meta.ended_at).await
            {
                console_log!("reflect error {}: {}", meta.id, error);
            }
        }
    }
}

fn best_match(vectors: &[(String, Vec<f32>)], embedding: &[f32]) -> Option<(String, f32)> {
    vectors
        .iter()
        .map(|(id, values)| (id.clone(), cosine(embedding, values)))
        .fold(None, |best, current| match best {
            Some((_, score)) if score >= current.1 => best,
            _ => Some(current),
        })
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for i in 0..len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a.sqrt() * norm_b.sqrt())
}

fn parse_drafts(raw: &str) -> Vec<LessonDraft> {
    let (Some(start), Some(end)) = (raw.find('['), raw.rfind(']')) else {
        return Vec::new();
    };
    if end <= start {
        return Vec::new();
    }
    let values: Vec<serde_json::Value> = serde_json::from_str(&raw[start..=end]).unwrap_or_default();
    values
        .into_iter()
        .filter_map(|value| {
            let title = value.get("title")?.as_str()?.trim().to_string();
            let insight = value.get("insight")?.as_str()?.trim().to_string();
            if title.is_empty() || insight.is_empty() {
                return None;
            }
            let category = value
                .get("category")
                .and_then(|c| c.as_str())
                .unwrap_or("insight")
                .trim()
                .to_string();
            Some(LessonDraft {
                category,
                title,
                insight,
            })
        })
        .collect()
}

fn lesson_id(project: &str, title: &str) -> String {
    let key = format!("{}\n{}", project.trim().to_lowercase(), normalize(title));
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in key.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn normalize(title: &str) -> String {
    title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
        .trim_end_matches(['.', ':', '!', '?'])
        .to_string()
}
