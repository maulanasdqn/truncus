use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, ContentBlock, ServerCapabilities, ServerInfo};
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler};
use std::sync::Arc;
use truncus_core::client::{ApiClient, ApiError};
use truncus_core::config::Config;
use truncus_core::textview;

const INSTRUCTIONS: &str = "truncus is the user's persistent memory of past Claude Code sessions, \
stored as distilled summaries and conversation chunks on Cloudflare. Use memory_search when the \
user refers to past work, prior decisions, or asks what they did before; use recent_sessions for \
a chronological view; use get_session to expand one session.";

#[derive(Clone)]
pub struct TruncusMcp {
    client: Arc<ApiClient>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SearchParams {
    #[schemars(description = "natural-language query over past session memory")]
    pub query: String,
    #[schemars(description = "optional project name to scope the search")]
    pub project: Option<String>,
    #[schemars(description = "max results, default 8, max 50")]
    pub limit: Option<u32>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct RecentParams {
    #[schemars(description = "optional project name to scope the listing")]
    pub project: Option<String>,
    #[schemars(description = "max sessions, default 10")]
    pub limit: Option<u32>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GetSessionParams {
    #[schemars(description = "id of the session to fetch")]
    pub session_id: String,
}

#[tool_router]
impl TruncusMcp {
    pub fn new(config: Config) -> Self {
        Self {
            client: Arc::new(ApiClient::new(&config)),
        }
    }

    #[tool(
        description = "Semantic search across all past Claude Code sessions stored in truncus memory"
    )]
    async fn memory_search(
        &self,
        Parameters(params): Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .search(
                &params.query,
                params.project.as_deref(),
                params.limit.unwrap_or(8) as usize,
            )
            .await
            .map_err(internal)?;
        Ok(text(textview::hits(&response.hits)))
    }

    #[tool(
        description = "List the most recent Claude Code sessions stored in truncus, newest first"
    )]
    async fn recent_sessions(
        &self,
        Parameters(params): Parameters<RecentParams>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .sessions(params.project.as_deref(), params.limit.unwrap_or(10) as usize, 0)
            .await
            .map_err(internal)?;
        Ok(text(textview::sessions(&response.sessions)))
    }

    #[tool(description = "Fetch one stored session: metadata, status, and distilled summary")]
    async fn get_session(
        &self,
        Parameters(params): Parameters<GetSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        let meta = self
            .client
            .session(&params.session_id)
            .await
            .map_err(internal)?;
        Ok(text(textview::session(&meta)))
    }
}

#[tool_handler]
impl ServerHandler for TruncusMcp {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(ServerCapabilities::builder().enable_tools().build());
        info.server_info.name = "truncus".into();
        info.server_info.version = env!("CARGO_PKG_VERSION").into();
        info.instructions = Some(INSTRUCTIONS.into());
        info
    }
}

fn text(body: String) -> CallToolResult {
    CallToolResult::success(vec![ContentBlock::text(body)])
}

fn internal(error: ApiError) -> McpError {
    McpError::internal_error(error.to_string(), None)
}
