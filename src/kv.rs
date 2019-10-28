use std::collections::HashMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::request::{delete, get, get_vec, put_data};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

use base64::STANDARD;

base64_serde_type!(Base64Standard, STANDARD);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Value(#[serde(with = "Base64Standard")] pub Vec<u8>);

#[serde(default)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct KVPair {
    pub Key: String,
    pub CreateIndex: Option<u64>,
    pub ModifyIndex: Option<u64>,
    pub LockIndex: Option<u64>,
    pub Flags: Option<u64>,
    pub Value: Option<Value>,
    pub Session: Option<String>,
}

pub trait KV {
    fn acquire(&self, _: KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn delete(&self, _: &str, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn get(&self, _: &str, _: Option<&QueryOptions>) -> Result<(Option<KVPair>, QueryMeta)>;
    fn list(&self, _: &str, _: Option<&QueryOptions>) -> Result<(Vec<KVPair>, QueryMeta)>;
    fn put(&self, _: KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn release(&self, _: KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
}

impl KV for Client {
    fn acquire(&self, pair: KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.Session {
            params.insert(String::from("acquire"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.Key);
            put_data(&path, pair.Value.map(|x| x.0), &self.config, params, o)
        } else {
            Err(Error::from("Session flag is required to acquire lock"))
        }
    }

    fn delete(&self, key: &str, options: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let path = format!("/v1/kv/{}", key);
        delete(&path, &self.config, HashMap::new(), options)
    }
    fn get(
        &self,
        key: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Option<KVPair>, QueryMeta)> {
        let path = format!("/v1/kv/{}", key);
        let x: Result<(Vec<KVPair>, QueryMeta)> = get(&path, &self.config, HashMap::new(), options);
        x.map(|r| (r.0.first().cloned(), r.1))
    }

    fn list(&self, prefix: &str, o: Option<&QueryOptions>) -> Result<(Vec<KVPair>, QueryMeta)> {
        let mut params = HashMap::new();
        params.insert(String::from("recurse"), String::from(""));
        let path = format!("/v1/kv/{}", prefix);
        get_vec(&path, &self.config, params, o)
    }

    fn put(&self, pair: KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        let path = format!("/v1/kv/{}", pair.Key);
        put_data(&path, pair.Value.map(|x| x.0), &self.config, params, o)
    }

    fn release(&self, pair: KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.Session {
            params.insert(String::from("release"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.Key);
            put_data(&path, pair.Value.map(|x| x.0), &self.config, params, o)
        } else {
            Err(Error::from("Session flag is required to release a lock"))
        }
    }
}
