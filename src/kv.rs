use std::collections::HashMap;
use std::str;
use std::str::FromStr;

use base64;
use serde::{Deserialize, Deserializer};

use error::*;
use request::{Method, Request, StatusCode};
use response::ResponseHelper;
use {BlockingOptions, BlockingResponse, Client};

// Types
#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct KvPair {
    pub key: String,
    pub create_index: u64,
    pub modify_index: u64,
    pub lock_index: u64,
    pub flags: u64,
    #[serde(deserialize_with = "base64_decode")]
    pub value: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

fn base64_decode<'de, D>(deserializer: D) -> std::result::Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(base64::decode(opt.unwrap_or_else(|| String::from("")).as_str()).unwrap_or_default())
}

// API
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GetOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct KeysOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PutOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AcquireOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ReleaseOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct DeleteOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas: Option<u64>,
}

pub trait Kv {
    fn get(&self, key: &str, options: Option<&GetOptions>) -> Result<KvPair>;
    fn list(&self, prefix: &str, options: Option<&ListOptions>) -> Result<Vec<KvPair>>;
    fn keys(&self, prefix: &str, options: Option<&KeysOptions>) -> Result<Vec<String>>;
    fn put(&self, key: &str, value: &[u8], options: Option<&PutOptions>) -> Result<bool>;
    fn acquire(&self, key: &KvPair, options: Option<&AcquireOptions>) -> Result<bool>;
    fn release(&self, key: &KvPair, options: Option<&ReleaseOptions>) -> Result<bool>;
    fn delete(&self, key: &str, options: Option<&DeleteOptions>) -> Result<bool>;
}

pub trait KvBlocking {
    fn get(
        &self,
        index: u64,
        key: &str,
        options: Option<BlockingOptions<&GetOptions>>,
    ) -> Result<BlockingResponse<Option<KvPair>>>;
    fn list(
        &self,
        index: u64,
        prefix: &str,
        options: Option<BlockingOptions<&ListOptions>>,
    ) -> Result<BlockingResponse<Vec<KvPair>>>;
    fn keys(
        &self,
        index: u64,
        prefix: &str,
        options: Option<BlockingOptions<&KeysOptions>>,
    ) -> Result<BlockingResponse<Vec<String>>>;
}

impl Kv for Client {
    /// https://www.consul.io/api/kv.html#read-key
    fn get(&self, key: &str, options: Option<&GetOptions>) -> Result<KvPair> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(separator) = &options.separator {
                params.insert(String::from("separator"), separator.to_string());
            }
        }
        let mut r =
            Request::new_with_params(&self, Method::GET, &format!("kv/{}", key), params).send()?;
        if r.status() == StatusCode::NOT_FOUND {
            return Err(crate::error::key_not_found(key));
        } else if r.status() != StatusCode::OK {
            return Err(crate::error::unexpected_response(
                r.text().unwrap_or(String::from("")),
            ));
        }
        let pairs: Vec<KvPair> = r.json().map_err(crate::error::invalid_response)?;
        pairs
            .into_iter()
            .next()
            .ok_or_else(|| crate::Error::new(crate::error::Kind::InvalidResponse))
    }

    /// https://www.consul.io/api/kv.html#read-key
    fn list(&self, prefix: &str, options: Option<&ListOptions>) -> Result<Vec<KvPair>> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("recurse"), String::from("true"));
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(separator) = &options.separator {
                params.insert(String::from("separator"), separator.to_string());
            }
        }
        let mut r = Request::new_with_params(&self, Method::GET, &format!("kv/{}", prefix), params)
            .send()?;
        if r.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        } else if r.status() != StatusCode::OK {
            return Err(crate::error::unexpected_response(
                r.text().unwrap_or(String::from("")),
            ));
        }
        r.json().map_err(crate::error::invalid_response)
    }

    /// https://www.consul.io/api/kv.html#read-key
    fn keys(&self, prefix: &str, options: Option<&KeysOptions>) -> Result<Vec<String>> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("keys"), String::from("true"));
        params.insert(String::from("recurse"), String::from("true"));
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(separator) = &options.separator {
                params.insert(String::from("separator"), separator.to_string());
            }
        }
        let mut r = Request::new_with_params(&self, Method::GET, &format!("kv/{}", prefix), params)
            .send()?;
        if r.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        } else if r.status() != StatusCode::OK {
            return Err(crate::error::unexpected_response(
                r.text().unwrap_or(String::from("")),
            ));
        }
        r.json().map_err(crate::error::invalid_response)
    }

    /// https://www.consul.io/api/kv.html#create-update-key
    fn put(&self, key: &str, value: &[u8], options: Option<&PutOptions>) -> Result<bool> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(flags) = options.flags {
                params.insert(String::from("flags"), flags.to_string());
            }
            if let Some(cas) = options.cas {
                params.insert(String::from("cas"), cas.to_string());
            }
            if let Some(acquire) = &options.acquire {
                params.insert(String::from("acquire"), acquire.to_string());
            }
            if let Some(release) = &options.release {
                params.insert(String::from("release"), release.to_string());
            }
        }
        Request::new_with_params(&self, Method::PUT, &format!("kv/{}", key), params)
            .body(value.to_vec())
            .send()?
            .parse_json()
    }

    /// https://www.consul.io/api/kv.html#create-update-key
    fn acquire(&self, kv: &KvPair, options: Option<&AcquireOptions>) -> Result<bool> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(session) = &kv.session {
            params.insert(String::from("acquire"), session.to_string());
        } else {
            return Err(crate::error::missing_session_flag());
        }
        params.insert(String::from("flags"), kv.flags.to_string());
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        Request::new_with_params(&self, Method::PUT, &format!("kv/{}", kv.key), params)
            .json(&kv.value)
            .send()?
            .parse_json()
    }

    /// https://www.consul.io/api/kv.html#create-update-key
    fn release(&self, kv: &KvPair, options: Option<&ReleaseOptions>) -> Result<bool> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(session) = &kv.session {
            params.insert(String::from("release"), session.to_string());
        } else {
            return Err(crate::error::missing_session_flag());
        }
        params.insert(String::from("flags"), kv.flags.to_string());
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        Request::new_with_params(&self, Method::PUT, &format!("kv/{}", kv.key), params)
            .json(&kv.value)
            .send()?
            .parse_json()
    }

    /// https://www.consul.io/api/kv.html#delete-key
    fn delete(&self, key: &str, options: Option<&DeleteOptions>) -> Result<bool> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(options) = options {
            if let Some(recurse) = options.recurse {
                params.insert(String::from("recurse"), recurse.to_string());
            }
            if let Some(cas) = options.cas {
                params.insert(String::from("cas"), cas.to_string());
            }
        }
        Request::new_with_params(&self, Method::DELETE, &format!("kv/{}", key), params)
            .send()?
            .parse_json()
    }
}

