use anyhow::Context;
use serde_json::Value;
use truncus_core::client::ApiClient;
use truncus_core::config::Config;
use truncus_core::dto::IngestRequest;
use truncus_core::project::project_from_cwd;
use truncus_core::transcript::parse_jsonl;

pub async fn run(payload: &Value) -> anyhow::Result<()> {
    let session_id = payload["session_id"]
        .as_str()
        .context("session_id missing")?;
    let transcript_path = payload["transcript_path"]
        .as_str()
        .context("transcript_path missing")?;
    let cwd = payload["cwd"].as_str().unwrap_or(".");

    let content = std::fs::read_to_string(transcript_path)
        .with_context(|| format!("reading {transcript_path}"))?;
    let transcript = parse_jsonl(&content);
    if transcript.messages.is_empty() {
        return Ok(());
    }

    let request = IngestRequest {
        session_id: session_id.to_string(),
        project: project_from_cwd(cwd),
        cwd: cwd.to_string(),
        machine: gethostname::gethostname().to_string_lossy().to_string(),
        started_at: transcript.started_at,
        ended_at: transcript.ended_at,
        messages: transcript.messages,
    };
    let client = ApiClient::new(&Config::load()?);
    let response = client.ingest(&request).await?;
    eprintln!("truncus: session {} accepted ({})", response.id, response.status);
    Ok(())
}
