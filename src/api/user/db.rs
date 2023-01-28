extern crate bcrypt;
use aws_sdk_dynamodb::{
    model::{AttributeValue, Put, TransactWriteItem},
    Client,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

use crate::{
    error::{AppError, AppErrorType},
    lib::dynamodb::get_s,
};

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
                        .item("user_id", AttributeValue::S(id.clone()))
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

pub async fn get_user_by_email(client: &Client, email: &str) -> Result<Option<User>, AppError> {
    let user_email = format!("{}{}", "UE#", email.clone());
    let result = client
        .get_item()
        .table_name(String::from(USER_TABLE))
        .key("pk", AttributeValue::S(user_email.clone()))
        .key("sk", AttributeValue::S(user_email.clone()))
        .send()
        .await
        .map_err(|error| AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::DbError,
        })?;

    let Some(item) = result.item else {
        return Ok(None);
    };

    let id = get_s(&item, "user_id")?;

    let result = client
        .get_item()
        .table_name(String::from(USER_TABLE))
        .key("pk", AttributeValue::S(id.clone()))
        .key("sk", AttributeValue::S(id.clone()))
        .send()
        .await
        .map_err(|error| AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::DbError,
        })?;

    match result.item {
        None => Ok(None),
        Some(item) => {
            let id = get_s(&item, "pk")?;
            let id = &id[2..id.len()];

            Ok(Some(User {
                id: Uuid::parse_str(id).map_err(|error| AppError {
                    cause: Some(error.to_string()),
                    message: None,
                    error_type: AppErrorType::DbError,
                })?,
                email: get_s(&item, "email")?,
                name: get_s(&item, "name")?,
                created_at: get_s(&item, "created_at")?,
                updated_at: get_s(&item, "updated_at")?,
            }))
        }
    }
}
