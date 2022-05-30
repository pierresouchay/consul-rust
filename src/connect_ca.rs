use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;

use crate::{
    errors::Result,
    request::{get, put},
    sealed::Sealed,
    Client, payload::{WriteOptions, QueryOptions, QueryMeta, WriteMeta},
};

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
    async fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)>;
    async fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)>;
    async fn ca_set_config(
        &self,
        conf: &CAConfig,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
}

#[async_trait]
impl ConnectCA for Client {
    /// https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    async fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)> {
        get("/v1/connect/ca/roots", &self.config, HashMap::new(), q).await
    }

    /// https://www.consul.io/api/connect/ca.html#get-ca-configuration
    async fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)> {
        get("/v1/connect/ca/configuration", &self.config, HashMap::new(), q).await
    }

    /// https://www.consul.io/api/connect/ca.html#update-ca-configuration
    async fn ca_set_config(
        &self,
        conf: &CAConfig,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put("/v1/connect/ca/configuration", Some(conf), &self.config, HashMap::new(), q).await
    }
}
