use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api_handlers::status;
use std::sync::Mutex;
mod api_handlers;
use app_state::State;
use dotenv::dotenv;
mod app_state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let state = web::Data::new(State {
    stops: Mutex::new(vec![]),
  });

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

  // Start server
  HttpServer::new(move || {
    let cors = Cors::permissive(); // TODO Permissive only when --dev flag is set, otherwise default
    App::new()
      .wrap(cors)
      .app_data(state.clone())
      .service(web::scope("/api").service(status))
  })
  .bind(("127.0.0.1", 7015))?
  .run()
  .await
}
