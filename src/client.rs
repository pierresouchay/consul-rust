use std::borrow::{Borrow, Cow};
use super::{Agent, Catalog, Health, Keystore, Session};

/// provides a client to the Consul API
pub struct Client{
    /// agent endpoint
    pub agent: Agent,
    /// catalog endpoint
    pub catalog: Catalog,
    /// health endpoint
    pub health: Health,
    pub keystore: Keystore,
    pub session: Session
}

impl Client {
    /// Constructs a consul client
    pub fn new<'a, S>(address: S, consul_token: S) -> Client where S: Into<Cow<'a, str>> {
        let cow = address.into();
        let addr = cow.borrow();
        let header_cow = consul_token.into();
        let header_token = header_cow.borrow();
        let agent = Agent::new(addr, header_token);
        let catalog = Catalog::new(addr, header_token);
        let health = Health::new(addr, header_token);
        let keystore = Keystore::new(addr, header_token);
        let session = Session::new(addr, header_token);
        Client {
            agent: agent,
            catalog: catalog,
            health: health,
            session: session,
            keystore: keystore
        }
    }
}
