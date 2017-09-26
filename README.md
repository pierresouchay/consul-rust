## consul-rust

[![Build Status](https://travis-ci.org/stusmall/consul-rust.svg)](https://travis-ci.org/stusmall/consul-rust.svg)
[![](https://img.shields.io/crates/v/consul.svg)](https://crates.io/crates/consul)

See the documentation [here](https://docs.rs/consul/).

Rust client library for the [Consul](https://consul.io/) HTTP API.

### Usage

```Rust
extern crate consul;

use consul::Client;
use std::collections::HashMap;

fn main() {
    let client = Client::new("http://127.0.0.1:8500");
    let services: HashMap<String, Vec<String>> = client.catalog.services().unwrap();
    println!("{:?}", services);
}
```


For more examples, see the **[tests](https://github.com/stusmall/consul-rust/blob/master/tests/example.rs)**.

### Installation

Simply include consul-rust in your Cargo dependencies.

```toml
[dependencies]
consul = "0.1"
```
