mod api;
mod error;
mod lib;
mod log;
mod routes;

use actix_web::{web, App, HttpServer};
use aws_sdk_dynamodb as dynamodb;
use log::{configure_log, Logging};
use routes::{customer, health_check};
use slog::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);
    let log = configure_log();
    let host = "127.0.0.1";
    let port = 8080;

    info!(log, "Starting server at http://{}:{}", &host, &port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logging::new(log.clone()))
            .app_data(web::Data::new(client.clone()))
            .configure(health_check::route)
            .configure(customer::route)
    })
    .bind((host, port))?
    .run()
    .await
}
