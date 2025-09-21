use kube::{
    Client,
    api::{Api, ListParams},
    config::Context,
    runtime::{controller::Controller, watcher},
};
use tracing::*;

use crate::crd::Model;

pub async fn run(client: Client) -> anyhow::Result<()> {
    let models: Api<Model> = Api::all(client.clone());

    Controller::new(models, watcher::Config::default())
        .run(reconcile, error_policy, Context::new(()))
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("Reconciled {:?}", o),
                Err(e) => warn!("Reconcile failed: {:?}", e),
            }
        })
        .await;

    Ok(())
}
