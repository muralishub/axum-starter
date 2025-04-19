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





#[tokio::main]
async fn main() {
    let app = Router::new()
      .merge(routes_hello_query_param())
      .merge(routes_hello_path())
      .merge(routes_static())
      ;
                         //        .route("/hello", axum::routing::get(hello_handler))
                               // .route("/hello2/:name", axum::routing::get(hello_handler2) this is old way of doing things 0.6 verison of axum
                          //     .route("/hello2/{name}", axum::routing::get(hello_handler2)
                               
                          //      );
  
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
   
    serve(listener, app).await.unwrap();
}

 fn routes_hello_query_param() -> Router {
    Router::new().route("/hello", axum::routing::get(hello_handler_qp)) 
 }
 fn routes_hello_path() -> Router {
    Router::new().route("/hello2/{name}", axum::routing::get(hello_handler_path)) 
 }

 fn routes_static() -> Router {
    Router::new().fallback_service(
        get_service(ServeDir::new("./"))
    )
}


#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>,
}



async fn hello_handler_qp(Query(params): Query<HelloParams>) -> impl IntoResponse {
   println!("->> {:12} - hello_handler- {params:?}", "Handler");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello <Strong>{name}</Strong>"))
    }

    async fn hello_handler_path(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:12} - hello_handler- {name:?}", "Handler");
         
         Html(format!("Hello <Strong>{name}</Strong>"))
         }




