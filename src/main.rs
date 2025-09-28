#[tokio::main]
async fn main() -> anyhow::Result<()> {
    zero2prod::run().await?;
    Ok(())
}
