#![allow(non_snake_case)]

use std::str::from_utf8;
use std::collections::HashMap;
use rustc_serialize::json;

use curl::http;

use structs::Service;

pub struct Agent{
    address: String,
}

impl Agent {

    pub fn new(address: &str) -> Agent {
        Agent{address: address.to_string()}
    }

    pub fn services(&self) -> HashMap<String, Service> {
        let url = format!("{}/v1/agent/services", self.address);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }
}
