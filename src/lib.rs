#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate url;

pub mod agent;
pub mod catalog;
pub mod errors;
pub mod health;
pub mod kv;
pub mod session;

mod request;

use std::time::Duration;

use reqwest::Client as HttpClient;
use reqwest::ClientBuilder;

use errors::Result;
use errors::ResultExt;

#[derive(Clone, Debug)]
pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config: config }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigBuilder {
    pub address: String,
    pub datacenter: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            address: String::from("http://localhost:8500"),
            datacenter: None,
        }
    }

    pub fn address<I>(mut self, url: I) -> ConfigBuilder
    where
        I: Into<String>,
    {
        self.address = url.into();
        self
    }

    pub fn datacenter<I>(mut self, name: I) -> ConfigBuilder
    where
        I: Into<String>,
    {
        self.datacenter = Some(name.into());
        self
    }

    pub fn build(self) -> Result<Config> {
        ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: self.address,
                datacenter: self.datacenter,
                http_client: client,
            })
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    address: String,
    datacenter: Option<String>,
    http_client: HttpClient,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn datacenter(&self) -> &Option<String> {
        &self.datacenter
    }

    pub fn client(&self) -> &HttpClient {
        &self.http_client
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryOptions {
    pub datacenter: Option<String>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct QueryMeta {
    pub last_index: Option<u64>,
    pub request_time: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    pub datacenter: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WriteMeta {
    pub request_time: Duration,
}
