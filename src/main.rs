use std::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let setting = zero2prod::config::Settings::configuration()?;
    let listener = TcpListener::bind(setting.application.application_address())?;
    let server = zero2prod::startup::run(listener).await?;
    server.await?;
    Ok(())
}
