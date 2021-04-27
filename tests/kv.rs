extern crate base64;

extern crate consul;
use consul::kv::{KVPair, KV};
use consul::{Client, Config};

extern crate rand;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::str;

#[test]
fn kv_add_test() {
    let (client, unique_test_path) = set_up();

    let kv_list_result = client.list(&unique_test_path, None).unwrap();
    assert!(kv_list_result.0.len() == 3);

    let new_kv_pair = KVPair {
        Key: format!("{}/newkey", unique_test_path),
        Value: String::from("newvalue"),
        ..Default::default()
    };

    client.put(&new_kv_pair, None).unwrap().0;

    let kv_list_result = client.list(&unique_test_path, None).unwrap();
    assert!(kv_list_result.0.len() == 4);

    tear_down(client, &unique_test_path);
}

#[test]
fn kv_delete_test() {
    let (client, unique_test_path) = set_up();

    let kv_list_result = client.list(&unique_test_path, None).unwrap();
    assert!(kv_list_result.0.len() == 3);

    let key_for_deletion = format!("{}/secondkey", unique_test_path);
    client.delete(&key_for_deletion, None).unwrap();

    let kv_list_result = client.list(&unique_test_path, None).unwrap();
    assert!(kv_list_result.0.len() == 2);

    let actual_key_names = kv_list_result
        .0
        .iter()
        .map(|kv| kv.Key.split("/").skip(1).next().unwrap())
        .collect::<Vec<&str>>();

    let expected_key_names = vec!["firstkey", "thirdkey"];

    assert_eq!(actual_key_names, expected_key_names);

    tear_down(client, &unique_test_path);
}

#[test]
fn kv_get_test() {
    let (client, unique_test_path) = set_up();

    let key_to_get = format!("{}/secondkey", unique_test_path);
    let kv_pair = client.get(&key_to_get, None).unwrap();

    assert!(kv_pair.0.is_some());
    assert!(kv_pair.0.unwrap().Value == "\"secondvalue\"");

    tear_down(client, &unique_test_path);
}

#[test]
fn kv_list_test() {
    let (client, unique_test_path) = set_up();

    let kv_list_result = client.list(&unique_test_path, None).unwrap();
    assert!(kv_list_result.0.len() == 3);

    let actual_key_names = kv_list_result
        .0
        .iter()
        .map(|kv| kv.Key.split("/").skip(1).next().unwrap())
        .collect::<Vec<&str>>();

    let expected_key_names = vec!["firstkey", "secondkey", "thirdkey"];

    assert_eq!(actual_key_names, expected_key_names);

    tear_down(client, &unique_test_path);
}

#[test]
fn kv_put_test() {
    let (client, unique_test_path) = set_up();

    let updated_kv_pair = KVPair {
        Key: format!("{}/secondkey", unique_test_path),
        Value: String::from("updatedsecondvalue"),
        ..Default::default()
    };

    client.put(&updated_kv_pair, None).unwrap();

    let key_to_get = format!("{}/secondkey", unique_test_path);
    let kv_pair = client.get(&key_to_get, None).unwrap();

    assert!(kv_pair.0.is_some());
    assert!(kv_pair.0.unwrap().Value == "\"updatedsecondvalue\"");

    tear_down(client, &unique_test_path);
}

fn set_up() -> (Client, String) {
    let config = Config::new().unwrap();
    let client = Client::new(config);

    let unique_test_path: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    let kv_pairs = vec![
        KVPair {
            Key: format!("{}/firstkey", unique_test_path),
            Value: String::from("firstvalue"),
            ..Default::default()
        },
        KVPair {
            Key: format!("{}/secondkey", unique_test_path),
            Value: String::from("secondvalue"),
            ..Default::default()
        },
        KVPair {
            Key: format!("{}/thirdkey", unique_test_path),
            Value: String::from("thirdvalue"),
            ..Default::default()
        },
    ];

    for kv_pair in kv_pairs {
        client.put(&kv_pair, None).unwrap().0;
    }

    return (client, unique_test_path);
}

fn tear_down(client: Client, unique_test_path: &str) {
    let kv_list_result = client.list(unique_test_path, None).unwrap();

    for kv_pair in kv_list_result.0 {
        client.delete(&kv_pair.Key, None).unwrap();
    }
}
