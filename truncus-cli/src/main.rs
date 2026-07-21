mod install;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use truncus_core::client::ApiClient;
use truncus_core::config::Config;
use truncus_core::dto::NoteInput;
use truncus_core::textview;

#[derive(Parser)]
#[command(name = "truncus", about = "Unified AI memory cluster on Cloudflare", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Search {
        query: String,
        #[arg(long)]
        project: Option<String>,
        #[arg(long, default_value_t = 8)]
        limit: usize,
    },
    Sessions {
        #[arg(long)]
        project: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: usize,
        #[arg(long, default_value_t = 1)]
        page: usize,
    },
    Session {
        id: String,
    },
    Reprocess {
        id: String,
    },
    Delete {
        id: String,
    },
    Lessons {
        #[arg(long)]
        project: Option<String>,
        #[arg(long, default_value_t = 30)]
        limit: usize,
    },
    Reflect {
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        session: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    Vault {
        #[command(subcommand)]
        action: VaultAction,
    },
    Knowledge {
        query: String,
        #[arg(long)]
        project: Option<String>,
        #[arg(long, default_value_t = 8)]
        limit: usize,
    },
    Install {
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        token: Option<String>,
    },
}

#[derive(Subcommand)]
enum VaultAction {
    Sync {
        #[arg(long)]
        project: String,
        folder: String,
    },
    List {
        #[arg(long)]
        project: String,
    },
    Clear {
        #[arg(long)]
        project: String,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Install { url, token } => install::run(url, token),
        Command::Search {
            query,
            project,
            limit,
        } => {
            let response = client()?.search(&query, project.as_deref(), limit).await?;
            println!("{}", textview::hits(&response.hits));
            Ok(())
        }
        Command::Sessions {
            project,
            limit,
            page,
        } => {
            let limit = limit.max(1);
            let page = page.max(1);
            let offset = (page - 1) * limit;
            let response = client()?
                .sessions(project.as_deref(), limit, offset)
                .await?;
            let total = response.total;
            let shown = response.sessions.len();
            if shown == 0 {
                match total {
                    0 => println!("No sessions stored yet."),
                    _ => println!(
                        "No sessions on page {page} (total {total}, {} pages).",
                        pages(total, response.limit)
                    ),
                }
            } else {
                println!("{}", textview::sessions(&response.sessions));
                let from = response.offset + 1;
                let to = response.offset + shown as i64;
                println!(
                    "\n— {from}–{to} of {total} · page {page}/{} —",
                    pages(total, response.limit)
                );
            }
            Ok(())
        }
        Command::Session { id } => {
            let meta = client()?.session(&id).await?;
            println!("{}", textview::session(&meta));
            Ok(())
        }
        Command::Reprocess { id } => {
            let response = client()?.reprocess(&id).await?;
            println!("session {} queued ({})", response.id, response.status);
            Ok(())
        }
        Command::Delete { id } => {
            let response = client()?.delete_session(&id).await?;
            println!("session {} {}", response.id, response.status);
            Ok(())
        }
        Command::Lessons { project, limit } => {
            let response = client()?.lessons(project.as_deref(), limit).await?;
            println!("{}", textview::lessons(&response.lessons));
            Ok(())
        }
        Command::Reflect {
            project,
            session,
            limit,
        } => {
            let ack = client()?
                .reflect(project.as_deref(), session.as_deref(), limit)
                .await?;
            println!("reflection {} ({})", ack.id, ack.status);
            Ok(())
        }
        Command::Vault { action } => run_vault(action).await,
        Command::Knowledge {
            query,
            project,
            limit,
        } => {
            let response = client()?
                .knowledge(&query, project.as_deref(), limit)
                .await?;
            println!("{}", textview::knowledge(&response.hits));
            Ok(())
        }
    }
}

async fn run_vault(action: VaultAction) -> anyhow::Result<()> {
    match action {
        VaultAction::Sync { project, folder } => vault_sync(&project, &folder).await,
        VaultAction::List { project } => {
            let list = client()?.list_notes(&project).await?;
            if list.notes.is_empty() {
                println!("no notes synced for {project}");
                return Ok(());
            }
            for note in &list.notes {
                println!("{} · {} · {} chunks", note.path, note.title, note.chunk_count);
            }
            println!("\n{} notes", list.notes.len());
            Ok(())
        }
        VaultAction::Clear { project } => {
            let removed = client()?.clear_notes(&project).await?;
            println!("cleared {} notes from {project}", removed.removed);
            Ok(())
        }
    }
}

async fn vault_sync(project: &str, folder: &str) -> anyhow::Result<()> {
    let root = Path::new(folder);
    if !root.is_dir() {
        anyhow::bail!("{folder} is not a directory");
    }
    let mut files = Vec::new();
    collect_markdown(root, &mut files)?;
    files.sort();
    if files.is_empty() {
        println!("no markdown files under {folder}");
        return Ok(());
    }
    let client = client()?;
    let mut paths = Vec::new();
    let mut batch = Vec::new();
    let (mut ingested, mut skipped, mut chunks) = (0i64, 0i64, 0i64);
    for file in &files {
        let rel = file
            .strip_prefix(root)
            .unwrap_or(file)
            .to_string_lossy()
            .replace('\\', "/");
        let content = std::fs::read_to_string(file).unwrap_or_default();
        if content.trim().is_empty() {
            continue;
        }
        let title = note_title(&rel, &content);
        paths.push(rel.clone());
        batch.push(NoteInput {
            path: rel,
            title,
            content,
        });
        if batch.len() >= 10 {
            let result = client
                .ingest_notes(project, std::mem::take(&mut batch))
                .await?;
            ingested += result.ingested;
            skipped += result.skipped;
            chunks += result.chunks;
        }
    }
    if !batch.is_empty() {
        let result = client.ingest_notes(project, batch).await?;
        ingested += result.ingested;
        skipped += result.skipped;
        chunks += result.chunks;
    }
    let pruned = client.prune_notes(project, paths).await?;
    println!(
        "synced {project}: {ingested} embedded, {skipped} unchanged, {chunks} chunks, {} removed",
        pruned.removed
    );
    Ok(())
}

fn collect_markdown(dir: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            collect_markdown(&path, out)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

fn note_title(rel_path: &str, content: &str) -> String {
    for line in content.lines() {
        if let Some(heading) = line.trim().strip_prefix("# ") {
            let heading = heading.trim();
            if !heading.is_empty() {
                return heading.to_string();
            }
        }
    }
    rel_path
        .rsplit('/')
        .next()
        .unwrap_or(rel_path)
        .trim_end_matches(".md")
        .to_string()
}

fn client() -> anyhow::Result<ApiClient> {
    Ok(ApiClient::new(&Config::load()?))
}

fn pages(total: i64, limit: i64) -> i64 {
    let limit = limit.max(1);
    (total + limit - 1) / limit
}
