use actix_web::{App, HttpResponse, HttpServer, web};

pub async fn run() -> anyhow::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    Ok(())
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("I'm healthy")
}
