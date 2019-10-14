extern crate consul;
use consul::kv::KVPair;
use consul::{Client, Config};
use serde_json::Value;

#[test]
fn kv_test() {
    use consul::kv::KV;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());

    let pair = KVPair {
        Key: String::from("testkey"),
        Value: Value::from(6),
        ..Default::default()
    };

    assert!(client.put(&pair, None).unwrap().0);

    assert_eq!(
        Value::from(6),
        client.get("testkey", None).unwrap().0.unwrap().Value
    );

    let r = client.list("t", None).unwrap();
    assert!(!r.0.is_empty());

    client.delete("testkey", None).unwrap();

    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());
}
