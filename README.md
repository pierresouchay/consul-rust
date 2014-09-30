## consul-rust (Incomplete implement)

Rust client libray for Consul HTTP API

### Usage

    extern crate consul;
    use consul::catalog::Catalog;

    fn main(){
        let catalog1 = Catalog::new("http://localhost:8500/v1");
        let service: HashMap<String, Vec<String>> = catalog1.services();
        println!("{}", service);
    }


For more example, see the **tests** .
