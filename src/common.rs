/// A service's weights, comonly used in response payloads from Consul.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ServiceWeights {
    passing: u32,
    warning: u32,
}

/// A tagged address, commonly used in response payloads from Consul.
#[derive(Deserialize, Debug)]
pub struct TaggedAddress {
    /// The tagged address.
    pub address: String,
    /// The port included with this address.
    pub port: u16,
}
