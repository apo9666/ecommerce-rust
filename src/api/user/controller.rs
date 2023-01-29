use actix_web::{
    get, post,
    web::{self, Path},
    HttpRequest, HttpResponse, Responder,
};
use aws_sdk_dynamodb::Client;
use slog::{info, o, Logger};

use crate::{
    api::user::db::{self, NewUser},
    error::AppError,
    log::{create_log, log_error},
};

#[post("/users")]
pub async fn create_user(
    log: web::Data<Logger>,
    client: web::Data<Client>,
    new_user: web::Json<NewUser>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let sublog = create_log(&log, &req);
    info!(sublog, "aquiii"; "ss" => "booom");
    let user = db::create_user(&client, &new_user).await?;

    Ok(HttpResponse::Created().json(user))
}

#[get("/users/{id}")]
pub async fn get_user(
    client: web::Data<Client>,
    log: web::Data<Logger>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    let sublog = log.new(o!("handler" => "create_todo"));

    info!(sublog, "aquiii");
    let customer = db::get_user_by_email(&client, &path.into_inner())
        .await
        .map_err(log_error(sublog))?;

    match customer {
        None => Ok(HttpResponse::NotFound().body("not found")),
        Some(customer) => Ok(HttpResponse::Ok().json(customer)),
    }
}
