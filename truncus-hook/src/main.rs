mod capture;
mod session_end;
mod session_start;

use std::io::Read;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    let payload: serde_json::Value = serde_json::from_str(&input).unwrap_or_default();
    let outcome = match mode.as_str() {
        "session-end" => session_end::run(&payload).await,
        "session-start" => session_start::run(&payload).await,
        "capture" => capture::run(&payload).await,
        _ => Err(anyhow::anyhow!(
            "usage: truncus-hook <session-end|session-start|capture>"
        )),
    };
    if let Err(error) = outcome {
        eprintln!("truncus-hook: {error}");
    }
}
