use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ollama.io",
    version = "v1alpha1",
    kind = "Model",
    plural = "models",
    namespaced
)]
pub struct ModelSpec {
    pub name: String,   // ex: "gemma:2b"
    pub source: String, // ex: "registry.ollama.ai"
    pub preload: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
pub struct ModelStatus {
    pub state: Option<String>,       // Pending, Downloading, Ready, Failed
    pub size: Option<String>,        // ex: "2GB"
    pub last_update: Option<String>, // timestamp
}
