use truncus_core::chunk::{chunk_messages, CHUNK_BYTES, OVERLAP_BYTES};
use truncus_core::dto::Msg;

fn msg(role: &str, text: &str) -> Msg {
    Msg {
        role: role.into(),
        text: text.into(),
    }
}

#[test]
fn empty_input_yields_no_chunks() {
    assert!(chunk_messages(&[]).is_empty());
}

#[test]
fn short_conversation_fits_one_chunk() {
    let chunks = chunk_messages(&[msg("user", "hello"), msg("assistant", "hi there")]);
    assert_eq!(chunks.len(), 1);
    assert!(chunks[0].contains("user: hello"));
    assert!(chunks[0].contains("assistant: hi there"));
}

#[test]
fn long_conversation_splits_with_overlap() {
    let messages: Vec<Msg> = (0..20)
        .map(|i| msg("user", &format!("message number {i} {}", "x".repeat(300))))
        .collect();
    let chunks = chunk_messages(&messages);
    assert!(chunks.len() > 1);
    for chunk in &chunks {
        assert!(chunk.len() <= CHUNK_BYTES + OVERLAP_BYTES + 2);
    }
    let tail: String = chunks[0].chars().rev().take(50).collect();
    let reversed_tail: String = tail.chars().rev().collect();
    assert!(chunks[1].starts_with(reversed_tail.as_str()));
}

#[test]
fn oversized_single_message_is_split() {
    let chunks = chunk_messages(&[msg("assistant", &"y".repeat(CHUNK_BYTES * 3))]);
    assert!(chunks.len() >= 3);
}

#[test]
fn multibyte_text_never_panics() {
    let chunks = chunk_messages(&[msg("user", &"héllo wörld 日本語 ".repeat(500))]);
    assert!(!chunks.is_empty());
}
