use std::collections::HashMap;

use rustc_serialize::json;

use client::Client;
use structs::{Service, HealthService};


#[test]
pub fn test_agent() {
    let client = Client::new("127.0.0.1:8500");
    let map: HashMap<String, Service> = client.agent.services();
    assert!(map.contains_key("consul"));
}

#[test]
pub fn test_catalog(){
    let client = Client::new("127.0.0.1:8500");
    let map: HashMap<String, Vec<String>> = client.catalog.services();
    assert!(map.contains_key("gsearch"));
}


#[test]
pub fn test_health(){
    let client = Client::new("127.0.0.1:8500");
    let list1: Vec<HealthService> = client.health.service("gsearch", "release");
    assert_eq!(list1.len(), 1);
    let list2: Vec<HealthService> = client.health.service("redis", "release2");
    assert_eq!(list2.len(), 0);
    println!("{:?}", json::encode(&list1));
}
