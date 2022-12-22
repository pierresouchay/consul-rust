extern crate consul;
use consul::{Client, Config};

#[test]
fn health_test() {
    use consul::health::Health;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    // An existing service for a agent in dev mode
    let r = client
        .service("consul", Option::None, true, Option::None)
        .unwrap();
    let (snodes, meta) = (r.0, r.1);
    {
        assert!(!snodes.is_empty(), "should have at least one Service Node");
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing, should be empty
    let r = client
        .service("non-existing-service", Option::None, true, Option::None)
        .unwrap();
    let (snodes, meta) = (r.0, r.1);
    {
        assert_eq!(snodes.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
}
#[test]
fn health_node_test() {
    use consul::health::Health;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let system_hostname = hostname::get().unwrap().into_string().unwrap();
    // An existing service for a agent in dev mode
    let r = client
        .node(&system_hostname, Some("serfHealth"), None, None, None)
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert!(
            !services.is_empty(),
            "should have at least one Service Node"
        );
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing node, should be empty
    let r = client
        .node("non-existing-node", Some("serfHealth"), None, None, None)
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert_eq!(services.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing check, should be empty
    let r = client
        .node(
            &system_hostname,
            Some("non-existing-check"),
            None,
            None,
            None,
        )
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert_eq!(services.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing service, should be empty
    let r = client
        .node(
            &system_hostname,
            None,
            Some("non-existing-service"),
            None,
            None,
        )
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert_eq!(services.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
}
