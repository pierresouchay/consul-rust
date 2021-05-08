use std::collections::HashMap;

use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct SessionID {
    pub ID: String,
}

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct SessionEntry {
    pub CreateIndex: Option<u64>,
    pub ID: Option<String>,
    pub Name: Option<String>,
    pub Node: Option<String>,
    pub LockDelay: Option<u64>, //TODO: Change this to a Durations
    pub Behavior: Option<String>,
    pub Checks: Option<Vec<String>>,
    pub TTL: Option<String>,
}

pub trait Session {
    fn create(
        &self,
        session: &SessionEntry,
        options: Option<&WriteOptions>,
    ) -> Result<(SessionEntry, WriteMeta)>;
    fn destroy(&self, id: &str, options: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn info(
        &self,
        id: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<SessionEntry>, QueryMeta)>;
    fn list(&self, options: Option<&QueryOptions>) -> Result<(Vec<SessionEntry>, QueryMeta)>;
    fn node(
        &self,
        node: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<SessionEntry>, QueryMeta)>;
    fn renew(
        &self,
        id: &str,
        options: Option<&WriteOptions>,
    ) -> Result<(Vec<SessionEntry>, WriteMeta)>;
}

impl Session for Client {
    fn create(
        &self,
        session: &SessionEntry,
        options: Option<&WriteOptions>,
    ) -> Result<(SessionEntry, WriteMeta)> {
        put(
            "/v1/session/create",
            Some(session),
            &self.config,
            HashMap::new(),
            options,
        )
    }
    fn destroy(&self, id: &str, options: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let path = format!("/v1/session/destroy/{}", id);
        put(
            &path,
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            options,
        )
    }
    fn info(
        &self,
        id: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<SessionEntry>, QueryMeta)> {
        let path = format!("/v1/session/info/{}", id);
        get(&path, &self.config, HashMap::new(), options)
    }
    fn list(&self, options: Option<&QueryOptions>) -> Result<(Vec<SessionEntry>, QueryMeta)> {
        get("/v1/session/list", &self.config, HashMap::new(), options)
    }
    fn node(
        &self,
        node: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<SessionEntry>, QueryMeta)> {
        let path = format!("/v1/session/node/{}", node);
        get(&path, &self.config, HashMap::new(), options)
    }

    fn renew(
        &self,
        id: &str,
        options: Option<&WriteOptions>,
    ) -> Result<(Vec<SessionEntry>, WriteMeta)> {
        let path = format!("/v1/session/renew/{}", id);
        put(
            &path,
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            options,
        )
    }
}
