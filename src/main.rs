#![allow(unused)]


use axum::{response::{Html, IntoResponse}, serve, Router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let app = Router::new()
                                 .route("/", axum::routing::get(|| hello_handler()));
  
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
   
    serve(listener, app).await.unwrap();
}

async fn hello_handler() -> impl IntoResponse {

    println!("->> {:12} - hello_handler", "Handler");
    Html("Hello <Strong>world!</Strong>")



    }
