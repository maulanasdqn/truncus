use anyhow::Context;
use serde_json::{json, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use truncus_core::config::Config;

pub fn run(url: Option<String>, token: Option<String>) -> anyhow::Result<()> {
    let url = resolve(url, "Worker URL (e.g. https://truncus.<account>.workers.dev): ")?;
    let token = resolve(token, "API token (TRUNCUS_API_TOKEN): ")?;
    let config = Config { url, token };
    let config_path = config.save()?;
    println!("config written to {}", config_path.display());

    let hook_bin = sibling("truncus-hook")?;
    let settings_path = install_hooks(&hook_bin)?;
    println!("hooks registered in {}", settings_path.display());

    register_mcp(&sibling("truncus-mcp")?);
    Ok(())
}

fn resolve(value: Option<String>, label: &str) -> anyhow::Result<String> {
    if let Some(v) = value {
        return Ok(v.trim().to_string());
    }
    print!("{label}");
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let trimmed = line.trim().to_string();
    anyhow::ensure!(!trimmed.is_empty(), "value is required");
    Ok(trimmed)
}

fn sibling(name: &str) -> anyhow::Result<PathBuf> {
    let dir = std::env::current_exe()?
        .parent()
        .context("resolving binary directory")?
        .to_path_buf();
    let path = dir.join(name);
    anyhow::ensure!(
        path.exists(),
        "{} not found; install all binaries first (cargo install --path truncus-hook etc.)",
        path.display()
    );
    Ok(path)
}

fn install_hooks(hook_bin: &Path) -> anyhow::Result<PathBuf> {
    let path = dirs::home_dir()
        .context("resolving home directory")?
        .join(".claude")
        .join("settings.json");
    let mut settings: Value = std::fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_else(|| json!({}));
    upsert_hook(
        &mut settings,
        "SessionEnd",
        None,
        format!("{} session-end", hook_bin.display()),
    );
    upsert_hook(
        &mut settings,
        "SessionStart",
        Some("startup|resume|clear"),
        format!("{} session-start", hook_bin.display()),
    );
    upsert_hook(
        &mut settings,
        "Stop",
        None,
        format!("{} capture", hook_bin.display()),
    );
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(&settings)?)?;
    Ok(path)
}

fn upsert_hook(settings: &mut Value, event: &str, matcher: Option<&str>, command: String) {
    if !settings["hooks"].is_object() {
        settings["hooks"] = json!({});
    }
    if !settings["hooks"][event].is_array() {
        settings["hooks"][event] = json!([]);
    }
    let entries = settings["hooks"][event].as_array_mut().expect("array ensured");
    entries.retain(|entry| !mentions_truncus(entry));
    let mut entry = json!({ "hooks": [{ "type": "command", "command": command }] });
    if let Some(m) = matcher {
        entry["matcher"] = json!(m);
    }
    entries.push(entry);
}

fn mentions_truncus(entry: &Value) -> bool {
    entry["hooks"]
        .as_array()
        .map(|hooks| {
            hooks.iter().any(|hook| {
                hook["command"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("truncus-hook")
            })
        })
        .unwrap_or(false)
}

fn register_mcp(mcp_bin: &Path) {
    let target = mcp_bin.display().to_string();
    let status = std::process::Command::new("claude")
        .args(["mcp", "add", "truncus", "-s", "user", "--", &target])
        .status();
    match status {
        Ok(code) if code.success() => println!("MCP server registered with Claude Code"),
        _ => println!("register the MCP server manually:\n  claude mcp add truncus -s user -- {target}"),
    }
}
