use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use worker::{Env, Fetch, Headers, Method, Request, RequestInit, Result};

pub struct VectorIndex {
    account_id: String,
    index: String,
    token: String,
}

#[derive(Serialize)]
pub struct VectorRecord {
    pub id: String,
    pub values: Vec<f32>,
    pub metadata: Value,
}

#[derive(Debug, Deserialize)]
pub struct VectorMatch {
    pub id: String,
    pub score: f64,
}

#[derive(Deserialize)]
struct QueryResult {
    matches: Vec<VectorMatch>,
}

impl VectorIndex {
    pub fn new(env: &Env) -> Result<Self> {
        Ok(Self {
            account_id: env.var("ACCOUNT_ID")?.to_string(),
            index: env.var("VECTORIZE_INDEX")?.to_string(),
            token: env.secret("CF_API_TOKEN")?.to_string(),
        })
    }

    pub async fn upsert(&self, records: &[VectorRecord]) -> Result<()> {
        let body = records
            .iter()
            .map(|record| serde_json::to_string(record).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n");
        self.send("upsert", "application/x-ndjson", body).await?;
        Ok(())
    }

    pub async fn query(
        &self,
        vector: &[f32],
        top_k: usize,
        filter: Option<Value>,
    ) -> Result<Vec<VectorMatch>> {
        let mut payload = json!({
            "vector": vector,
            "topK": top_k,
            "returnValues": false,
            "returnMetadata": "none"
        });
        if let Some(f) = filter {
            payload["filter"] = f;
        }
        let result = self
            .send("query", "application/json", payload.to_string())
            .await?;
        let parsed: QueryResult = serde_json::from_value(result)
            .map_err(|e| worker::Error::RustError(format!("vectorize query parse: {e}")))?;
        Ok(parsed.matches)
    }

    pub async fn delete_by_ids(&self, ids: &[String]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        self.send(
            "delete_by_ids",
            "application/json",
            json!({ "ids": ids }).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn send(&self, op: &str, content_type: &str, body: String) -> Result<Value> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/vectorize/v2/indexes/{}/{op}",
            self.account_id, self.index
        );
        let headers = Headers::new();
        headers.set("authorization", &format!("Bearer {}", self.token))?;
        headers.set("content-type", content_type)?;
        let init = RequestInit {
            method: Method::Post,
            headers,
            body: Some(body.into()),
            ..RequestInit::default()
        };
        let request = Request::new_with_init(&url, &init)?;
        let mut response = Fetch::Request(request).send().await?;
        let mut payload: Value = response.json().await?;
        if response.status_code() >= 300 || payload["success"] != Value::Bool(true) {
            return Err(worker::Error::RustError(format!(
                "vectorize {op} failed ({}): {}",
                response.status_code(),
                payload["errors"]
            )));
        }
        Ok(payload["result"].take())
    }
}
