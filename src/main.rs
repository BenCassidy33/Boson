use boson::Boson;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Boson::start()?;

    Ok(())
}
