use crate::models::{Queue, Task};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use thiserror::Error;

/// Errors that can occur when interacting with the GQueues API.
#[derive(Error, Debug)]
pub enum GqueuesError {
    /// The API rate limit was exceeded.
    #[error("API Rate Limit Exceeded. Retry after {0:?}")]
    RateLimited(Duration),
    /// Authentication failed (401 or 403).
    #[error("Authentication Error: {0}")]
    AuthError(String),
    /// A general API error occurred.
    #[error("API Error: {0}")]
    ApiError(String),
    /// A network error occurred during the request.
    #[error("Network Error: {0}")]
    NetworkError(#[from] reqwest::Error),
    /// Failed to serialize or deserialize JSON data.
    #[error("Serialization Error: {0}")]
    SerializationError(String),
    /// An unexpected internal error occurred.
    #[error("Internal Error: {0}")]
    InternalError(String),
}

/// A specialized Result type for GQueues API operations.
pub type Result<T> = std::result::Result<T, GqueuesError>;

/// The main client for interacting with the GQueues API.
///
/// Use [`GqueuesClient::builder`] or [`GqueuesClient::new`] to instantiate.
#[derive(Clone)]
pub struct GqueuesClient {
    client: Client,
    base_url: String,
    access_token: String,
}

/// A builder for creating a [`GqueuesClient`] with custom configuration.
pub struct GqueuesClientBuilder {
    access_token: String,
    base_url: Option<String>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    proxy: Option<reqwest::Proxy>,
}

impl GqueuesClientBuilder {
    /// Creates a new builder with the required access token.
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            base_url: None,
            timeout: None,
            user_agent: None,
            proxy: None,
        }
    }

    /// Sets a custom base URL for the API.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets a timeout for API requests.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets a custom User-Agent header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sets a proxy for API requests.
    pub fn proxy(mut self, proxy: reqwest::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Builds the [`GqueuesClient`] with the configured settings.
    pub fn build(self) -> Result<GqueuesClient> {
        let mut builder = Client::builder();

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(user_agent) = self.user_agent {
            builder = builder.user_agent(user_agent);
        }

        if let Some(proxy) = self.proxy {
            builder = builder.proxy(proxy);
        }

        let client = builder.build()?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.gqueues.com".to_string());

        Ok(GqueuesClient {
            client,
            base_url,
            access_token: self.access_token,
        })
    }
}

#[derive(Deserialize)]
struct QueuesResponse {
    personal: Option<Vec<Queue>>,
    team: Option<Vec<Queue>>,
    shared: Option<Vec<Queue>>,
}

#[derive(Deserialize)]
struct TasksResponse {
    items: Vec<Task>,
    #[serde(rename = "nextCursor")]
    _next_cursor: Option<String>,
}

impl GqueuesClient {
    /// Creates a new [`GqueuesClientBuilder`] with the given access token.
    pub fn builder(access_token: impl Into<String>) -> GqueuesClientBuilder {
        GqueuesClientBuilder::new(access_token)
    }

