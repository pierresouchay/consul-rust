#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

//! # consul-rust
//! ```rust
//! #![allow(unused_must_use)]
//! use consul::Client;
//! use consul::Config;
//! use consul::catalog::Catalog;
//! 
//! fn main(){
//!     let mut config = Config::new().unwrap();
//!     config.address = String::from("http://localhost:8500");
//!     let client = Client::new(config);
//!     let nodes = client.nodes(None);
//!     nodes.map(|(nodes, _)|{
//!         println!("nodes: {:?}", nodes);
//!         for node in nodes.iter() {
//!             println!("node {}: {:?}", node.ID, client.node(node.ID.as_str(), None));
//!         }
//!     });
//! 
//!     let res = client.services(None);
//!     res.map(|(m, _)|{
//!         println!("services: {:?}", m);
//!         for key in m.keys() {
//!             let service = client.service(key.as_str(), None);
//!             println!("service {}: {:?}", key, service);
//!         }
//!     });
//! }
//! ```


#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;


pub mod agent;
pub mod catalog;
pub mod connect_ca;
pub mod errors;
pub mod health;
pub mod kv;
pub mod session;

mod request;

use std::time::Duration;


use reqwest::ClientBuilder;
use reqwest::Client as HttpClient;

use errors::{Result, ResultExt};



#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config: config }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub datacenter: Option<String>,
    pub http_client: HttpClient,
    pub wait_time: Option<Duration>,
}

impl Config {
    pub fn new() -> Result<Config> {
       ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: String::from("http://localhost:8500"),
                datacenter: None,
                http_client: client,
                wait_time: None,
            })
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
