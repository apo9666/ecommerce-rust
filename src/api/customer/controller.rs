use actix_web::{
    get,
    web::{self, Path},
    HttpResponse,
};
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Customer {
    id: String,
    email: String,
    name: String,
}

pub fn get_s(
    item: &HashMap<String, AttributeValue>,
    key: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let value = item
        .get(&key)
        .ok_or(format!("{} not set", &key))?
        .as_s()
        .map_err(|_| format!("{} expect a string", &key))?
        .clone();

    Ok(value)
}

pub fn get_optional_s(
    item: &HashMap<String, AttributeValue>,
    key: String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if let Some(attribute) = item.get(&key) {
        let value = attribute
            .as_s()
            .map_err(|_| format!("{} expect a string", &key))?
            .clone();

        return Ok(Some(value));
    }

    Ok(None)
}

#[get("/customers/{id}")]
pub async fn get_customers(
    client: web::Data<Client>,
    path: Path<String>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let id = format!("c#{}", path.into_inner());
    let result = client
        .get_item()
        .key("pk", AttributeValue::S(id.clone()))
        .key("sk", AttributeValue::S(id.clone()))
        .table_name("teste")
        .send()
        .await?;

    if let Some(item) = result.item {
        let response = Customer {
            id: get_s(&item, "pk".into())?,
            email: get_s(&item, "email".into())?,
            name: get_s(&item, "name".into())?,
        };

        let response = serde_json::to_string(&response).unwrap();
        Ok(HttpResponse::Ok().body(response))
    } else {
        Ok(HttpResponse::NotFound().body("not found"))
    }
}
