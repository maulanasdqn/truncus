use worker::{Env, Request, Response};

pub fn require_bearer(req: &Request, env: &Env) -> Result<(), Response> {
    let expected = env
        .secret("TRUNCUS_API_TOKEN")
        .map(|secret| secret.to_string())
        .unwrap_or_default();
    let provided = req
        .headers()
        .get("authorization")
        .ok()
        .flatten()
        .and_then(|header| header.strip_prefix("Bearer ").map(str::to_string))
        .unwrap_or_default();
    if expected.is_empty() || !constant_time_eq(&provided, &expected) {
        return Err(unauthorized());
    }
    Ok(())
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    a.len() == b.len()
        && a.bytes()
            .zip(b.bytes())
            .fold(0u8, |acc, (x, y)| acc | (x ^ y))
            == 0
}

fn unauthorized() -> Response {
    Response::error("unauthorized", 401).expect("static error response")
}
