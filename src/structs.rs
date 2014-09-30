#![allow(non_snake_case)]

#[deriving(Decodable, Show)]
pub struct Node {
    Node: String,
    Address: String,
}

#[deriving(Decodable, Show)]
pub struct Service {
    ID: String,
    Service: String,
    Tags: Vec<String>,
    Port: int,
}
