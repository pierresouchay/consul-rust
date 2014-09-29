use std::collections::HashMap;
use agent::{Agent, Service};


#[test]
pub fn test_agent() {
    let agent1 = Agent::new("http://localhost:8500/v1");
    let map: HashMap<String, Service> = agent1.services();
    assert!(map.contains_key(& String::from_str("redis")));
    // assert_eq!(vec!["hello"], agent1.services())
}
