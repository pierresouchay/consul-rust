## consul-rust (Incomplete implement)

[![Build Status](https://travis-ci.org/youngking/consul-rust.svg)](https://travis-ci.org/youngking/consul-rust)


Rust client libray for [Consul](http://consul.io/) HTTP API

### Usage

```
    extern crate consul;

    use std::collections::HashMap;

    fn main(){
        let catalog1 = consul::catalog::Catalog::new("127.0.0.1:8500");
        let services: HashMap<String, Vec<String>> = catalog1.services();
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
