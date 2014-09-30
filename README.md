## consul-rust (Incomplete implement)

Rust client libray for Consul HTTP API

### Usage

    extern crate consul;
    use consul::catalog::Catalog;

    fn main(){
        let catalog1 = Catalog::new("http://localhost:8500/v1");
        let services: HashMap<String, Vec<String>> = catalog1.services();
        println!("{}", services);
    }


For more example, see the **[tests](https://github.com/youngking/consul-rust/blob/master/src/test/basic.rs)** .

### Installation

Simply include the consul-rust in your Cargo dependencies.

    [dependencies.consul]

    git = "git@github.com:youngking/consul-rust.git"
