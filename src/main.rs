use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::{Value, json};

mod infraestructure;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/health", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000!");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> impl IntoResponse {
    let json_response: Value = json!({
        "success": true,
        "message": "Task manager is up and running!"
    });
    Json(json_response)
}
