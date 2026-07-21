mod ai;
mod auth;
mod db;
mod db_lessons;
mod db_notes;
mod db_read;
mod handlers;
mod notes;
mod pipeline;
mod reflect;
mod vectorize;

use worker::{
    event, Context, Date, Env, Method, Request, Response, Result, Router, ScheduleContext,
    ScheduledEvent,
};

#[event(scheduled)]
async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    if let Ok(database) = env.d1("DB") {
        let store = db::Store::new(database);
        let _ = store.unstick_sessions().await;
        if event.cron() == "0 3 * * *" {
            let now = Date::now().as_millis() as i64;
            let _ = store.decay_lessons(now).await;
        }
    }
}

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
        .get_async("/v1/lessons", handlers::lessons::list)
        .post_async("/v1/lessons/reflect", handlers::lessons::reflect)
        .delete_async("/v1/lessons/:id", handlers::lessons::remove)
        .get_async("/v1/notes/projects", handlers::notes::projects)
        .get_async("/v1/notes/content", handlers::notes::content)
        .get_async("/v1/notes", handlers::notes::list)
        .post_async("/v1/notes", handlers::notes::ingest)
        .post_async("/v1/notes/prune", handlers::notes::prune)
        .delete_async("/v1/notes", handlers::notes::clear)
        .get_async("/v1/knowledge", handlers::notes::search)
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
