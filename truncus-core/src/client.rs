use crate::config::Config;
use crate::dto::{
    ContextBundle, IngestRequest, IngestResponse, SearchResponse, SessionList, SessionMeta,
};
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("api {status}: {body}")]
    Api { status: u16, body: String },
}

pub struct ApiClient {
    base: String,
    token: String,
    http: reqwest::Client,
}

impl ApiClient {
    pub fn new(config: &Config) -> Self {
        Self {
            base: config.url.trim_end_matches('/').to_string(),
            token: config.token.clone(),
            http: reqwest::Client::new(),
        }
    }

    pub async fn ingest(&self, request: &IngestRequest) -> Result<IngestResponse, ApiError> {
        self.execute(self.http.post(self.url("/v1/sessions")).json(request))
            .await
    }

    pub async fn reprocess(&self, id: &str) -> Result<IngestResponse, ApiError> {
        self.execute(self.http.post(self.url(&format!("/v1/sessions/{id}/process"))))
            .await
    }

    pub async fn delete_session(&self, id: &str) -> Result<IngestResponse, ApiError> {
        self.execute(self.http.delete(self.url(&format!("/v1/sessions/{id}"))))
            .await
    }

    pub async fn search(
        &self,
        query: &str,
        project: Option<&str>,
        limit: usize,
    ) -> Result<SearchResponse, ApiError> {
        let mut params = vec![("q", query.to_string()), ("limit", limit.to_string())];
        if let Some(p) = project {
            params.push(("project", p.to_string()));
        }
        self.execute(self.http.get(self.url("/v1/search")).query(&params))
            .await
    }

    pub async fn context(&self, project: &str) -> Result<ContextBundle, ApiError> {
        self.execute(
            self.http
                .get(self.url("/v1/context"))
                .query(&[("project", project)]),
        )
        .await
    }

    pub async fn sessions(
        &self,
        project: Option<&str>,
        limit: usize,
        offset: usize,
    ) -> Result<SessionList, ApiError> {
        let mut params = vec![
            ("limit", limit.to_string()),
            ("offset", offset.to_string()),
        ];
        if let Some(p) = project {
            params.push(("project", p.to_string()));
        }
        self.execute(self.http.get(self.url("/v1/sessions")).query(&params))
            .await
    }

    pub async fn session(&self, id: &str) -> Result<SessionMeta, ApiError> {
        self.execute(self.http.get(self.url(&format!("/v1/sessions/{id}"))))
            .await
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base, path)
    }

    async fn execute<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, ApiError> {
        let response = request.bearer_auth(&self.token).send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(ApiError::Api {
                status: status.as_u16(),
                body: response.text().await.unwrap_or_default(),
            });
        }
        Ok(response.json::<T>().await?)
    }
}
