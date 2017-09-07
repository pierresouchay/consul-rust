
extern crate consul;
use consul::{Client, Config};
use consul::catalog::Catalog;

#[test]
fn ds_test() {
    use consul::catalog::Catalog;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.datacenters().unwrap();
    assert_eq!(r.0, ["dc1"]);
}
