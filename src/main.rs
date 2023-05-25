use axum::{response::Html, routing::get, Router};
use reqwest::Client;
use std::net::SocketAddr;
use tower_service::CountBytesService;

mod gateway;
mod tower_service;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let svc = tower_service::CountBytesService::new(client.clone());

    let app = router(client, svc);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn router(client: Client, my_service: CountBytesService) -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/count", get(gateway::count_bytes))
        .route_service("/count2", my_service)
        .with_state(client)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
