#![allow(non_snake_case)]

#[deriving(Decodable,Encodable,Show)]
pub struct Node {
    Node: String,
    Address: String,
}

#[deriving(Decodable,Encodable,Show)]
pub struct Service {
    ID: String,
    pub Service: String,
    pub Tags: Vec<String>,
    pub Port: int,
}
