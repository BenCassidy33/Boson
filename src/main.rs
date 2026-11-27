use tachyon::Tachyon;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Tachyon::start()?;

    Ok(())
}
