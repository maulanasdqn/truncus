use crate::db::Store;
use crate::handlers::query_params;
use serde_json::json;
use truncus_core::dto::LessonList;
use worker::{Context, Request, Response, Result, RouteContext};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;
const REFLECT_DEFAULT: usize = 20;
const REFLECT_MAX: usize = 50;

pub async fn list(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let limit = params
        .get("limit")
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(DEFAULT_LIMIT)
        .min(MAX_LIMIT);
    let lessons = Store::new(ctx.env.d1("DB")?)
        .list_lessons(params.get("project").map(String::as_str), limit)
        .await?;
    Response::from_json(&LessonList { lessons })
}

pub async fn remove(_req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let Some(id) = ctx.param("id") else {
        return Response::error("missing id", 400);
    };
    Store::new(ctx.env.d1("DB")?).delete_lesson(id).await?;
    Response::from_json(&json!({ "id": id, "status": "deleted" }))
}

pub async fn reflect(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let params = query_params(&req)?;
    let session = params.get("session").cloned();
    let project = params.get("project").cloned();
    let limit = params
        .get("limit")
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(REFLECT_DEFAULT)
        .min(REFLECT_MAX);
    let target = session
        .clone()
        .or_else(|| project.clone())
        .unwrap_or_else(|| "recent".into());
    let env = ctx.env.clone();
    ctx.data
        .wait_until(crate::reflect::backfill(env, session, project, limit));
    Ok(Response::from_json(&json!({ "id": target, "status": "reflecting" }))?.with_status(202))
}
