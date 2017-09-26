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
    pub fn new<'a, S>(address: S) -> Client where S: Into<Cow<'a, str>> {
        let cow = address.into();
        let addr = cow.borrow();
        let agent = Agent::new(addr);
        let catalog = Catalog::new(addr);
        let health = Health::new(addr);
        let keystore = Keystore::new(addr);
        let session = Session::new(addr);
        Client {
            agent: agent,
            catalog: catalog,
            health: health,
            session: session,
            keystore: keystore
        }
    }
}
