use serde_json::Value;

const DEFAULT_INTERVAL_SECS: u64 = 300;

pub async fn run(payload: &Value) -> anyhow::Result<()> {
    let Some(session_id) = payload["session_id"].as_str() else {
        return Ok(());
    };
    if !due(session_id) {
        return Ok(());
    }
    crate::session_end::run(payload).await?;
    mark(session_id)?;
    Ok(())
}

fn interval_secs() -> u64 {
    std::env::var("TRUNCUS_CAPTURE_INTERVAL_SECS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS)
}

fn state_path(session_id: &str) -> std::path::PathBuf {
    let safe: String = session_id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    std::env::temp_dir().join(format!("truncus-capture-{safe}"))
}

fn due(session_id: &str) -> bool {
    std::fs::metadata(state_path(session_id))
        .and_then(|meta| meta.modified())
        .map(|modified| {
            modified
                .elapsed()
                .map(|age| age.as_secs() >= interval_secs())
                .unwrap_or(true)
        })
        .unwrap_or(true)
}

fn mark(session_id: &str) -> anyhow::Result<()> {
    std::fs::write(state_path(session_id), b"")?;
    Ok(())
}
