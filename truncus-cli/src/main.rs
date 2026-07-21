mod install;

use clap::{Parser, Subcommand};
use truncus_core::client::ApiClient;
use truncus_core::config::Config;
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
    Install {
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        token: Option<String>,
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
    }
}

fn client() -> anyhow::Result<ApiClient> {
    Ok(ApiClient::new(&Config::load()?))
}

fn pages(total: i64, limit: i64) -> i64 {
    let limit = limit.max(1);
    (total + limit - 1) / limit
}
