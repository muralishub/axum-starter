#![allow(unused)]


use axum::{extract::Query, http::header::InvalidHeaderName, response::{Html, IntoResponse}, serve, Router};
use axum::extract::path::Path;
use tokio::net::TcpListener;
use serde::Deserialize;


#[tokio::main]
async fn main() {
    let app = Router::new()
                                 .route("/hello", axum::routing::get(hello_handler))
                               // .route("/hello2/:name", axum::routing::get(hello_handler2) this is old way of doing things 0.6 verison of axum
                               .route("/hello2/{name}", axum::routing::get(hello_handler2)
                                );
  
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
   
    serve(listener, app).await.unwrap();
}
#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>,
}



async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
   println!("->> {:12} - hello_handler- {params:?}", "Handler");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello <Strong>{name}</Strong>"))
    }

    async fn hello_handler2(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:12} - hello_handler- {name:?}", "Handler");
         
         Html(format!("Hello <Strong>{name}</Strong>"))
         }

