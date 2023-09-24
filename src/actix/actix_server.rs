use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_root() -> impl Responder {
    HttpResponse::Ok().body("Hello there")
}

async fn anonymous_func() -> impl Responder {
    HttpResponse::Ok().body("surprise")
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_root)
            .route("/anonymous", web::get().to(anonymous_func))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
