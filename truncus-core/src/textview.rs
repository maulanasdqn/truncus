use crate::dto::{Lesson, SearchHit, SessionMeta};
use crate::timefmt::date;

const LOW_CONFIDENCE: f64 = 0.5;
const UNSURE: &str = "⚠ Low-confidence recall — nothing below is a strong match. Do not present \
any of it as fact; if you can't ground your answer in a high-confidence hit or the actual code, \
say you don't know rather than guessing.";

fn top_score(hits: &[SearchHit]) -> f64 {
    hits.iter().fold(0.0, |max, hit| max.max(hit.score))
}

fn with_confidence(hits: &[SearchHit], body: String) -> String {
    if top_score(hits) < LOW_CONFIDENCE {
        format!("{UNSURE}\n\n{body}")
    } else {
        body
    }
}

pub fn hits(hits: &[SearchHit]) -> String {
    if hits.is_empty() {
        return "No matching memories found. Say you don't know rather than inventing an answer."
            .into();
    }
    let body = hits
        .iter()
        .map(|hit| {
            format!(
                "[{:.3}] {} · {} · {} · session {}\n{}",
                hit.score,
                hit.kind,
                hit.project,
                date(hit.ended_at),
                hit.session_id,
                hit.text
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    with_confidence(hits, body)
}

pub fn sessions(sessions: &[SessionMeta]) -> String {
    if sessions.is_empty() {
        return "No sessions stored yet.".into();
    }
    sessions
        .iter()
        .map(|meta| {
            format!(
                "{} · {} · {} · {} · {} chunks\n{}",
                date(meta.ended_at),
                meta.project,
                meta.id,
                meta.status,
                meta.chunk_count,
                first_line(meta.summary.as_deref().unwrap_or("(no summary yet)"))
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn knowledge(hits: &[SearchHit]) -> String {
    if hits.is_empty() {
        return "No matching knowledge found. Say you don't know rather than guessing.".into();
    }
    let body = hits
        .iter()
        .map(|hit| {
            format!(
                "[{:.3}] {} · {}\n{}",
                hit.score, hit.project, hit.session_id, hit.text
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    with_confidence(hits, body)
}

pub fn lessons(lessons: &[Lesson]) -> String {
    if lessons.is_empty() {
        return "No lessons learned yet.".into();
    }
    lessons
        .iter()
        .map(|lesson| {
            format!(
                "[{}] {} · seen ×{} · {:.0}% confidence\n{}",
                lesson.category,
                lesson.title,
                lesson.times_seen,
                lesson.confidence * 100.0,
                lesson.insight
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn session(meta: &SessionMeta) -> String {
    format!(
        "session {}\nproject: {}\nmachine: {}\nrange: {} → {}\nstatus: {}{}\n\n{}",
        meta.id,
        meta.project,
        meta.machine,
        date(meta.started_at),
        date(meta.ended_at),
        meta.status,
        meta.error
            .as_deref()
            .map(|e| format!(" ({e})"))
            .unwrap_or_default(),
        meta.summary.as_deref().unwrap_or("(no summary yet)")
    )
}

fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or_default()
}
