use crate::dto::Msg;
use serde_json::Value;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

pub const MAX_MSG_CHARS: usize = 8000;

#[derive(Debug, Default)]
pub struct Transcript {
    pub messages: Vec<Msg>,
    pub started_at: i64,
    pub ended_at: i64,
}

pub fn parse_jsonl(content: &str) -> Transcript {
    let mut transcript = Transcript::default();
    for line in content.lines() {
        let Ok(entry) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if !is_conversation_entry(&entry) {
            continue;
        }
        if let Some(ts) = timestamp_millis(&entry) {
            if transcript.started_at == 0 {
                transcript.started_at = ts;
            }
            transcript.ended_at = ts;
        }
        let Some(text) = extract_text(&entry["message"]["content"]) else {
            continue;
        };
        let role = entry["message"]["role"]
            .as_str()
            .unwrap_or("user")
            .to_string();
        transcript.messages.push(Msg {
            role,
            text: truncate_chars(&text, MAX_MSG_CHARS),
        });
    }
    transcript
}

fn is_conversation_entry(entry: &Value) -> bool {
    matches!(entry["type"].as_str(), Some("user") | Some("assistant"))
        && !entry["isSidechain"].as_bool().unwrap_or(false)
        && !entry["isMeta"].as_bool().unwrap_or(false)
}

fn extract_text(content: &Value) -> Option<String> {
    let text = match content {
        Value::String(s) => s.clone(),
        Value::Array(blocks) => blocks
            .iter()
            .filter(|b| b["type"] == "text")
            .filter_map(|b| b["text"].as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => return None,
    };
    let trimmed = text.trim();
    let skip = trimmed.is_empty()
        || trimmed.starts_with("<system-reminder>")
        || trimmed.starts_with("<command-name>")
        || trimmed.starts_with("<local-command")
        || trimmed.starts_with("Caveat: ");
    (!skip).then(|| trimmed.to_string())
}

fn timestamp_millis(entry: &Value) -> Option<i64> {
    let raw = entry["timestamp"].as_str()?;
    let parsed = OffsetDateTime::parse(raw, &Rfc3339).ok()?;
    Some((parsed.unix_timestamp_nanos() / 1_000_000) as i64)
}

pub fn truncate_chars(text: &str, max: usize) -> String {
    match text.char_indices().nth(max) {
        Some((idx, _)) => text[..idx].to_string(),
        None => text.to_string(),
    }
}
