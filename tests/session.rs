extern crate consul;
use consul::session::SessionEntry;
use consul::{Client, Config};

#[test]
fn session_test() {
    use consul::session::Session;
    let config = Config::builder().build().unwrap();
    let client = Client::new(config);
    let r = client.list(None).unwrap();
    assert!(r.is_empty());

    let id = client.create(None).unwrap();

    client.renew(&id, None).unwrap();

    client.destroy(&id, None).unwrap();
}
