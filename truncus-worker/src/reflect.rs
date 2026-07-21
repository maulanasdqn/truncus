use crate::ai::AiService;
use crate::db::Store;
use std::collections::HashSet;
use worker::{console_log, Env, Result};

const MAX_LESSONS: usize = 5;

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
    let drafts = parse_drafts(&raw);
    console_log!(
        "reflect {session_id}: {} chars -> {} drafts",
        raw.len(),
        drafts.len()
    );
    let mut seen = HashSet::new();
    let mut count = 0;
    for draft in drafts.into_iter().take(MAX_LESSONS) {
        let id = lesson_id(project, &draft.title);
        if !seen.insert(id.clone()) {
            continue;
        }
        store
            .upsert_lesson(
                &id,
                project,
                &draft.category,
                &draft.title,
                &draft.insight,
                session_id,
                ts,
            )
            .await?;
        count += 1;
    }
    Ok(count)
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
    console_log!("backfill: {} sessions", metas.len());
    for meta in metas {
        if meta.status != "ready" {
            continue;
        }
        if let Some(summary) = meta.summary.as_deref() {
            match reflect_session(&store, &ai, &meta.project, &meta.id, summary, meta.ended_at).await
            {
                Ok(count) => console_log!("reflected {} -> {} lessons", meta.id, count),
                Err(error) => console_log!("reflect error {}: {}", meta.id, error),
            }
        }
    }
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
