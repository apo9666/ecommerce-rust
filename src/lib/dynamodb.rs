use crate::error::AppError;
use crate::error::AppErrorType::*;
use aws_sdk_dynamodb::{model::AttributeValue, output::GetItemOutput, Client};
use std::collections::HashMap;

pub async fn get_item(
    client: &Client,
    pk: String,
    sk: String,
    table: String,
) -> Result<GetItemOutput, AppError> {
    client
        .get_item()
        .key("pk", AttributeValue::S(pk))
        .key("sk", AttributeValue::S(sk))
        .table_name(table)
        .send()
        .await
        .map_err(|error| AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: DbError,
        })
}

pub fn as_s(attribute_value: &AttributeValue, key: &String) -> Result<String, AppError> {
    let value = attribute_value
        .as_s()
        .map_err(|_| AppError {
            cause: Some(format!("{} expect a string", &key)),
            message: None,
            error_type: DbValidationError,
        })?
        .clone();
    Ok(value)
}

pub fn get_s(item: &HashMap<String, AttributeValue>, key: String) -> Result<String, AppError> {
    let value = item.get(&key).ok_or_else(|| AppError {
        cause: Some(format!("{} not set", &key)),
        message: None,
        error_type: DbValidationError,
    })?;

    Ok(as_s(value, &key)?)
}

// pub fn get_optional_s(
//     item: &HashMap<String, AttributeValue>,
//     key: &String,
// ) -> Result<Option<String>, AppError> {
//     match item.get(key) {
//         None => Ok(None),
//         Some(attribute_value) => Ok(Some(as_s(attribute_value, &key)?)),
//     }
// }
