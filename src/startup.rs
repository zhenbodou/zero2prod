use crate::routes::{health_check::health_check, subscriptions::subscribe};
use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> anyhow::Result<Server> {
    let database_connection = crate::config::Settings::configuration()?
        .database
        .get_database_connection()
        .await?;
    let database_connection = web::Data::new(database_connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
            .app_data(database_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
