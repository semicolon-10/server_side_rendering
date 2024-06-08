use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use crate::model::User;
use crate::template::{HtmlTemplate, IndexTemplate};

mod model;
mod template;


async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}

async fn add_user(Json(user): Json<User>) -> StatusCode {
    println!("{:?}",user);
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    // Server Side Rendering - RUST
    let app = Router::new()
        .route("/", get(index))
        .route("/user", post(add_user));

    let listener = TcpListener::bind(
        "0.0.0.0:3000"
    ).await.unwrap();

    println!("Listening on ...!{:?}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap()
}
