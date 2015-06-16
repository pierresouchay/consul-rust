use  agent::Agent;
use  catalog::Catalog;
use  health::Health;

pub struct Client{
    pub agent: Agent,
    pub catalog: Catalog,
    pub health: Health,
}

impl Client {
    pub fn new(address: &str) -> Client {
        let agent = Agent::new(address);
        let catalog = Catalog::new(address);
        let health = Health::new(address);
        Client{agent:agent, catalog: catalog, health: health}
    }
}
