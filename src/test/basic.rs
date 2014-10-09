use std::collections::HashMap;
use agent::Agent;
use catalog::Catalog;
use health::{Health, HealthService};
use structs::Service;
use serialize::json;


#[test]
pub fn test_agent() {
    let agent1 = Agent::new("http://localhost:8500/v1");
    let map: HashMap<String, Service> = agent1.services();
    assert!(map.contains_key(& String::from_str("redis")));
}

#[test]
pub fn test_catalog(){
    let catalog1 = Catalog::new("http://localhost:8500/v1");
    let map: HashMap<String, Vec<String>> = catalog1.services();
    assert!(map.contains_key(& String::from_str("redis")));
}


#[test]
pub fn test_health(){
    let health = Health::new("http://localhost:8500/v1");
    let list1: Vec<HealthService> = health.service("redis", "release");
    assert_eq!(list1.len(), 1);
    let list2: Vec<HealthService> = health.service("redis", "release2");
    assert_eq!(list2.len(), 0);
    println!("{}", json::encode(&list1));
}
