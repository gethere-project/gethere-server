use actix_web::{
  get,
  web::{Data, Json},
  Responder, Result,
};
use serde_json::json;

use crate::app_state::State;
#[get("/status")]
pub async fn status(_: Data<State>) -> Result<impl Responder> {
  Ok(Json(json!({
    "status": "ok",
  })))
}

#[get("/stops")]
pub async fn stops(data: Data<State>) -> Result<impl Responder> {
  Ok(Json(json!(*data.stops.lock().unwrap())))
}
