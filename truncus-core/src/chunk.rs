use crate::dto::Msg;

pub const CHUNK_BYTES: usize = 2000;
pub const OVERLAP_BYTES: usize = 200;

pub fn chunk_messages(messages: &[Msg]) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let pieces = messages
        .iter()
        .map(|m| format!("{}: {}", m.role, m.text))
        .flat_map(|f| split_oversized(&f));
    for piece in pieces {
        if !current.is_empty() && current.len() + piece.len() + 2 > CHUNK_BYTES {
            let seed = byte_tail(&current, OVERLAP_BYTES);
            chunks.push(std::mem::take(&mut current));
            current = seed;
        }
        if !current.is_empty() {
            current.push_str("\n\n");
        }
        current.push_str(&piece);
    }
    if !current.trim().is_empty() {
        chunks.push(current);
    }
    chunks
}

fn split_oversized(text: &str) -> Vec<String> {
    if text.len() <= CHUNK_BYTES {
        return vec![text.to_string()];
    }
    let mut parts = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let end = ceil_boundary(text, (start + CHUNK_BYTES).min(text.len()));
        parts.push(text[start..end].to_string());
        if end == text.len() {
            break;
        }
        start = ceil_boundary(text, end - OVERLAP_BYTES);
    }
    parts
}

fn byte_tail(text: &str, max_bytes: usize) -> String {
    if text.len() <= max_bytes {
        return text.to_string();
    }
    text[ceil_boundary(text, text.len() - max_bytes)..].to_string()
}

fn ceil_boundary(text: &str, mut idx: usize) -> usize {
    while idx < text.len() && !text.is_char_boundary(idx) {
        idx += 1;
    }
    idx.min(text.len())
}
