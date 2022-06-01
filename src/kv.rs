use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    errors::{Error, Result},
    payload::QueryOptions,
    sealed::Sealed,
    Client,
};

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct KVPair {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "CreateIndex")]
    pub createindex: Option<u64>,
    #[serde(rename = "ModifyIndex")]
    pub modifyindex: Option<u64>,
    #[serde(rename = "LockIndex")]
    pub lockindex: Option<u64>,
    #[serde(rename = "Flags")]
    pub flags: Option<u64>,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Session")]
    pub session: Option<String>,
}

#[async_trait]
pub trait KV: Sealed {
    async fn acquire(&self, _: &KVPair, _: Option<QueryOptions>) -> Result<bool>;
    async fn delete(&self, _: &str, _: Option<QueryOptions>) -> Result<bool>;
    async fn get(&self, _: &str, _: Option<QueryOptions>) -> Result<Option<KVPair>>;
    async fn list(&self, _: &str, _: Option<QueryOptions>) -> Result<Vec<KVPair>>;
    async fn put(&self, _: &KVPair, _: Option<QueryOptions>) -> Result<bool>;
    async fn release(&self, _: &KVPair, _: Option<QueryOptions>) -> Result<bool>;
}

#[async_trait]
impl KV for Client {
    async fn acquire(&self, pair: &KVPair, options: Option<QueryOptions>) -> Result<bool> {
        let mut params = HashMap::new();
        if let Some(i) = pair.flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.session {
            params.insert(String::from("acquire"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.key);
            self.put(&path, &pair.value, Some(params), options).await
        } else {
            Err(Error::from("Session flag is required to acquire lock"))
        }
    }

    async fn delete(&self, key: &str, options: Option<QueryOptions>) -> Result<bool> {
        let path = format!("/v1/kv/{}", key);
        self.delete(&path, None, options).await
    }
    async fn get(&self, key: &str, options: Option<QueryOptions>) -> Result<Option<KVPair>> {
        let path = format!("/v1/kv/{}", key);
        self.get(&path, options).await
    }

    async fn list(&self, prefix: &str, o: Option<QueryOptions>) -> Result<Vec<KVPair>> {
        let mut params = HashMap::new();
        params.insert(String::from("recurse"), String::from(""));
        let path = format!("/v1/kv/{}", prefix);
        self.get_with_params(&path, Some(params), o).await
    }

    async fn put(&self, pair: &KVPair, o: Option<QueryOptions>) -> Result<bool> {
        let mut params = HashMap::new();
        if let Some(i) = pair.flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        let path = format!("/v1/kv/{}", pair.key);
        self.put(&path, &pair.value, None, o).await
    }

    async fn release(&self, pair: &KVPair, o: Option<QueryOptions>) -> Result<bool> {
        let mut params = HashMap::new();
        if let Some(i) = pair.flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.session {
            params.insert(String::from("release"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.key);
            self.put(&path, &pair.value, Some(params), o).await
        } else {
            Err(Error::from("Session flag is required to release a lock"))
        }
    }
}
