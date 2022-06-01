use async_trait::async_trait;
use serde_json::Value;

use crate::{payload::QueryOptions, sealed::Sealed, Client, ConsulResult};

/// This trait provides the ability to interact with the Secure Session API.
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CAConfig {
    #[serde(rename = "Provider")]
    provider: String,
    #[serde(rename = "Config")]
    config: Value,
    #[serde(rename = "CreateIndex")]
    create_index: u64,
    #[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARootList {
    #[serde(rename = "ActiveRootID")]
    active_root_id: String,
    #[serde(rename = "TrustDomain")]
    trust_domain: String,
    #[serde(rename = "Roots")]
    roots: Vec<CARoot>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARoot {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "RootCert")]
    root_cert: String,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "CreateIndex")]
    create_index: u64,
    #[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

#[async_trait]
pub trait ConnectCA: Sealed {
    async fn list_ca_root_certs(&self, options: Option<QueryOptions>) -> ConsulResult<CARootList>;
    async fn get_ca_config(&self, options: Option<QueryOptions>) -> ConsulResult<CAConfig>;
    async fn update_ca_config(
        &self,
        conf: CAConfig,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()>;
}

#[async_trait]
impl ConnectCA for Client {
    /// See the [API documentation] for more information.
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    async fn list_ca_root_certs(&self, options: Option<QueryOptions>) -> ConsulResult<CARootList> {
        self.get("/v1/connect/ca/roots", options).await
    }

    /// See the [API documentation] for more information.
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#get-ca-configuration
    async fn get_ca_config(&self, options: Option<QueryOptions>) -> ConsulResult<CAConfig> {
        self.get("/v1/connect/ca/configuration", options).await
    }

    /// See the [API documentation] for more information.
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#update-ca-configuration
    async fn update_ca_config(
        &self,
        payload: CAConfig,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()> {
        self.put("/v1/connect/ca/configuration", payload, None, options).await
    }
}
