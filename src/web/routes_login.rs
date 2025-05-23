use crate::{web, Error, Result};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::routing::post;


pub fn routes() -> Router {
   Router::new().route("/api/login", post(api_login))
}



async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    
    println!("--> {:<12} - api_login", "Handler");

      if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail)
      } 

//TODO: set cookies

//Create the success body

let body = Json(json!({
    "result":{
        "success": true
    }
}));
    


   Ok(body)
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,

}
