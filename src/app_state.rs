use serde::Serialize;
use std::sync::Mutex;
#[derive(Serialize)]
pub struct State {
  pub stops: Mutex<Vec<Stop>>,
}

#[derive(Serialize)]
pub struct Stop {
  pub id: u32,
  pub name: String,
  pub discriminator: u8,
  pub direction: String,
  pub latitude: f64,
  pub longtitude: f64,
  pub street_id: u32,
}
