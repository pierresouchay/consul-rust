use  super::{Agent, Catalog, Health};

/// provides a client to the Consul API
pub struct Client{
    /// agent endpoint
    pub agent: Agent,
    /// catalog endpoint
    pub catalog: Catalog,
    /// health endpoint
    pub health: Health,
}

impl Client {
    /// Constructs a consul client
    pub fn new(address: &str) -> Client {
        let agent = Agent::new(address);
        let catalog = Catalog::new(address);
        let health = Health::new(address);
        Client{agent:agent, catalog: catalog, health: health}
    }
}
