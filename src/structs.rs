#![allow(non_snake_case)]

#[derive(RustcDecodable, RustcEncodable)]
pub struct Node {
    pub Node: String,
    pub Address: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Service {
    ID: String,
    pub Service: String,
    pub Tags: Vec<String>,
    pub Port: u32,
}
