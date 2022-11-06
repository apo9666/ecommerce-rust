use aws_sdk_dynamodb::Client;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    lib::dynamodb::{get_item, get_s},
};

#[derive(Serialize, Deserialize)]
pub struct Customer {
    id: String,
    email: String,
    name: String,
}

pub async fn get_customer(client: &Client, id: &String) -> Result<Option<Customer>, AppError> {
    let result = get_item(client, id.clone(), id.clone(), "teste".into()).await?;

    match result.item {
        None => Ok(None),
        Some(item) => Ok(Some(Customer {
            id: get_s(&item, "pk".into())?,
            email: get_s(&item, "email".into())?,
            name: get_s(&item, "name".into())?,
        })),
    }
}
