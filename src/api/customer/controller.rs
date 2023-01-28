use actix_web::{
    get, post,
    web::{self, Path},
    HttpResponse, Responder,
};
use aws_sdk_dynamodb::Client;

use crate::{api::customer::db, error::AppError};

#[post("/customers")]
pub async fn create_customer(
    client: web::Data<Client>,
    customer: web::Json<db::Customer>,
) -> Result<impl Responder, AppError> {
    db::create_customer(&client, customer.into_inner()).await?;

    Ok(HttpResponse::Created())
}

#[get("/customers/{id}")]
pub async fn get_customer(
    client: web::Data<Client>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    let id = format!("c#{}", path.into_inner());

    let customer = db::get_customer(&client, &id).await?;

    match customer {
        None => Ok(HttpResponse::NotFound().body("not found")),
        Some(customer) => {
            let response = serde_json::to_string(&customer).unwrap();
            Ok(HttpResponse::Ok().body(response))
        }
    }
}
