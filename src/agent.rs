#![allow(non_snake_case)]

use std::str::from_utf8;
use std::collections::HashMap;
use serialize::json;

use curl::http;

#[deriving(Decodable, Show)]
pub struct Service {
    ID: String,
    Service: String,
    Tags: Vec<String>,
    Port: int,
}

pub struct Agent{
    address: &'static str,
}

impl Agent {

    pub fn new(address: &'static str) -> Agent {
        Agent{address: address}
    }

    pub fn services(&self) -> HashMap<String, Service> {
        let url = self.address.to_string() + "/agent/services";
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }
}
