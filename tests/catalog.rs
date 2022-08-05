extern crate consul_oxide;
use consul_oxide::{Client, Config};

#[tokio::test]
async fn ds_test() {
    use consul_oxide::Catalog;
    let config = Config::new_from_env();
    let client = Client::new(config);
    let r = client.list_datacenters().await.unwrap();
    assert_eq!(r, ["dc1"]);
}

#[tokio::test]
async fn ds_services_test() {
    use consul_oxide::Catalog;
    let config = Config::default();
    let client = Client::new(config);
    let r = client.list_datacenter_services(None).await.unwrap();
    assert_ne!(r.len(), 0);
    match r.get("consul") {
        None => panic!("Should have a Consul service"),
        Some(val) => assert_eq!(val.len(), 0), // consul has no tags
    }
}
