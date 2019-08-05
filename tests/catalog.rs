extern crate consul;
use consul::catalog::Catalog;
use consul::{Client, Config};

#[test]
fn ds_test() {
    use consul::catalog::Catalog;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.datacenters().unwrap();
    assert_eq!(r.0, ["dc1"]);
}
