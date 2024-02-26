use actix_web::{get, web::Json, Responder, Result};
use serde_json::json;

#[get("/status")]
async fn status() -> Result<impl Responder> {
  Ok(Json(json!({
    "status": "ok",
  })))
}
