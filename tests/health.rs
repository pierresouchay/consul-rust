extern crate consul;
use consul::{Client, Config};

#[tokio::test]
async fn health_test() {
    use consul::Health;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    // An existing service for a agent in dev mode
    let r =
        client.list_service_instances("consul", Option::None, true, Option::None).await.unwrap();
    let (snodes, meta) = (r.0, r.1);
    {
        assert!(!snodes.is_empty(), "should have at least one Service Node");
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing, should be empty
    let r = client
        .list_service_instances("non-existing-service", Option::None, true, Option::None)
        .await
        .unwrap();
    let (snodes, meta) = (r.0, r.1);
    {
        assert_eq!(snodes.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
}
