use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api_handlers::status;
use std::sync::Mutex;
mod api_handlers;
use app_state::State;
use dotenv::dotenv;
mod app_state;
use serde_json::{Map, Value};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let client = reqwest::Client::new();

  let _data = client
    .get("https://api.um.warszawa.pl/api/action/dbstore_get")
    .query(&[
      ("id", "1c08a38c-ae09-46d2-8926-4f9d25cb0630"),
      (
        "apikey",
        dotenv::var("UM_WARSZAWA_API_KEY").unwrap().as_str(),
      ),
    ])
    .send()
    .await
    .unwrap()
    .json::<serde_json::Value>()
    .await
    .unwrap();

  let mut stops: Vec<Value> = vec![];

  if let Value::Object(data) = _data {
    if !data.contains_key(&String::from("result")) {
      panic!("No result found");
    }
    let raw_values = data.get(&String::from("result"));
    if let Some(Value::Array(values)) = raw_values {
      for (i, raw_value) in values.iter().enumerate() {
        if let Value::Object(value) = raw_value {
          if let Some(Value::Array(values)) = value.get("values") {
            stops.insert(i, Value::Object(Map::new()));
            for raw_row in values {
              if let Value::Object(row) = raw_row {
                stops.get_mut(i).unwrap().as_object_mut().unwrap().insert(
                  String::from(row.get(&String::from("key")).unwrap().as_str().unwrap()),
                  row.get(&String::from("value")).unwrap().to_owned(),
                );
              }
            }
          }
        } else {
          panic!("OBJECT ERROR");
        }
      }
    } else {
      panic!("No result array found");
    }
  } else {
    panic!("The request failed");
  }

  let state = web::Data::new(State {
    stops: Mutex::new(stops),
  });

  // dbg!(stops);
  // Start server
  HttpServer::new(move || {
    let cors = Cors::permissive(); // TODO Permissive only when --dev flag is set, otherwise default
    App::new()
      .wrap(cors)
      .app_data(state.clone())
      .service(web::scope("/api").service(api_handlers::stops))
      .service(web::scope("/api").service(status))
  })
  .bind(("127.0.0.1", 7015))?
  .run()
  .await
}
