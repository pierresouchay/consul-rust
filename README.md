## consul-rust (Incomplete implement)

[Documentation here](https://github.com/stusmall/consul-rust/).

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
consul = "*"
```
