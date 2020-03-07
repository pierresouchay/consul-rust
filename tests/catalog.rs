extern crate consul;
use consul::{Client, Config};

#[test]
fn ds_test() {
    use consul::catalog::Catalog;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.datacenters().unwrap();
    assert_eq!(r.0, ["dc1"]);
}

#[test]
fn ds_services_test() {
    use consul::catalog::Catalog;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.services(Option::None).unwrap();
    assert_ne!(r.0.len(), 0);
    match r.0.get("consul") {
        None => panic!("Should have a Consul service"),
        Some(val) => assert_eq!(val.len(), 0), // consul has no tags
    }
}
