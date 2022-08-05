//! Integration tests for the KV API. Since these methods interact with each
//! other, they are tested in a single integration test.

extern crate consul_oxide;
use consul_oxide::{Client, Config, KVPair, KV};

#[tokio::test]
async fn test_kv_methods() {
    let config = Config::default();
    let client = Client::new(config);
    let r = client.list_entries("", None).await.unwrap();
    assert!(r.is_empty());

    let pair = KVPair {
        key: String::from("testkey"),
        value: String::from("testvalue"),
        ..Default::default()
    };

    assert!(client.put_entry(&pair, None).await.unwrap());

    let b64val = client.get_entry("testkey", None).await.unwrap().into_iter().next().unwrap().value;
    let bytes = base64::decode(b64val).unwrap();
    assert_eq!(std::str::from_utf8(&bytes).unwrap(), "\"testvalue\"");

    let r = client.list_entries("t", None).await.unwrap();
    assert!(!r.is_empty());

    client.delete_entry("testkey", None).await.unwrap();

    let r = client.list_entries("", None).await.unwrap();
    assert!(r.is_empty());
}
