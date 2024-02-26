use serde::Serialize;
use std::sync::Mutex;
#[derive(Serialize)]
pub struct State {
  pub stops: Mutex<Vec<Stop>>,
}

#[derive(Serialize)]
pub struct Stop {
  id: u32,
  name: String,
  discriminator: u8,
  direction: u32,
  latitude: f64,
  longtude: f64,
  street_id: u32,
}
