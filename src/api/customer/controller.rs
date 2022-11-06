use actix_web::{
    get,
    web::{self, Path},
    HttpResponse, Responder,
};
use aws_sdk_dynamodb::Client;

use crate::{api::customer::db::get_customer, error::AppError};

#[get("/customers/{id}")]
pub async fn get_customers(
    client: web::Data<Client>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    let id = format!("c#{}", path.into_inner());

    let customer = get_customer(&client, &id).await?;

    match customer {
        None => Ok(HttpResponse::NotFound().body("not found")),
        Some(customer) => {
            let response = serde_json::to_string(&customer).unwrap();
            Ok(HttpResponse::Ok().body(response))
        }
    }
}
