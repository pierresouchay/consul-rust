
extern crate consul;
use consul::{Client, Config};
use consul::kv::KVPair;

#[test]
fn kv_test() {
    use consul::kv::KV;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());

    let pair = KVPair{
        Key: String::from("testkey"),
        Value: String::from("testvalue"),
        ..Default::default()
    };

    assert!(client.put(&pair, None).unwrap().0);

    let r = client.list("", None).unwrap();
    assert!(!r.0.is_empty());

    client.delete("testkey", None).unwrap();

    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());

}
