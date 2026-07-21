use crate::ai::AiService;
use crate::db::Store;
use crate::handlers::query_params;
use crate::vectorize::{VectorIndex, VectorMatch};
use serde_json::json;
use std::collections::HashMap;
use truncus_core::dto::{ContextBundle, SearchHit, SearchResponse, SessionList};
use worker::{Context, Request, Response, Result, RouteContext};

const DEFAULT_SEARCH_LIMIT: usize = 8;
const MAX_SEARCH_LIMIT: usize = 50;
const CONTEXT_PROJECT_SESSIONS: usize = 3;
const CONTEXT_OTHER_SESSIONS: usize = 2;
const CONTEXT_LESSONS: usize = 8;

pub async fn search(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let Some(query) = params.get("q").filter(|q| !q.trim().is_empty()) else {
        return Response::error("q is required", 400);
    };
    let limit = params
        .get("limit")
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(DEFAULT_SEARCH_LIMIT)
        .min(MAX_SEARCH_LIMIT);

    let ai = AiService::new(&ctx.env)?;
    let vector = ai
        .embed(std::slice::from_ref(query))
        .await?
        .pop()
        .ok_or_else(|| worker::Error::RustError("query embedding missing".into()))?;
    let matches = VectorIndex::new(&ctx.env)?
        .query(&vector, limit, build_filter(&params))
        .await?;

    let store = Store::new(ctx.env.d1("DB")?);
    let hits = hydrate(&store, &matches).await?;
    Response::from_json(&SearchResponse { hits })
}

pub async fn context(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let Some(project) = params.get("project").filter(|p| !p.is_empty()) else {
        return Response::error("project is required", 400);
    };
    let store = Store::new(ctx.env.d1("DB")?);
    Response::from_json(&ContextBundle {
        project_sessions: store
            .recent_briefs(project, true, CONTEXT_PROJECT_SESSIONS)
            .await?,
        other_sessions: store
            .recent_briefs(project, false, CONTEXT_OTHER_SESSIONS)
            .await?,
        lessons: store.list_lessons(Some(project), CONTEXT_LESSONS).await?,
    })
}

pub async fn list_sessions(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let limit = params
        .get("limit")
        .and_then(|raw| raw.parse::<usize>().ok())
        .unwrap_or(20)
        .min(100);
    let offset = params
        .get("offset")
        .and_then(|raw| raw.parse::<usize>().ok())
        .unwrap_or(0);
    let project = params.get("project").map(String::as_str);
    let store = Store::new(ctx.env.d1("DB")?);
    let sessions = store.list_sessions(project, limit, offset).await?;
    let total = store.count_sessions(project).await?;
    Response::from_json(&SessionList {
        sessions,
        total,
        limit: limit as i64,
        offset: offset as i64,
    })
}

pub async fn get_session(_req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let Some(id) = ctx.param("id") else {
        return Response::error("missing id", 400);
    };
    match Store::new(ctx.env.d1("DB")?).get_session(id).await? {
        Some(session) => Response::from_json(&session),
        None => Response::error("not found", 404),
    }
}

fn build_filter(params: &HashMap<String, String>) -> Option<serde_json::Value> {
    let mut filter = serde_json::Map::new();
    if let Some(project) = params.get("project").filter(|p| !p.is_empty()) {
        filter.insert("project".into(), json!({ "$eq": project }));
    }
    if let Some(kind) = params.get("kind").filter(|k| !k.is_empty()) {
        filter.insert("kind".into(), json!({ "$eq": kind }));
    }
    (!filter.is_empty()).then_some(serde_json::Value::Object(filter))
}

async fn hydrate(store: &Store, matches: &[VectorMatch]) -> Result<Vec<SearchHit>> {
    let summary_ids: Vec<String> = matches
        .iter()
        .filter_map(|m| m.id.strip_suffix("#s").map(str::to_string))
        .collect();
    let chunk_ids: Vec<String> = matches
        .iter()
        .filter(|m| m.id.contains("#c"))
        .map(|m| m.id.clone())
        .collect();

    let summaries = store.hydrate_summaries(&summary_ids).await?;
    let chunks = store.hydrate_chunks(&chunk_ids).await?;

    let mut hits = Vec::with_capacity(matches.len());
    for m in matches {
        if let Some(session_id) = m.id.strip_suffix("#s") {
            if let Some(brief) = summaries.iter().find(|s| s.id == session_id) {
                hits.push(SearchHit {
                    session_id: brief.id.clone(),
                    kind: "summary".into(),
                    score: m.score,
                    text: brief.summary.clone(),
                    project: brief.project.clone(),
                    ended_at: brief.ended_at,
                });
            }
        } else if let Some(chunk) = chunks.iter().find(|c| c.id == m.id) {
            hits.push(SearchHit {
                session_id: chunk.session_id.clone(),
                kind: "chunk".into(),
                score: m.score,
                text: chunk.text.clone(),
                project: chunk.project.clone(),
                ended_at: chunk.ended_at,
            });
        }
    }
    Ok(hits)
}
