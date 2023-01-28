extern crate bcrypt;
use aws_sdk_dynamodb::{
    model::{AttributeValue, Put, TransactWriteItem},
    Client,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, AppErrorType};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    email: String,
    name: String,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    email: String,
    name: String,
    password: String,
}

const USER_TABLE: &str = "teste";

pub async fn create_user(client: &Client, new_user: &NewUser) -> Result<User, AppError> {
    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let user = User {
        id: Uuid::new_v4(),
        name: new_user.name.clone(),
        email: new_user.email.clone(),
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    let password = hash(new_user.password.clone(), DEFAULT_COST).map_err(|error| AppError {
        cause: Some(error.to_string()),
        message: None,
        error_type: AppErrorType::DbError,
    })?;

    let id = format!("{}{}", "U#", user.id.clone());
    let user_email = format!("{}{}", "UE#", user.email.clone());
    client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    Put::builder()
                        .table_name(USER_TABLE)
                        .item("pk", AttributeValue::S(id.clone()))
                        .item("sk", AttributeValue::S(id.clone()))
                        .item("email", AttributeValue::S(user.email.clone()))
                        .item("name", AttributeValue::S(user.name.clone()))
                        .item("password", AttributeValue::S(password))
                        .item("created_at", AttributeValue::S(user.created_at.clone()))
                        .item("updated_at", AttributeValue::S(user.updated_at.clone()))
                        .condition_expression("attribute_not_exists(pk)")
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    Put::builder()
                        .table_name(USER_TABLE)
                        .item("pk", AttributeValue::S(user_email.clone()))
                        .item("sk", AttributeValue::S(user_email.clone()))
                        .condition_expression("attribute_not_exists(pk)")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .map_err(|error| AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::DbError,
        })?;

    Ok(user)
}
