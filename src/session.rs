use std::collections::HashMap;
use std::time::Duration;


use {Client, QueryOptions, QueryMeta, WriteOptions, WriteMeta};
use errors::Result;
use request::{get, put};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct SessionCreate {
    Name: String,
    TTL: String,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct SessionID {
    ID: String,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct SessionEntry {
    CreateIndex: String,
    ID: String,
    Name: String,
    Node: String,
    LockDelay: Duration,
    Behavior: Option<String>,
    Checks: Vec<String>,
}

pub trait Session {
    fn create(
        &self,
        session: &SessionEntry,
        options: Option<&WriteOptions>,
    ) -> Result<(SessionID, WriteMeta)>;
    fn destroy(&self, id: &str, options: Option<&WriteOptions>) -> Result<((), WriteMeta)>;
    fn info(&self, id: &str, options: Option<&QueryOptions>) -> Result<(SessionEntry, QueryMeta)>;
    fn list(&self, options: Option<&QueryOptions>) -> Result<(Vec<SessionEntry>, QueryMeta)>;
    fn node(
        &self,
        node: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<SessionEntry>, QueryMeta)>;
    fn renew(&self, id: &str, options: Option<&WriteOptions>) -> Result<(SessionEntry, WriteMeta)>;
}

impl Session for Client {
    fn create(
        &self,
        session: &SessionEntry,
        options: Option<&WriteOptions>,
    ) -> Result<(SessionID, WriteMeta)> {
        put(
            "/v1/session/create/",
            Some(session),
            &self.config,
            HashMap::new(),
            options,
        )
    }
    fn destroy(&self, id: &str, options: Option<&WriteOptions>) -> Result<((), WriteMeta)> {
        let path = format!("/v1/session/destroy/{}", id);
        put(
            &path,
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            options,
        )
    }
    fn info(&self, id: &str, options: Option<&QueryOptions>) -> Result<(SessionEntry, QueryMeta)> {
        let path = format!("/v1/session/info/{}", id);
        get(&path, &self.config, HashMap::new(), options)
    }
    fn list(&self, options: Option<&QueryOptions>) -> Result<(Vec<SessionEntry>, QueryMeta)> {
        get("/session/list", &self.config, HashMap::new(), options)
    }
    fn node(
        &self,
        node: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<SessionEntry>, QueryMeta)> {

        let path = format!("/v1/session/node/{}", node);
        get(&path, &self.config, HashMap::new(), options)
    }

    fn renew(&self, id: &str, options: Option<&WriteOptions>) -> Result<(SessionEntry, WriteMeta)> {
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
