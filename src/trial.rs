use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

#[get("/")]
pub async fn hello(query: web::Query<QueryParams>) -> impl Responder {
    let process = query.into_inner();
    let love = '\u{2764}';
    let rav: String = format!("{} {}'s rust!", &process.name, &love);
    HttpResponse::Ok().body(rav)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
