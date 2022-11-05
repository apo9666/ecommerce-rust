use actix_web::{get, web, HttpResponse, Responder};
use aws_sdk_dynamodb::{model::AttributeValue, Client};

#[get("/customers")]
pub async fn get_customers(client: web::Data<Client>) -> impl Responder {
    let result = client
        .get_item()
        .key("pk", AttributeValue::S("teste".into()))
        .key("sk", AttributeValue::S("teste".into()))
        .table_name("teste")
        .send()
        .await
        .expect("get item");

    if let Some(item) = result.item {
        let response = item.get("pk").unwrap().as_s().unwrap();
        HttpResponse::Ok().body(response.clone())
    } else {
        HttpResponse::Ok().body("not ok")
    }
}
