use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use aws_sdk_dynamodb::Client;

use crate::{
    api::user::db::{self, NewUser},
    error::AppError,
};

#[post("/users")]
pub async fn create_user(
    client: web::Data<Client>,
    new_user: web::Json<NewUser>,
) -> Result<impl Responder, AppError> {
    let user = db::create_user(&client, &new_user).await?;

    Ok(HttpResponse::Created().json(user))
}
