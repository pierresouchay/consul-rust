use std::collections::HashMap;

use {Client, QueryOptions, QueryMeta, WriteOptions, WriteMeta};
use errors::Error;
use errors::Result;
use request::{delete, get, get_vec, put};



#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct KVPair {
    pub Key: String,
    pub CreateIndex: Option<u64>,
    pub ModifyIndex: Option<u64>,
    pub LockIndex: Option<u64>,
    pub Flags: Option<u64>,
    pub Value: String,
    pub Session: Option<String>,
}

pub trait KV {
    fn acquire(&self, &KVPair, Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    //fn cas(&self, &WriteKVPair, Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    fn delete(&self, key: &str, Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    //fn delete_cas(&self, &WriteKVPair, Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    //fn delete_tree(&self, &str, Option<&WriteOptions>) -> Result<((), WriteMeta)>;
    fn get(&self, &str, Option<&QueryOptions>) -> Result<(Option<KVPair>, QueryMeta)>;
    //fn keys(&self, &str, &str, Option<&QueryOptions>) -> Result<(Vec<String>, QueryMeta)>;
    fn list(&self, &str, Option<&QueryOptions>) -> Result<(Vec<KVPair>, QueryMeta)>;
    fn put(&self, &KVPair, Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    //fn release(&self, &WriteKVPair, Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
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
            params.insert(String::from("aquire"), session.to_owned());
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
}
