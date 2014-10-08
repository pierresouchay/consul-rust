use std::collections::HashMap;
use agent::Agent;
use catalog::Catalog;
use health::{Health, HealthService};
use structs::Service;


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
    let list: Vec<HealthService> = health.service("redis", "release");
    assert_eq!(list.len(), 1);
    let list: Vec<HealthService> = health.service("redis", "release2");
    assert_eq!(list.len(), 0);
}
