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
//! let client = Client::new("127.0.0.1:8500");
//! let services: HashMap<String, Service> = client.agent.services();
//! println!("{:?}", services);
//! ```
//!

#![crate_name = "consul"]
#![crate_type = "lib"]

extern crate curl;
extern crate rustc_serialize;

/// public api
pub use agent::{Agent, AgentMember};
pub use catalog::Catalog;
pub use health::Health;
pub use client::Client;
pub use structs::{Node, Service, HealthService, RegisterService};

mod agent;
mod catalog;
mod structs;
mod health;
mod client;
