extern crate consul;
use consul::kv::KVPair;
use consul::{Client, Config};

#[test]
fn kv_test() {
    use consul::kv::KV;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());

    let pair = KVPair {
        Key: String::from("testkey"),
        Value: String::from("testvalue"),
        ..Default::default()
    };

    assert!(client.put(&pair, None).unwrap().0);

    let b64val = client.get("testkey", None).unwrap().0.unwrap().Value;
    let bytes = base64::decode(b64val).unwrap();
    assert_eq!(std::str::from_utf8(&bytes).unwrap(), "\"testvalue\"");

    let r = client.list("t", None).unwrap();
    assert!(!r.0.is_empty());

    client.delete("testkey", None).unwrap();

    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());

    let pair = KVPair {
        Key: String::from("testkeyraw"),
        Value: String::from("testvalue"),
        ..Default::default()
    };

    assert!(client.put_raw(&pair, None).unwrap().0);

    let b64val = client.get("testkeyraw", None).unwrap().0.unwrap().Value;
    let bytes = base64::decode(b64val).unwrap();
    assert_eq!(std::str::from_utf8(&bytes).unwrap(), "testvalue");
}
