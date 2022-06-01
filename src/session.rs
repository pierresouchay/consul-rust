use async_trait::async_trait;

use crate::{payload::QueryOptions, sealed::Sealed, Client, ConsulResult};

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct SessionID {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct SessionEntry {
    #[serde(rename = "CreateIndex")]
    pub createindex: Option<u64>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Node")]
    pub node: Option<String>,
    #[serde(rename = "LockDelay")]
    pub lockdelay: Option<u64>, //delay: Change this to a Durations
    #[serde(rename = "Behavior")]
    pub behavior: Option<String>,
    #[serde(rename = "Checks")]
    pub checks: Option<Vec<String>>,
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
}

#[async_trait]
pub trait Session: Sealed {
    async fn create(
        &self,
        session: SessionEntry,
        options: Option<QueryOptions>,
    ) -> ConsulResult<SessionEntry>;
    async fn destroy(&self, id: &str, options: Option<QueryOptions>) -> ConsulResult<bool>;
    async fn info(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>>;
    async fn list(&self, options: Option<QueryOptions>) -> ConsulResult<Vec<SessionEntry>>;
    async fn node(
        &self,
        node: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>>;
    async fn renew(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>>;
}

#[async_trait]
impl Session for Client {
    async fn create(
        &self,
        session: SessionEntry,
        options: Option<QueryOptions>,
    ) -> ConsulResult<SessionEntry> {
        self.put("/v1/session/create", session, None, options).await
    }
    async fn destroy(&self, id: &str, options: Option<QueryOptions>) -> ConsulResult<bool> {
        let path = format!("/v1/session/destroy/{}", id);
        self.put(&path, None as Option<&()>, None, options).await
    }
    async fn info(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>> {
        let path = format!("/v1/session/info/{}", id);
        self.get(&path, options).await
    }
    async fn list(&self, options: Option<QueryOptions>) -> ConsulResult<Vec<SessionEntry>> {
        self.get("/v1/session/list", options).await
    }
    async fn node(
        &self,
        node: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>> {
        let path = format!("/v1/session/node/{}", node);
        self.get(&path, options).await
    }

    async fn renew(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>> {
        let path = format!("/v1/session/renew/{}", id);
        self.put(&path, None as Option<&()>, None, options).await
    }
}
