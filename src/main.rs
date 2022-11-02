mod api;
mod routes;

use actix_web::{web, App, HttpServer};
use aws_sdk_dynamodb as dynamodb;
use routes::{customer, health_check};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(health_check::route)
            .configure(customer::route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