    /// Creates a new [`GqueuesClient`] with default settings.
    pub fn new(base_url: String, access_token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            access_token,
        }
    }

    /// Fetches all queues (personal, team, and shared) from GQueues.
    pub async fn get_queues(&self) -> Result<Vec<Queue>> {
        let url = format!("{}/v0?action=getQueues", self.base_url);
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = resp
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .map(Duration::from_secs)
                .unwrap_or(Duration::from_secs(300));
            return Err(GqueuesError::RateLimited(retry_after));
        }

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED
            || resp.status() == reqwest::StatusCode::FORBIDDEN
        {
            return Err(GqueuesError::AuthError(format!(
                "Authentication failed: {}",
                resp.status()
            )));
        }

        if !resp.status().is_success() {
            return Err(GqueuesError::ApiError(format!(
                "Failed to fetch queues: {}",
                resp.status()
            )));
        }

        let body = resp.text().await.map_err(|e| {
            GqueuesError::InternalError(format!("Failed to read queues response body: {}", e))
        })?;

        let data: QueuesResponse = serde_json::from_str(&body).map_err(|e| {
            GqueuesError::SerializationError(format!(
                "Failed to decode queues response: {}. Body: {}",
                e, body
            ))
        })?;
        let mut all_queues = Vec::new();
        if let Some(mut q) = data.personal {
            for item in &mut q {
                item.scope = Some("Personal".into());
            }
            all_queues.append(&mut q);
        }
        if let Some(mut q) = data.team {
            for item in &mut q {
                item.scope = Some("Team".into());
            }
            all_queues.append(&mut q);
        }
        if let Some(mut q) = data.shared {
            for item in &mut q {
                item.scope = Some("Shared".into());
            }
            all_queues.append(&mut q);
        }

        Ok(all_queues)
    }

    /// Fetches active tasks for a specific queue.
    pub async fn get_tasks(&self, queue_key: &str) -> Result<Vec<Task>> {
        let url = format!(
            "{}/v0?action=getActiveTasks&queueKey={}",
            self.base_url, queue_key
        );
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = resp
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .map(Duration::from_secs)
                .unwrap_or(Duration::from_secs(300));
            return Err(GqueuesError::RateLimited(retry_after));
        }

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED
            || resp.status() == reqwest::StatusCode::FORBIDDEN
        {
            return Err(GqueuesError::AuthError(format!(
                "Authentication failed: {}",
                resp.status()
            )));
        }

        if !resp.status().is_success() {
            return Err(GqueuesError::ApiError(format!(
                "Failed to fetch tasks for queue {}: {}",
                queue_key,
                resp.status()
            )));
        }

        let body = resp.text().await.map_err(|e| {
            GqueuesError::InternalError(format!(
                "Failed to read tasks response body for queue {}: {}",
                queue_key, e
            ))
        })?;

        let data: TasksResponse = serde_json::from_str(&body).map_err(|e| {
            GqueuesError::SerializationError(format!(
                "Failed to decode tasks response for queue {}: {}. Body: {}",
                queue_key, e, body
            ))
        })?;
        Ok(data.items)
    }

    /// Creates a new task with a specific idempotency key.
    ///
    /// This is useful for ensuring that retries do not result in duplicate tasks.
    pub async fn create_task_with_idempotency(
        &self,
        text: &str,
        queue_key: Option<&str>,
        parent_key: Option<&str>,
        notes: Option<&str>,
        tags: Option<Vec<String>>,
        due_date: Option<&str>,
        parse_quick_add_syntax: bool,
        idempotency_key: &str,
    ) -> Result<Task> {
        let url = format!("{}/v0", self.base_url);
        let mut instruction = serde_json::json!({
            "text": text,
            "parseQuickAddSyntax": parse_quick_add_syntax,
        });

        if let Some(qk) = queue_key {
            instruction["queueKey"] = serde_json::json!(qk);
        }
        if let Some(pk) = parent_key {
            instruction["parentKey"] = serde_json::json!(pk);
        }
        if let Some(n) = notes {
            instruction["notes"] = serde_json::json!(n);
        }
        if let Some(t) = tags {
            instruction["tags"] = serde_json::json!(t);
        }
        if let Some(d) = due_date {
            instruction["dueDate"] = serde_json::json!({ "rawDate": d });
        }

        let body = serde_json::json!({
            "action": "createTask",
            "instructions": [instruction]
        });

        let resp = self
            .client
            .post(url)
            .bearer_auth(&self.access_token)
            .header("Idempotency-Key", idempotency_key)
            .json(&body)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = resp
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .map(Duration::from_secs)
                .unwrap_or(Duration::from_secs(300));
            return Err(GqueuesError::RateLimited(retry_after));
        }

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED
            || resp.status() == reqwest::StatusCode::FORBIDDEN
        {
            return Err(GqueuesError::AuthError(format!(
                "Authentication failed: {}",
                resp.status()
            )));
        }

        if !resp.status().is_success() {
            return Err(GqueuesError::ApiError(format!(
                "Failed to create task: {}",
                resp.status()
            )));
        }

        let body = resp.text().await.map_err(|e| {
            GqueuesError::InternalError(format!("Failed to read create task response body: {}", e))
        })?;

        #[derive(Deserialize)]
        struct CreateResponse {
            results: Vec<serde_json::Value>,
        }

        let data: CreateResponse = serde_json::from_str(&body).map_err(|e| {
            GqueuesError::SerializationError(format!(
                "Failed to decode create task response: {}. Body: {}",
                e, body
            ))
        })?;
        let task_json = data.results.first().ok_or_else(|| {
            GqueuesError::InternalError(format!(
                "No task returned in creation response. Body: {}",
                body
            ))
        })?;

        // The API returns { status: "created", task: { ... } }
        let task: Task = serde_json::from_value(task_json["task"].clone()).map_err(|e| {
            GqueuesError::SerializationError(format!(
                "Failed to parse created task: {}. Item: {}",
                e, task_json
            ))
        })?;

        Ok(task)
    }

    /// Creates a new task with an automatically generated idempotency key.
    pub async fn create_task(
        &self,
        text: &str,
        queue_key: Option<&str>,
        notes: Option<&str>,
    ) -> Result<Task> {
        let idempotency_key = uuid::Uuid::new_v4().to_string();
        self.create_task_with_idempotency(
            text,
            queue_key,
            None,
            notes,
            None,
            None,
            queue_key.is_none(),
            &idempotency_key,
        )
        .await
    }
}