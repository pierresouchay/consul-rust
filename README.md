## consul-rust 

[![Build Status](https://travis-ci.org/stusmall/consul-rust.svg)](https://travis-ci.org/stusmall/consul-rust.svg)
[![](https://img.shields.io/crates/v/consul.svg)](https://crates.io/crates/consul)

[Documentation here](https://docs.rs/consul/).

Rust client libray for [Consul](http://consul.io/) HTTP API

### Usage

```
    extern crate consul;

    use std::collections::HashMap;
    use consul::Client;

    fn main(){
        let client = Client::new("http://127.0.0.1:8500");
        let services: HashMap<String, Vec<String>> = client.catalog.services().unwrap();
        println!("{:?}", services);
    }
```


For more example, see the **[tests](https://github.com/stusmall/consul-rust/blob/master/tests/example.rs)** .

### Installation

Simply include the consul-rust in your Cargo dependencies.

```
[dependencies]
consul = "0.1"
```
