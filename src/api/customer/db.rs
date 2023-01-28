use aws_sdk_dynamodb::{model::AttributeValue, Client};
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppErrorType},
    lib::dynamodb::get_s,
};

#[derive(Serialize, Deserialize)]
pub struct Customer {
    id: String,
    email: String,
    name: String,
}

const CUSTOMER_TABLE: &str = "teste";

pub async fn create_customer(client: &Client, customer: Customer) -> Result<(), AppError> {
    client
        .put_item()
        .table_name(String::from(CUSTOMER_TABLE))
        .item("pk", AttributeValue::S(customer.id.clone()))
        .item("sk", AttributeValue::S(customer.id.clone()))
        .item("email", AttributeValue::S(customer.email.clone()))
        .item("name", AttributeValue::S(customer.name.clone()))
        .send()
        .await
        .map_err(|error| AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::DbError,
        })?;

    Ok(())
}

pub async fn get_customer(client: &Client, id: &String) -> Result<Option<Customer>, AppError> {
    let result = client
        .get_item()
        .table_name(String::from(CUSTOMER_TABLE))
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
        Some(item) => Ok(Some(Customer {
            id: get_s(&item, "pk")?,
            email: get_s(&item, "email")?,
            name: get_s(&item, "name")?,
        })),
    }
}
