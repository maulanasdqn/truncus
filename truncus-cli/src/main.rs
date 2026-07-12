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
        Command::Sessions { project, limit } => {
            let response = client()?.sessions(project.as_deref(), limit).await?;
            println!("{}", textview::sessions(&response.sessions));
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
