use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Deserializer};

use error::*;
use request::{Method, Request, StatusCode};
use Client;

// Types
#[serde(rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct SessionEntry {
    pub create_index: u64,
    pub modify_index: u64,
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub node: String,
    pub checks: Vec<String>,
    #[serde(deserialize_with = "duration_from_integer")]
    pub lock_delay: Duration,
    pub behavior: String,
    #[serde(rename = "TTL")]
    pub ttl: String,
}

fn duration_from_integer<'de, D>(deserializer: D) -> std::result::Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(Duration::from_nanos(opt.unwrap_or_else(|| 15000000000)))
}

// API
#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CreateOptions {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Node")]
    pub node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Checks")]
    pub checks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LockDelay")]
    pub lock_delay: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Behavior")]
    pub behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TTL")]
    pub ttl: Option<String>,
}

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub dc: Option<String>,
}
pub type DestroyOptions = Options;
pub type InfoOptions = Options;
pub type NodeOptions = Options;
pub type ListOptions = Options;
pub type RenewOptions = Options;

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct SessionID {
    #[serde(rename = "ID")]
    pub id: String,
}

pub trait Session {
    fn create(&self, options: Option<&CreateOptions>) -> Result<String>;
    fn destroy(&self, uuid: &str, options: Option<&DestroyOptions>) -> Result<bool>;
    // TODO: Blocking
    fn info(&self, uuid: &str, options: Option<&InfoOptions>) -> Result<Option<SessionEntry>>;
    // TODO: Blocking
    fn node(&self, node: &str, options: Option<&NodeOptions>) -> Result<Vec<SessionEntry>>;
    // TODO: Blocking
    fn list(&self, options: Option<&ListOptions>) -> Result<Vec<SessionEntry>>;
    fn renew(&self, uuid: &str, options: Option<&RenewOptions>) -> Result<SessionEntry>;
}

impl Session for Client {
    /// https://www.consul.io/api/session.html#create-session
    fn create(&self, options: Option<&CreateOptions>) -> Result<String> {
        let mut r = if let Some(opts) = options {
            Request::new(&self, Method::PUT, "session/create")
                .json(opts)
                .send()?
        } else {
            Request::new(&self, Method::PUT, "session/create").send()?
        };
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        let session_id: SessionID = r.json().context(ErrorKind::InvalidResponse)?;
        Ok(session_id.id)
    }

    /// https://www.consul.io/api/session.html#delete-session
    fn destroy(&self, uuid: &str, options: Option<&DestroyOptions>) -> Result<bool> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        let mut r = Request::new_with_params(
            &self,
            Method::PUT,
            &format!("session/destroy/{}", uuid),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/session.html#read-session
    fn info(&self, uuid: &str, options: Option<&InfoOptions>) -> Result<Option<SessionEntry>> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        let mut r = Request::new_with_params(
            &self,
            Method::GET,
            &format!("session/info/{}", uuid),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        let entries: Vec<SessionEntry> = r.json().context(ErrorKind::InvalidResponse)?;
        Ok(entries.into_iter().next())
    }

    /// https://www.consul.io/api/session.html#list-sessions-for-node
    fn node(&self, node: &str, options: Option<&NodeOptions>) -> Result<Vec<SessionEntry>> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        let mut r = Request::new_with_params(
            &self,
            Method::GET,
            &format!("session/node/{}", node),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/session.html#list-sessions
    fn list(&self, options: Option<&ListOptions>) -> Result<Vec<SessionEntry>> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        let mut r = Request::new_with_params(&self, Method::GET, "session/list", params).send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/session.html#renew-session
    fn renew(&self, uuid: &str, options: Option<&RenewOptions>) -> Result<SessionEntry> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        let mut r = Request::new_with_params(
            &self,
            Method::PUT,
            &format!("session/renew/{}", uuid),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        let entries: Vec<SessionEntry> = r.json().context(ErrorKind::InvalidResponse)?;
        entries
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidResponse))
    }
}
