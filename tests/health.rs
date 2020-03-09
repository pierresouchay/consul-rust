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
        assert!(snodes.len() > 0, "should have at least one Service Node");
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
