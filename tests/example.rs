extern crate rustc_serialize;
extern crate consul;

use std::collections::HashMap;

use rustc_serialize::json;


#[test]
pub fn test_agent() {
    let client = consul::Client::new("127.0.0.1:8500");
    let services: HashMap<String, consul::Service> = client.agent.services();
    assert!(services.contains_key("consul"));
    let members:  Vec<consul::AgentMember> = client.agent.members();
    assert_eq!(members.len(), 1);
    println!("{:?}", json::encode(&members))
}

#[test]
pub fn test_catalog(){
    let client = consul::Client::new("127.0.0.1:8500");
    let map: HashMap<String, Vec<String>> = client.catalog.services();
    assert!(map.contains_key("gsearch"));
}


#[test]
pub fn test_health(){
    let client = consul::Client::new("127.0.0.1:8500");
    let list1: Vec<consul::HealthService> = client.health.service("gsearch", "release");
    assert_eq!(list1.len(), 1);
    let list2: Vec<consul::HealthService> = client.health.service("redis", "release2");
    assert_eq!(list2.len(), 0);
    println!("{:?}", json::encode(&list1));
}
