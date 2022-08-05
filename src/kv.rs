use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::Method;

use crate::{sealed::Sealed, Client, ConsulError, ConsulResult, QueryOptions};

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
    async fn acquire_entry(&self, _: &KVPair, _: Option<QueryOptions>) -> ConsulResult<bool>;
    async fn delete_entry(&self, _: &str, _: Option<QueryOptions>) -> ConsulResult<bool>;
    async fn get_entry(&self, _: &str, _: Option<QueryOptions>) -> ConsulResult<Vec<KVPair>>;
    async fn list_entries(&self, _: &str, _: Option<QueryOptions>) -> ConsulResult<Vec<KVPair>>;
    async fn put_entry(&self, _: &KVPair, _: Option<QueryOptions>) -> ConsulResult<bool>;
    async fn release_entry(&self, _: &KVPair, _: Option<QueryOptions>) -> ConsulResult<bool>;
}

#[async_trait]
impl KV for Client {
    async fn acquire_entry(
        &self,
        pair: &KVPair,
        options: Option<QueryOptions>,
    ) -> ConsulResult<bool> {
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
            Err(ConsulError::MissingParameter("session_flag".to_owned()))
        }
    }

    async fn delete_entry(&self, key: &str, options: Option<QueryOptions>) -> ConsulResult<bool> {
        let path = format!("/v1/kv/{}", key);
        self.delete(&path, None, options).await
    }
    async fn get_entry(
        &self,
        key: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<KVPair>> {
        let path = format!("/v1/kv/{}", key);
        self.get(&path, options).await
    }

    async fn list_entries(
        &self,
        prefix: &str,
        o: Option<QueryOptions>,
    ) -> ConsulResult<Vec<KVPair>> {
        let mut params = HashMap::new();
        // enable key mode
        params.insert(String::from("recurse"), String::from(""));
        let path = format!("/v1/kv/{}", prefix);
        // use send with empty as consul returns invalid json
        self.send_with_empty(Method::GET, path, Some(params), None as Option<()>, o)
            .await
            .map(|r: Option<Vec<KVPair>>| r.unwrap_or_default())
    }

    async fn put_entry(&self, pair: &KVPair, o: Option<QueryOptions>) -> ConsulResult<bool> {
        let mut params = HashMap::new();
        if let Some(i) = pair.flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        let path = format!("/v1/kv/{}", pair.key);
        self.put(&path, &pair.value, None, o).await
    }

    async fn release_entry(&self, pair: &KVPair, o: Option<QueryOptions>) -> ConsulResult<bool> {
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
            Err(ConsulError::MissingParameter("session_flag".to_owned()))
        }
    }
}
