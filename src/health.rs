#![allow(non_snake_case)]
use serialize::json;
use std::str::from_utf8;

use curl::http;
use structs::{Node, Service};


#[deriving(Decodable, Encodable, Show)]
pub struct HealthService{
    Node: Node,
    Service: Service,
}


pub struct Health{
    address: String,
}


impl Health {

    pub fn new(address: &str) -> Health {
        Health{address: address.to_string()}
    }

    fn request(&self, url: &str) -> Vec<HealthService> {
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

   // Rust does not support default parameters or optional parameters for now, so `tags` must be provided
   pub fn service(&self, name: &str, tags: &str) -> Vec<HealthService>{
       let url = 
             if tags == "" {
                self.address + "/health/service/" + name
             } else {
                self.address + "/health/service/" + name + "?tag=" + tags
             };
             
        self.request(url.as_slice())
   }
}
