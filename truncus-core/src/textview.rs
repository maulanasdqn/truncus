use crate::dto::{SearchHit, SessionMeta};
use crate::timefmt::date;

pub fn hits(hits: &[SearchHit]) -> String {
    if hits.is_empty() {
        return "No matching memories found.".into();
    }
    hits.iter()
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
        .join("\n\n")
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
