## consul-rust 

[![Build Status](https://travis-ci.org/stusmall/consul-rust.svg)](https://travis-ci.org/stusmall/consul-rust.svg)
[![](https://img.shields.io/crates/v/consul.svg)](https://crates.io/crates/consul)

[Documentation here](https://docs.rs/consul/).

Rust client libray for [Consul](http://consul.io/) HTTP API

### Usage

```rust
#![allow(unused_must_use)]

use consul::Client;
use consul::Config;
use consul::catalog::Catalog;

fn main(){
    let mut config = Config::new().unwrap();
    config.address = String::from("http://localhost:8500");
    let client = Client::new(config);
    let nodes = client.nodes(None);
    nodes.map(|(nodes, _)|{
        println!("nodes: {:?}", nodes);
       for node in nodes.iter() {
           println!("node {}: {:?}", node.ID, client.node(node.ID.as_str(), None));
       }
    });

    let res = client.services(None);
    res.map(|(m, _)|{
        println!("services: {:?}", m);
        for key in m.keys() {
            let service = client.service(key.as_str(), None);
            println!("service {}: {:?}", key, service);
        }
    });
}
```


For more example, see the **[tests](https://github.com/stusmall/consul-rust/blob/master/tests/example.rs)** .

### Installation

Simply include the consul-rust in your Cargo dependencies.

```
[dependencies]
consul = "0.4"
```
