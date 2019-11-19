extern crate consul;
use consul::kv::{KVPair, KV};
use consul::{Client, Config};

use mockito::mock;

#[test]
fn kv_test() {
    use consul::kv::KV;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());

    let pair = KVPair {
        Key: String::from("testkey"),
        Value: Some(String::from("testvalue")),
        ..Default::default()
    };

    assert!(client.put(&pair, None).unwrap().0);

    let r = client.list("t", None).unwrap();
    assert!(!r.0.is_empty());

    client.delete("testkey", None).unwrap();

    let r = client.list("", None).unwrap();
    assert!(r.0.is_empty());
}

#[test]
fn test_null_value() {
    let _m = mock("GET", "/v1/kv/a-path/?recurse=")
        .with_status(201)
        .with_header("Content-Type", "application/json")
        .with_body("[{\"LockIndex\":0,\"Key\":\"a-path/\",\"Flags\":0,\"Value\":null,\"CreateIndex\":84,\"ModifyIndex\":84}]")
        .create();

    let expected: Vec<KVPair> = vec![KVPair {
        Key: String::from("a-path/"),
        Value: None,
        CreateIndex: Some(84),
        ModifyIndex: Some(84),
        LockIndex: Some(0),
        Flags: Some(0),
        ..Default::default()
    }];

    let mockito_address = mockito::server_url();

    let mut config = consul::Config::new().unwrap();

    config.address = mockito_address;

    let client = consul::Client::new(config);
    let res = client.list("a-path/", None);

    assert_eq!(expected, res.unwrap().0);
}
