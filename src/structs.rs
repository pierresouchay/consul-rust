#![allow(non_snake_case)]

/// Node represents a node
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub Node: String,
    pub Address: String,
}

/// Service represents a service
#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub ID: String,
    pub Service: String,
    pub Tags: Option<Vec<String>>,
    pub Port: u32,
    pub Address: String
}

/// HealthService is used for the health service
#[derive(Serialize, Deserialize)]
pub struct HealthService {
    pub Node: Node,
    pub Service: Service,
}

/// Service represents a service
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterService {
    pub ID: String,
    pub Name: String,
    pub Tags: Vec<String>,
    pub Port: u16,
    pub Address: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TtlHealthCheck {
    pub ServiceID: String,
    pub ID: String,
    pub Name: String,
    pub Notes: String,
    pub TTL: String
}
