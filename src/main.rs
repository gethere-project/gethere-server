mod api_handlers;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api_handlers::status;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Start server
  HttpServer::new(move || {
    let cors = Cors::permissive(); // TODO Permissive only when --dev flag is set, otherwise default
    App::new()
      .wrap(cors)
      .service(web::scope("/api").service(status))
  })
  .bind(("127.0.0.1", 7015))?
  .run()
  .await
}
