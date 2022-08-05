extern crate consul;
use consul::{Client, Config};

#[tokio::test]
async fn health_test() {
    use consul::Health;
    let config = Config::default();
    let client = Client::new(config);
    // An existing service for a agent in dev mode
    let snodes =
        client.list_service_instances("consul", Option::None, true, Option::None).await.unwrap();
    {
        assert!(!snodes.is_empty(), "should have at least one Service Node");
    }
    // A non existing, should be empty
    let snodes = client
        .list_service_instances("non-existing-service", Option::None, true, Option::None)
        .await
        .unwrap();
    {
        assert_eq!(snodes.len(), 0);
    }
}
