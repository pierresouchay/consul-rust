use std::str::from_utf8;

use curl::http;
use rustc_serialize::json;

use super::HealthService;

/// Health can be used to query the Health endpoints
pub struct Health{
    endpoint: String,
}


impl Health {

    pub fn new(address: &str) -> Health {
        Health{endpoint: format!("{}/v1/health", address)}
    }

    fn request(&self, url: &str) -> Vec<HealthService> {
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

   // Rust does not support default parameters or optional parameters for now, so `tag` must be provided
   pub fn service(&self, name: &str, tag: &str) -> Vec<HealthService>{
       let url =
             if tag == "" {
                 format!("{}/service/{}", self.endpoint, name)
             } else {
                 format!("{}/service/{}?tag={}", self.endpoint, name, tag)
             };

        self.request(&url)
   }
}
