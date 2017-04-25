extern crate consul;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

#[test]
pub fn test_agent() {
    let client = consul::Client::new("http://127.0.0.1:8500");
    let services: HashMap<String, consul::Service> = client.agent.services().unwrap();
    assert!(services.contains_key("consul"));
    let members:  Vec<consul::AgentMember> = client.agent.members().unwrap();
    assert!(!members.is_empty());
    println!("Members: {}", serde_json::to_string(&members).unwrap())
}

#[test]
pub fn test_catalog(){
    let client = consul::Client::new("http://127.0.0.1:8500");
    let map: HashMap<String, Vec<String>> = client.catalog.services().unwrap();
    assert!(map.contains_key("consul"));
}


#[test]
pub fn test_health(){
    let client = consul::Client::new("http://127.0.0.1:8500");
    let list: Vec<consul::HealthService> = client.health.service("consul", None).unwrap();
    assert!(!list.is_empty());
    println!("Consul nodes: {}", serde_json::to_string(&list).unwrap());
}
