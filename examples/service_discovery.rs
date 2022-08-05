use consul::{AgentServices, Client, Config, ServiceRegistrationPayload};

#[tokio::main]
async fn main() {
    let client = Client::new(Config::new());
    client
        .register_service(ServiceRegistrationPayload {
            name: "My Service".to_string(),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            port: 8080,
            ..Default::default()
        })
        .await
        .unwrap();
    // print
    println!("{:?}", client.list_local_services().await.unwrap());
}
