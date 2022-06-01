//! `consul` is a library for interacting with Consul agents via their HTTP API.
//!
//! Consul is a service mesh solution providing a full featured control plane
//! with service discovery, configuration, and segmentation functionality. For
//! more information on what Consul is, read the [Consul documentation]
//!
//! [Consul documentation]: https://www.consul.io/docs/
//!
//! ## Supported Features
//!
//! The key features of Consul, and thus this crate, are:

//! * Service Discovery
//! * Health Checking
//! * KV Store
//! * Secure Service Communication
//! * Multi Datacenter Support
//!
//! `consul` aims to support all of these to the best of its ability. Each
//! feature is available as a compiler feature, and can be enabled by using the
//! `discovery`, `health`, `kv`, `ssc` and `mds` features respectively. By
//! default, all features are enabled.
//!
//! ## Usage
//!
//! The `Client` struct provides the main entry point for the library.
//! ```
//! let config = Config::new().unwrap();
//! let client = Client::new(config);
//! ```
//! You can pass in custom configuration by using the `Config` datatype. By
//! default, it will assume the Consul agent is running on localhost, on the
//! default port 8500.
//!
//! Requests can be made to the Consul agent by importing the relevant trait:
//! ```
//! use consul::Agent;
//!
//! let client = Client::new(Config::new().unwrap());
//! let agents = client.agents(false).await;
//! ```
//!
//! ## Async Support
//!
//! The library is designed to be fully async compatible, and works with both
//! the `tokio` and `async-std` runtimes. At this time, there is no blocking API
//! available. As an alternative, you can use versions of this library below
//! `0.5.0`, as these are blocking.

#![allow(unused_doc_comments)]

use thiserror::Error;

#[macro_use]
extern crate serde_derive;

use std::{env, time::Duration};

use reqwest::{Client as HttpClient, ClientBuilder};

mod agent;
mod catalog;
mod connect_ca;
mod health;
mod kv;
mod request;
mod session;

pub mod payload;

pub use agent::*;
pub use catalog::*;
pub use connect_ca::*;
pub use health::*;
pub use kv::*;
pub use session::*;

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub datacenter: Option<String>,
    pub http_client: HttpClient,
    pub token: Option<String>,
    pub wait_time: Option<Duration>,
}

impl Config {
    pub fn new() -> Config {
        let client = ClientBuilder::new().build().unwrap();
        Config {
            address: String::from("http://localhost:8500"),
            datacenter: None,
            http_client: client,
            token: None,
            wait_time: None,
        }
    }

    pub fn new_from_env() -> Config {
        let consul_addr = match env::var("CONSUL_HTTP_ADDR") {
            Ok(val) => {
                if val.starts_with("http") {
                    val
                } else {
                    format!("http://{}", val)
                }
            }
            Err(_e) => String::from("http://127.0.0.1:8500"),
        };
        let consul_token = env::var("CONSUL_HTTP_TOKEN").ok();
        let client = ClientBuilder::new().build().unwrap();
        Config {
            address: consul_addr,
            datacenter: None,
            http_client: client,
            token: consul_token,
            wait_time: None,
        }
    }

    /// Create a new `Config` with the given address.
    ///
    /// # Panics
    /// Panics if `request::Client` construction fails.
    pub fn new_from_consul_host(host: &str, port: Option<u16>, token: Option<String>) -> Config {
        let client = ClientBuilder::new().build().unwrap();
        Config {
            address: format!("{}:{}", host, port.unwrap_or(8500)),
            datacenter: None,
            http_client: client,
            token,
            wait_time: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConsulError {
    /// The Consul API returned an error.
    #[error("http request failed")]
    HttpError(#[from] reqwest::Error),
    /// A parameter was not provided.
    #[error("missing parameter, {0}")]
    MissingParameter(String),
}

/// Type alias for `Result<T, ConsulError>`.
pub type ConsulResult<T> = Result<T, ConsulError>;
pub(crate) mod sealed {
    ///! Internal module to prevent re-implementation of strictly
    /// client-related traits.
    use crate::Client;

    pub trait Sealed {}
    impl Sealed for Client {}
}
