extern crate consul;
use consul::session::SessionEntry;
use consul::{Client, Config};

#[test]
fn session_test() {
    use consul::session::Session;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.list(None).unwrap();
    assert!(r.0.is_empty());

    let entry = SessionEntry {
        Name: Some(String::from("test session")),
        ..Default::default()
    };

    let id = client.create(&entry, None).unwrap().0.ID.unwrap();

    client.renew(&id, None).unwrap();

    client.destroy(&id, None).unwrap();
}
