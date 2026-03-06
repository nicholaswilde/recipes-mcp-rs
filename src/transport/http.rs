use axum::{
    extract::State,
    http::StatusCode,
    response::{sse::{Event, Sse}, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::{self, Stream};
use mcp_sdk_rs::protocol::Request;
use crate::conversion::data::WeightChart;
use crate::handler::handle_request;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct ServerState {
    pub weight_chart: Arc<WeightChart>,
    pub weight_conversion_enabled: bool,
    pub port: u16,
    pub cache: Option<Arc<dyn crate::cache::RecipeCache>>,
}

#[derive(Debug, Deserialize)]
pub struct MessageQuery {
    pub session_id: String,
}

pub async fn run_server(state: ServerState) -> Result<(), Box<dyn std::error::Error>> {
    let port = state.port;
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/sse", get(sse_handler))
        .route("/message", post(handle_message))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("HTTP MCP Server listening on http://0.0.0.0:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn sse_handler(
    State(state): State<Arc<ServerState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let endpoint = format!("http://localhost:{}/message", state.port);
    let initial_event = Event::default()
        .event("endpoint")
        .data(endpoint);

    let stream = stream::once(async move { Ok(initial_event) });

    Sse::new(stream)
}

async fn handle_message(
    State(state): State<Arc<ServerState>>,
    Json(req): Json<Request>,
) -> impl IntoResponse {
    let response = handle_request(
        req,
        state.weight_chart.clone(),
        state.weight_conversion_enabled,
        state.cache.clone(),
    ).await;

    (StatusCode::OK, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mcp_sdk_rs::protocol::{RequestId, Response};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_http_transport_lifecycle() {
        let chart = Arc::new(WeightChart::new());
        let port = 0; // Random port
        
        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
        let actual_port = listener.local_addr().unwrap().port();
        
        let state = Arc::new(ServerState {
            weight_chart: chart,
            weight_conversion_enabled: true,
            port: actual_port,
            cache: None,
        });

        let app = Router::new()
            .route("/sse", get(sse_handler))
            .route("/message", post(handle_message))
            .with_state(state);

        let server_handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        // Give server time to start
        sleep(Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        
        // Test SSE
        let sse_resp = client.get(format!("http://127.0.0.1:{}/sse", actual_port))
            .send()
            .await
            .unwrap();
        assert_eq!(sse_resp.status(), StatusCode::OK);
        assert_eq!(sse_resp.headers()["content-type"], "text/event-stream");

        // Test Message
        let req = Request {
            jsonrpc: "2.0".into(),
            id: RequestId::Number(1),
            method: "tools/list".into(),
            params: None,
        };
        let msg_resp = client.post(format!("http://127.0.0.1:{}/message", actual_port))
            .json(&req)
            .send()
            .await
            .unwrap();
        
        assert_eq!(msg_resp.status(), StatusCode::OK);
        let mcp_resp: Response = msg_resp.json().await.unwrap();
        assert_eq!(mcp_resp.id, RequestId::Number(1));
        
        server_handle.abort();
    }
}
