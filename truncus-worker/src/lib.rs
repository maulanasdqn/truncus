mod ai;
mod auth;
mod db;
mod db_read;
mod handlers;
mod pipeline;
mod vectorize;

use worker::{event, Context, Env, Method, Request, Response, Result, Router};

#[event(fetch)]
async fn fetch(req: Request, env: Env, ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    if req.method() == Method::Options {
        return with_cors(Response::empty()?.with_status(204));
    }
    if let Err(denied) = auth::require_bearer(&req, &env) {
        return with_cors(denied);
    }
    let response = Router::with_data(ctx)
        .post_async("/v1/sessions", handlers::ingest::create)
        .post_async("/v1/sessions/:id/process", handlers::ingest::reprocess)
        .delete_async("/v1/sessions/:id", handlers::ingest::remove)
        .get_async("/v1/search", handlers::read::search)
        .get_async("/v1/context", handlers::read::context)
        .get_async("/v1/sessions", handlers::read::list_sessions)
        .get_async("/v1/sessions/:id", handlers::read::get_session)
        .run(req, env)
        .await?;
    with_cors(response)
}

fn with_cors(response: Response) -> Result<Response> {
    let headers = response.headers();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Authorization, Content-Type")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(response)
}
