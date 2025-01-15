mod trial;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(trial::hello)
            .service(trial::echo)
            .route("/hey", web::get().to(trial::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
