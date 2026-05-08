use gqueues_api_rs::{GqueuesClient, GqueuesError};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_queues_success() {
    let mock_server = MockServer::start().await;
    let access_token = "test-token";

    let response_body = json!({
        "personal": [
            { "key": "q1", "name": "Inbox", "isInbox": true }
        ],
        "team": [
            { "key": "q2", "name": "Team Queue", "teamName": "Project X" }
        ],
        "shared": []
    });

    Mock::given(method("GET"))
        .and(path("/v0"))
        .and(query_param("action", "getQueues"))
        .and(header("Authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = GqueuesClient::builder(access_token)
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let queues = client.get_queues().await.unwrap();

    assert_eq!(queues.len(), 2);
    assert_eq!(queues[0].name, "Inbox");
    assert_eq!(queues[0].scope, Some("Personal".to_string()));
    assert_eq!(queues[1].name, "Team Queue");
    assert_eq!(queues[1].scope, Some("Team".to_string()));
}

#[tokio::test]
async fn test_get_queues_rate_limited() {
    let mock_server = MockServer::start().await;
    let access_token = "test-token";

    Mock::given(method("GET"))
        .and(path("/v0"))
        .and(query_param("action", "getQueues"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "60"))
        .mount(&mock_server)
        .await;

    let client = GqueuesClient::builder(access_token)
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.get_queues().await;

    match result {
        Err(GqueuesError::RateLimited(duration)) => assert_eq!(duration.as_secs(), 60),
        _ => panic!("Expected RateLimited error, got {:?}", result),
    }
}

#[tokio::test]
async fn test_get_tasks_success() {
    let mock_server = MockServer::start().await;
    let access_token = "test-token";
    let queue_key = "q1";

    let response_body = json!({
        "items": [
            { "key": "t1", "title": "Task 1", "completed": false },
            { "key": "t2", "title": "Task 2", "completed": true }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/v0"))
        .and(query_param("action", "getActiveTasks"))
        .and(query_param("queueKey", queue_key))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = GqueuesClient::builder(access_token)
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let tasks = client.get_tasks(queue_key).await.unwrap();

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Task 1");
    assert!(tasks[1].completed);
}

#[tokio::test]
async fn test_create_task_success() {
    let mock_server = MockServer::start().await;
    let access_token = "test-token";
    let idempotency_key = "unique-id";

    let response_body = json!({
        "results": [
            {
                "status": "created",
                "task": { "key": "t_new", "title": "New Task", "completed": false }
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/v0"))
        .and(header("Idempotency-Key", idempotency_key))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = GqueuesClient::builder(access_token)
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let task = client
        .create_task_with_idempotency("New Task", None, None, idempotency_key)
        .await
        .unwrap();

    assert_eq!(task.title, "New Task");
    assert_eq!(task.key, "t_new");
}

#[tokio::test]
async fn test_auth_error() {
    let mock_server = MockServer::start().await;
    let access_token = "invalid-token";

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;

    let client = GqueuesClient::builder(access_token)
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.get_queues().await;

    match result {
        Err(GqueuesError::AuthError(_)) => (),
        _ => panic!("Expected AuthError, got {:?}", result),
    }
}
