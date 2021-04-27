use std::collections::HashMap;

use serde_json::Value;

use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[serde(default)]
#[derive(Default, Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct CAConfig {
    Provider: String,
    Config: Value,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[serde(default)]
#[derive(Default, Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARootList {
    ActiveRootID: String,
    TrustDomain: String,
    Roots: Vec<CARoot>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARoot {
    ID: String,
    Name: String,
    RootCert: String,
    Active: bool,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[allow(clippy::upper_case_acronyms)]
pub trait ConnectCA {
    fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)>;
    fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)>;
    fn ca_set_config(&self, conf: &CAConfig, q: Option<&WriteOptions>) -> Result<((), WriteMeta)>;
}

impl ConnectCA for Client {
    /// https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)> {
        get("/v1/connect/ca/roots", &self.config, HashMap::new(), q)
    }

    /// https://www.consul.io/api/connect/ca.html#get-ca-configuration
    fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)> {
        get(
            "/v1/connect/ca/configuration",
            &self.config,
            HashMap::new(),
            q,
        )
    }

    /// https://www.consul.io/api/connect/ca.html#update-ca-configuration
    fn ca_set_config(&self, conf: &CAConfig, q: Option<&WriteOptions>) -> Result<((), WriteMeta)> {
        put(
            "/v1/connect/ca/configuration",
            Some(conf),
            &self.config,
            HashMap::new(),
            q,
        )
    }
}
