//! Rust client libray for [Consul](http://consul.io/) HTTP API
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/consul) and
//! can be used by adding `consul` to the dependencies in your
//! project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! consul = "*"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate consul;
//! ```
//! # Examples
//! ```rust
//! use std::collections::HashMap;
//! use consul::{Client, Service};
//!
//! let client = Client::new("http://127.0.0.1:8500");
//! let services: HashMap<String, Service> = client.agent.services().unwrap();
//! println!("{:?}", services);
//! ```
//!

#![crate_name = "consul"]
#![crate_type = "lib"]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate openssl;
extern crate hyper_openssl;

/// public api
pub use agent::{Agent, AgentMember};
pub use catalog::Catalog;
pub use health::Health;
pub use client::Client;
pub use keystore::Keystore;
pub use session::Session;
pub use structs::{Node, Service, HealthService, RegisterService, TtlHealthCheck};

mod agent;
mod catalog;
mod structs;
mod health;
mod client;
mod session;
mod keystore;
mod request;
mod error;

use serde_json::Value;
pub use error::ConsulResult;

#[inline]
pub fn get_string(json_data: &Value, path: &[&str]) -> Option<String> {
    let mut pointer_str = String::new();
    for entry in path.iter() {
        pointer_str = format!("{}/{}", pointer_str, entry);
    }

    json_data.pointer(&pointer_str)
        .and_then(|value| {
            value.as_str()
                .and_then(|value| Some(value.to_owned()))
        })
}

#[inline]
pub fn find_path<'a>(json_data: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut pointer_str = String::new();
    for entry in path.iter() {
        pointer_str = format!("{}/{}", pointer_str, entry);
    }

    json_data.pointer(&pointer_str)
}
