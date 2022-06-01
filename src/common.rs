#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ServiceWeights {
    passing: u32,
    warning: u32,
}

#[derive(Deserialize, Debug)]
pub struct TaggedAddress {
    pub address: String,
    pub port: u16,
}
