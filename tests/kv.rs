extern crate consul;
use consul::{Client, Config};

#[test]
fn kv_test() {
    use consul::kv::Kv;
    let config = Config::builder().build().unwrap();
    let client = Client::new(config);
    let r = client.list("", None).unwrap();
    assert!(r.is_empty());

    assert!(client.put("testkey", "testvalue".as_bytes(), None).unwrap());

    let r = client.list("t", None).unwrap();
    assert!(!r.is_empty());

    client.delete("testkey", None).unwrap();

    let r = client.list("", None).unwrap();
    assert!(r.is_empty());
}
