mod ai;
mod auth;
mod db;
mod handlers;
mod pipeline;
mod vectorize;

use worker::{event, Context, Env, Request, Response, Result, Router};

#[event(fetch)]
async fn fetch(req: Request, env: Env, ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    if let Err(denied) = auth::require_bearer(&req, &env) {
        return Ok(denied);
    }
    Router::with_data(ctx)
        .post_async("/v1/sessions", handlers::ingest::create)
        .post_async("/v1/sessions/:id/process", handlers::ingest::reprocess)
        .delete_async("/v1/sessions/:id", handlers::ingest::remove)
        .get_async("/v1/search", handlers::read::search)
        .get_async("/v1/context", handlers::read::context)
        .get_async("/v1/sessions", handlers::read::list_sessions)
        .get_async("/v1/sessions/:id", handlers::read::get_session)
        .run(req, env)
        .await
}
