extern crate consul;
use consul::{Client, Config, KVPair};

#[tokio::test]
async fn kv_test() {
    use consul::KV;
    let config = Config::new();
    let client = Client::new(config);
    let r = client.list("", None).await.unwrap();
    assert!(r.is_empty());

    let pair = KVPair {
        key: String::from("testkey"),
        value: String::from("testvalue"),
        ..Default::default()
    };

    assert!(client.put(&pair, None).await.unwrap());

    let b64val = client.get("testkey", None).await.unwrap().into_iter().next().unwrap().value;
    let bytes = base64::decode(b64val).unwrap();
    assert_eq!(std::str::from_utf8(&bytes).unwrap(), "\"testvalue\"");

    let r = client.list("t", None).await.unwrap();
    assert!(!r.is_empty());

    client.delete("testkey", None).await.unwrap();

    let r = client.list("", None).await.unwrap();
    assert!(r.is_empty());
}
