#![allow(unused)]


use axum::{response::Html, serve, Router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let app = Router::new()
                                 .route("/", axum::routing::get(|| async {Html("hello <Strong>world</Strong>")}));
  
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
   
    serve(listener, app).await.unwrap();
}