impl KvBlocking for Client {
    fn get(
        &self,
        index: u64,
        key: &str,
        options: Option<BlockingOptions<&GetOptions>>,
    ) -> Result<BlockingResponse<Option<KvPair>>> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("index"), index.to_string());
        if let Some(dc) = options
            .as_ref()
            .and_then(|o| o.options.and_then(|o| o.dc.as_ref()))
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(wait) = &options.wait {
                params.insert(String::from("wait"), format!("{}s", wait.as_secs()));
            }
            if let Some(options) = options.options {
                if let Some(separator) = &options.separator {
                    params.insert(String::from("separator"), separator.to_string());
                }
            }
        }
        let mut r =
            Request::new_with_params(&self, Method::GET, &format!("kv/{}", key), params).send()?;
        let index = match r.headers().get("X-Consul-Index") {
            Some(i) => u64::from_str(str::from_utf8(i.as_bytes()).map_err(crate::error::decode)?)
                .map_err(crate::error::decode)?,
            None => return Err(crate::error::missing_index())?,
        };
        let body: Option<KvPair> = match r.status() {
            StatusCode::OK => {
                let pairs: Vec<KvPair> = r.json().map_err(crate::error::invalid_response)?;
                pairs.into_iter().next()
            }
            StatusCode::NOT_FOUND => None,
            _ => {
                return Err(crate::error::unexpected_response(
                    r.text().unwrap_or(String::from("")),
                ));
            }
        };
        Ok(BlockingResponse { index, body })
    }

    fn list(
        &self,
        index: u64,
        prefix: &str,
        options: Option<BlockingOptions<&ListOptions>>,
    ) -> Result<BlockingResponse<Vec<KvPair>>> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("index"), index.to_string());
        params.insert(String::from("recurse"), String::from("true"));
        if let Some(dc) = options
            .as_ref()
            .and_then(|o| o.options.and_then(|o| o.dc.as_ref()))
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(wait) = &options.wait {
                params.insert(String::from("wait"), format!("{}s", wait.as_secs()));
            }
            if let Some(options) = options.options {
                if let Some(separator) = &options.separator {
                    params.insert(String::from("separator"), separator.to_string());
                }
            }
        }
        let mut r = Request::new_with_params(&self, Method::GET, &format!("kv/{}", prefix), params)
            .send()?;
        let index = match r.headers().get("X-Consul-Index") {
            Some(i) => u64::from_str(str::from_utf8(i.as_bytes()).map_err(crate::error::decode)?)
                .map_err(crate::error::decode)?,
            None => return Err(crate::error::missing_index())?,
        };
        let body: Vec<KvPair> = match r.status() {
            StatusCode::OK => r.json().map_err(crate::error::invalid_response)?,
            StatusCode::NOT_FOUND => Vec::new(),
            _ => {
                return Err(crate::error::unexpected_response(
                    r.text().unwrap_or(String::from("")),
                ));
            }
        };
        Ok(BlockingResponse { index, body })
    }

    fn keys(
        &self,
        index: u64,
        prefix: &str,
        options: Option<BlockingOptions<&KeysOptions>>,
    ) -> Result<BlockingResponse<Vec<String>>> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("keys"), String::from("true"));
        params.insert(String::from("index"), index.to_string());
        params.insert(String::from("recurse"), String::from("true"));
        if let Some(dc) = options
            .as_ref()
            .and_then(|o| o.options.as_ref().and_then(|o| o.dc.as_ref()))
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(wait) = &options.wait {
                params.insert(String::from("wait"), format!("{}s", wait.as_secs()));
            }
            if let Some(options) = options.options {
                if let Some(separator) = &options.separator {
                    params.insert(String::from("separator"), separator.to_string());
                }
            }
        }
        let mut r = Request::new_with_params(&self, Method::GET, &format!("kv/{}", prefix), params)
            .send()?;
        let index = match r.headers().get("X-Consul-Index") {
            Some(i) => u64::from_str(str::from_utf8(i.as_bytes()).map_err(crate::error::decode)?)
                .map_err(crate::error::decode)?,
            None => return Err(crate::error::missing_index())?,
        };
        let body: Vec<String> = match r.status() {
            StatusCode::OK => r.json().map_err(crate::error::invalid_response)?,
            StatusCode::NOT_FOUND => Vec::new(),
            _ => {
                return Err(crate::error::unexpected_response(
                    r.text().unwrap_or(String::from("")),
                ));
            }
        };
        Ok(BlockingResponse { index, body })
    }
}
