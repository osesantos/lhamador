use kube::Client;
use tracing::*;

mod controller;
mod crd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let _client = Client::try_default()
        .await
        .expect("Failed to create K8s client");

    info!("🚀 Lhamador is alive and connected to Kubernetes!");

    controller::run(_client).await?;

    Ok(())
}
