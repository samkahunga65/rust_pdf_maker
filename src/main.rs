mod controller {
    pub mod trial;
}
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("I am running at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(controller::trial::hello)
            .service(controller::trial::echo)
            .route("/hey", web::get().to(controller::trial::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
