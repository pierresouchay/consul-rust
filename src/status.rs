use std::collections::HashMap;

use crate::errors::Result;
use crate::request::get;
use crate::{Client, QueryMeta, QueryOptions};

pub type Peer = String;

pub trait Status {
    fn leader(&self, q: Option<&QueryOptions>) -> Result<(Peer, QueryMeta)>;
    fn peers(&self, q: Option<&QueryOptions>) -> Result<(Vec<Peer>, QueryMeta)>;
}

impl Status for Client {
    /// https://developer.hashicorp.com/consul/api-docs/status
    fn leader(&self, q: Option<&QueryOptions>) -> Result<(Peer, QueryMeta)> {
        get("/v1/status/leader", &self.config, HashMap::new(), q)
    }

    fn peers(&self, q: Option<&QueryOptions>) -> Result<(Vec<Peer>, QueryMeta)> {
        get("/v1/status/peers", &self.config, HashMap::new(), q)
    }
}
