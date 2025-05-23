#![allow(unused)]


use axum::{extract::Query,
     http::header::InvalidHeaderName,
     response::{Html, IntoResponse},
     routing::get_service,
     serve,
     Router
    };
use axum::extract::path::Path;
use tokio::net::TcpListener;
use serde::Deserialize;
use tower_http::services::ServeDir;
use axum::http::StatusCode;
use axum::body::Body;
use axum::http::Request;
pub mod error;
pub use error::Error;
pub use error::Result;

mod web;






#[tokio::main]
async fn main() {
    let app = Router::new()
     .merge(routes_hello_path())
     .merge(web::routes_login::routes())
      .merge(routes_static())
      ;



    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
   
    serve(listener, app).await.unwrap();
}


 fn routes_hello_path() -> Router {
    Router::new().route("/hello2/{name}", axum::routing::get(hello_handler_path)) 
 }

 fn routes_static() -> Router {
    Router::new().fallback_service(
        get_service(ServeDir::new("./"))
    )
}




    async fn hello_handler_path(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:12} - hello_handler- {name:?}", "Handler");
         
         Html(format!("Hello <Strong>{name}</Strong>"))
         }




