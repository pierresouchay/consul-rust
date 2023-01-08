extern crate consul;
use consul::status::Status;
use consul::{Client, Config};

#[test]
fn status_leader_test() {
    let config = Config::new().unwrap();
    let client = Client::new(config);
    // An existing service for a agent in dev mode
    let r = client.leader(None).unwrap();

    let (peer, _meta) = r;
    {
        assert_eq!(peer, "127.0.0.1:8300");
    }
}

#[test]
fn status_peers_test() {
    let config = Config::new().unwrap();
    let client = Client::new(config);
    // An existing service for a agent in dev mode
    let r = client.peers(None).unwrap();

    let (peers, _meta) = r;
    {
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0], "127.0.0.1:8300");
    }
}
