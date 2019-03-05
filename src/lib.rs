#![allow(unused_doc_comments)]

extern crate base64;
extern crate failure;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

pub mod agent;
pub mod catalog;
mod error;
pub mod health;
pub mod kv;
pub mod session;

mod request;

use std::time::Duration;

use reqwest::Client as HttpClient;
use reqwest::ClientBuilder;

pub use error::{Error, ErrorKind, Result};

#[derive(Clone, Debug)]
pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConfigBuilder {
    pub address: String,
    pub client: Option<HttpClient>,
    pub datacenter: Option<String>,
    pub timeout: Option<Duration>,
    pub token: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            address: String::from("http://localhost:8500"),
            client: None,
            datacenter: None,
            token: None,
            timeout: None,
        }
    }

    pub fn address<I>(&mut self, url: I) -> &mut ConfigBuilder
    where
        I: Into<String>,
    {
        self.address = url.into();
        self
    }

    pub fn client<I>(&mut self, client: HttpClient) -> &mut ConfigBuilder {
        self.client = Some(client);
        self
    }

    pub fn datacenter<I>(&mut self, datacenter: I) -> &mut ConfigBuilder
    where
        I: Into<String>,
    {
        self.datacenter = Some(datacenter.into());
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut ConfigBuilder {
        self.timeout = Some(timeout);
        self
    }

    pub fn token<I>(&mut self, token: I) -> &mut ConfigBuilder
    where
        I: Into<String>,
    {
        self.token = Some(token.into());
        self
    }

    pub fn build(&mut self) -> Result<Config> {
        let client = if let Some(client) = self.client.take() {
            client
        } else if let Some(timeout) = self.timeout {
            ClientBuilder::new().timeout(timeout).build()?
        } else {
            ClientBuilder::new().build()?
        };
        Ok(Config {
            address: self.address.to_string(),
            datacenter: self.datacenter.take(),
            token: self.token.take(),
            http_client: client,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    http_client: HttpClient,
    address: String,
    datacenter: Option<String>,
    token: Option<String>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn client(&self) -> &HttpClient {
        &self.http_client
    }
}

#[derive(Clone, Debug, Default)]
pub struct BlockingOptions<T> {
    pub wait: Option<Duration>,
    pub options: Option<T>,
}

#[derive(Clone, Debug, Default)]
pub struct BlockingResponse<T> {
    pub index: u64,
    pub body: T,
}
