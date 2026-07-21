use crate::ai::AiService;
use crate::db::Store;
use crate::handlers::query_params;
use crate::vectorize::VectorIndex;
use serde_json::{json, Value};
use truncus_core::dto::{
    NoteList, NoteProjectList, NotesIngest, NotesPrune, NotesRemoved, SearchHit, SearchResponse,
};
use worker::{Context, Date, Request, Response, Result, RouteContext};

const KNOWLEDGE_LIMIT: usize = 8;
const KNOWLEDGE_MAX: usize = 50;

pub async fn projects(_req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let projects = Store::new(ctx.env.d1("DB")?).note_projects().await?;
    Response::from_json(&NoteProjectList { projects })
}

pub async fn list(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let Some(project) = params.get("project").filter(|p| !p.is_empty()) else {
        return Response::error("project is required", 400);
    };
    let notes = Store::new(ctx.env.d1("DB")?).list_notes(project).await?;
    Response::from_json(&NoteList { notes })
}

pub async fn ingest(mut req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let Ok(payload) = req.json::<NotesIngest>().await else {
        return Response::error("invalid body", 400);
    };
    if payload.project.is_empty() {
        return Response::error("project is required", 400);
    }
    let ts = Date::now().as_millis() as i64;
    let response = crate::notes::ingest(&ctx.env, &payload.project, &payload.notes, ts).await?;
    Response::from_json(&response)
}

pub async fn prune(mut req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let Ok(payload) = req.json::<NotesPrune>().await else {
        return Response::error("invalid body", 400);
    };
    let removed = crate::notes::prune(&ctx.env, &payload.project, &payload.paths).await?;
    Response::from_json(&NotesRemoved { removed })
}

pub async fn clear(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let Some(project) = params.get("project").filter(|p| !p.is_empty()) else {
        return Response::error("project is required", 400);
    };
    let removed = crate::notes::clear(&ctx.env, project).await?;
    Response::from_json(&NotesRemoved { removed })
}

pub async fn search(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let Some(query) = params.get("q").filter(|q| !q.trim().is_empty()) else {
        return Response::error("q is required", 400);
    };
    let limit = params
        .get("limit")
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(KNOWLEDGE_LIMIT)
        .min(KNOWLEDGE_MAX);
    let ai = AiService::new(&ctx.env)?;
    let vector = ai
        .embed(std::slice::from_ref(query))
        .await?
        .pop()
        .ok_or_else(|| worker::Error::RustError("query embedding missing".into()))?;
    let mut filter = serde_json::Map::new();
    filter.insert("kind".into(), json!({ "$eq": "note" }));
    if let Some(p) = params.get("project").filter(|p| !p.is_empty()) {
        filter.insert("project".into(), json!({ "$eq": p }));
    }
    let matches = VectorIndex::new(&ctx.env)?
        .query(&vector, limit, Some(Value::Object(filter)))
        .await?;
    let ids: Vec<String> = matches.iter().map(|m| m.id.clone()).collect();
    let hydrated = Store::new(ctx.env.d1("DB")?)
        .hydrate_note_chunks(&ids)
        .await?;
    let mut hits = Vec::with_capacity(matches.len());
    for m in &matches {
        if let Some(hit) = hydrated.iter().find(|h| h.chunk_id == m.id) {
            hits.push(SearchHit {
                session_id: hit.path.clone(),
                kind: "note".into(),
                score: m.score,
                text: format!("[{}]\n{}", hit.title, hit.text),
                project: hit.project.clone(),
                ended_at: hit.updated_at,
            });
        }
    }
    Response::from_json(&SearchResponse { hits })
}
