#![allow(non_snake_case)]

/// Node represents a node
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Node {
    pub Node: String,
    pub Address: String,
}

/// Service represents a service
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Service {
    pub ID: String,
    pub Service: String,
    pub Tags: Vec<String>,
    pub Port: u32,
}

/// HealthService is used for the health service
#[derive(RustcDecodable, RustcEncodable)]
pub struct HealthService{
    pub Node: Node,
    pub Service: Service,
}

/// Service represents a service
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct RegisterService {
    pub ID: String,
    pub Name: String,
    pub Tags: Vec<String>,
    pub Port: u32,
}