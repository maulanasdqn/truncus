use truncus_core::transcript::parse_jsonl;

const SAMPLE: &str = r#"{"type":"mode","mode":"normal","sessionId":"s1"}
{"type":"user","isSidechain":false,"timestamp":"2026-07-01T04:38:56.798Z","message":{"role":"user","content":"fix the login bug"}}
{"type":"assistant","isSidechain":false,"timestamp":"2026-07-01T04:39:59.913Z","message":{"role":"assistant","content":[{"type":"thinking","thinking":"hmm"},{"type":"text","text":"I found the bug in auth.rs"},{"type":"tool_use","name":"Read","input":{}}]}}
{"type":"user","isSidechain":false,"timestamp":"2026-07-01T04:40:10.000Z","message":{"role":"user","content":[{"type":"tool_result","content":"file contents"}]}}
{"type":"user","isSidechain":true,"timestamp":"2026-07-01T04:40:20.000Z","message":{"role":"user","content":"sidechain noise"}}
{"type":"user","isMeta":true,"timestamp":"2026-07-01T04:40:30.000Z","message":{"role":"user","content":"meta noise"}}
{"type":"user","isSidechain":false,"timestamp":"2026-07-01T04:41:00.000Z","message":{"role":"user","content":"<system-reminder>ignore me</system-reminder>"}}
{"type":"assistant","isSidechain":false,"timestamp":"2026-07-01T04:42:00.000Z","message":{"role":"assistant","content":[{"type":"text","text":"Fixed and tests pass"}]}}
not even json
"#;

#[test]
fn parses_text_and_skips_noise() {
    let transcript = parse_jsonl(SAMPLE);
    let texts: Vec<&str> = transcript.messages.iter().map(|m| m.text.as_str()).collect();
    assert_eq!(
        texts,
        vec![
            "fix the login bug",
            "I found the bug in auth.rs",
            "Fixed and tests pass"
        ]
    );
    assert_eq!(transcript.messages[0].role, "user");
    assert_eq!(transcript.messages[1].role, "assistant");
}

#[test]
fn tracks_session_time_range() {
    let transcript = parse_jsonl(SAMPLE);
    assert_eq!(transcript.started_at, 1782880736798);
    assert_eq!(transcript.ended_at, 1782880920000);
}

#[test]
fn empty_input_is_safe() {
    let transcript = parse_jsonl("");
    assert!(transcript.messages.is_empty());
    assert_eq!(transcript.started_at, 0);
}
