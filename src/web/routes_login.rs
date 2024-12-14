use crate::{web, Error, Result};
use axum::{routing::post, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> axum::Router {
    axum::Router::new().route("/api/login", post(api_login))
}

async fn api_login(
    cookies: Cookies,
    payload: Json<LoginPayload>) -> Result<Json<Value>> {

    println!("->> {:<12} - api_login - {payload:?}", "HANDLER");
    if payload.username != "chico" && payload.password != "123" {
        return Err(Error::LoginFail);
    }
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-123.456.abc"));

    let body = json!({
        "status": "ok",
        "message": "Login successful",
    });

    Ok(Json(body))
}
