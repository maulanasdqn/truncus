use crate::db::Store;
use crate::pipeline;
use truncus_core::dto::{IngestRequest, IngestResponse};
use worker::{Context, Request, Response, Result, RouteContext};

pub async fn create(mut req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let Ok(payload) = req.json::<IngestRequest>().await else {
        return Response::error("invalid body", 400);
    };
    if payload.session_id.is_empty() || payload.messages.is_empty() {
        return Response::error("session_id and messages are required", 400);
    }
    let session_id = payload.session_id.clone();
    ctx.env
        .bucket("RAW")?
        .put(
            pipeline::raw_key(&session_id),
            serde_json::to_string(&payload)
                .map_err(|e| worker::Error::RustError(e.to_string()))?,
        )
        .execute()
        .await?;
    Store::new(ctx.env.d1("DB")?).upsert_pending(&payload).await?;
    schedule(&ctx, session_id.clone());
    accepted(session_id)
}

pub async fn reprocess(_req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let Some(session_id) = ctx.param("id").map(|id| id.to_string()) else {
        return Response::error("missing id", 400);
    };
    let store = Store::new(ctx.env.d1("DB")?);
    if store.get_session(&session_id).await?.is_none() {
        return Response::error("not found", 404);
    }
    schedule(&ctx, session_id.clone());
    accepted(session_id)
}

fn schedule(ctx: &RouteContext<Context>, session_id: String) {
    let env = ctx.env.clone();
    ctx.data.wait_until(pipeline::run(env, session_id));
}

fn accepted(id: String) -> Result<Response> {
    Ok(Response::from_json(&IngestResponse {
        id,
        status: "pending".into(),
    })?
    .with_status(202))
}
