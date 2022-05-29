extern crate consul;
use consul::kv::KVPair;
use consul::{Client, Config};

#[tokio::test]
async fn kv_test() {
    use consul::kv::KV;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.list("", None).await.unwrap();
    assert!(r.0.is_empty());

    let pair = KVPair {
        key: String::from("testkey"),
        value: String::from("testvalue"),
        ..Default::default()
    };

    assert!(client.put(&pair, None).await.unwrap().0);

    let b64val = client.get("testkey", None).await.unwrap().0.unwrap().value;
    let bytes = base64::decode(b64val).unwrap();
    assert_eq!(std::str::from_utf8(&bytes).unwrap(), "\"testvalue\"");

    let r = client.list("t", None).await.unwrap();
    assert!(!r.0.is_empty());

    client.delete("testkey", None).await.unwrap();

    let r = client.list("", None).await.unwrap();
    assert!(r.0.is_empty());
}
