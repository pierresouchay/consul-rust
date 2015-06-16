#![allow(non_snake_case)]
use std::str::from_utf8;

use rustc_serialize::json;
use curl::http;

use structs::{Node, Service};


#[derive(RustcDecodable, RustcEncodable)]
pub struct HealthService{
    pub Node: Node,
    pub Service: Service,
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
                 format!("{}/v1/health/service/{}", self.address, name)
             } else {
                 format!("{}/v1/health/service/{}?tag={}", self.address, name, tags)
             };
             
        self.request(&url)
   }
}
