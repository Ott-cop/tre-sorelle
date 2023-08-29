use actix_web::*;
use actix_files as fs;
use std::path::*;

#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))))
            .service(index)
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await;
}



#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/index.html"))
}