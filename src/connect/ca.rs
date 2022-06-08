use async_trait::async_trait;
use serde_json::Value;

use crate::{sealed::Sealed, Client, ConsulResult, QueryOptions};

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

/// This trait provides implementations of the Consul `/connect/ca` endpoint.
///
/// These endpoints provide tools for interacting with Connect's Certificate
/// Authority mechanism.
///
/// See the [API documentation](https://www.consul.io/api-docs/connect/ca) for more information.
#[async_trait]
pub trait ConnectCA: Sealed {
    /// See the [API documentation] for more information.
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    async fn list_ca_root_certs(&self, options: Option<QueryOptions>) -> ConsulResult<CARootList>;
    /// See the [API documentation] for more information.
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#get-ca-configuration
    async fn get_ca_config(&self, options: Option<QueryOptions>) -> ConsulResult<CAConfig>;
    /// See the [API documentation] for more information.
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#update-ca-configuration
    async fn update_ca_config(
        &self,
        conf: CAConfig,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()>;
}

#[async_trait]
impl ConnectCA for Client {
    async fn list_ca_root_certs(&self, options: Option<QueryOptions>) -> ConsulResult<CARootList> {
        self.get("/v1/connect/ca/roots", options).await
    }

    async fn get_ca_config(&self, options: Option<QueryOptions>) -> ConsulResult<CAConfig> {
        self.get("/v1/connect/ca/configuration", options).await
    }

    async fn update_ca_config(
        &self,
        payload: CAConfig,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()> {
        self.put("/v1/connect/ca/configuration", payload, None, options).await
    }
}
