#![allow(non_snake_case)]

#[deriving(Decodable, Show)]
pub struct Node {
    Node: String,
    Address: String,
}

#[deriving(Decodable, Show)]
pub struct Service {
    ID: String,
    pub Service: String,
    pub Tags: Vec<String>,
    pub Port: int,
}
