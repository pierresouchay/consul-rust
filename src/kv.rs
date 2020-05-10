extern crate base64;

use std::collections::HashMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::request::{delete, get, get_vec, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

use serde::{Deserialize, Deserializer};

#[serde(default)]
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct KVPair {
    pub Key: String,
    pub CreateIndex: Option<u64>,
    pub ModifyIndex: Option<u64>,
    pub LockIndex: Option<u64>,
    pub Flags: Option<u64>,
    #[serde(deserialize_with = "deserialize_kv_pair_value")]
    pub Value: String,
    pub Session: Option<String>,
}

pub trait KV {
    fn acquire(&self, _: &KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn delete(&self, _: &str, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn get(&self, _: &str, _: Option<&QueryOptions>) -> Result<(Option<KVPair>, QueryMeta)>;
    fn list(&self, _: &str, _: Option<&QueryOptions>) -> Result<(Vec<KVPair>, QueryMeta)>;
    fn put(&self, _: &KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn release(&self, _: &KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
}

impl KV for Client {
    fn acquire(&self, pair: &KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.Session {
            params.insert(String::from("acquire"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.Key);
            put(&path, Some(&pair.Value), &self.config, params, o)
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

    fn put(&self, pair: &KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        let path = format!("/v1/kv/{}", pair.Key);
        put(&path, Some(&pair.Value), &self.config, params, o)
    }

    fn release(&self, pair: &KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.Session {
            params.insert(String::from("release"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.Key);
            put(&path, Some(&pair.Value), &self.config, params, o)
        } else {
            Err(Error::from("Session flag is required to release a lock"))
        }
    }
}

fn deserialize_kv_pair_value<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_base64_string: &str = Deserialize::deserialize(deserializer).unwrap();
    let decoded_byte_array = base64::decode(raw_base64_string).unwrap();
    let decoded_string = std::str::from_utf8(&decoded_byte_array).unwrap();

    return Ok(decoded_string.to_string());
}
