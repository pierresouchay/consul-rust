## consul-rust (Incomplete implement)

[![Build Status](https://travis-ci.org/youngking/consul-rust.svg)](https://travis-ci.org/youngking/consul-rust)
[![](https://img.shields.io/crates/v/consul.svg)](https://crates.io/crates/consul)



Rust client libray for [Consul](http://consul.io/) HTTP API

### Usage

```
    extern crate consul;

    use std::collections::HashMap;
    use consul::client::Client;

    fn main(){
        let client = Client::new("127.0.0.1:8500");
        let services: HashMap<String, Vec<String>> = client.catalog.services();
        println!("{}", services);
    }
```


For more example, see the **[tests](https://github.com/youngking/consul-rust/blob/master/src/test/basic.rs)** .

### Installation

Simply include the consul-rust in your Cargo dependencies.

```
[dependencies]
consul = "*"
```
