#![allow(non_snake_case)]

use std::str::from_utf8;
use std::collections::HashMap;

use rustc_serialize::json;
use curl::http;

pub struct Keystore{
    endpoint: String
}

impl Keystore {
    pub fn new(address: &str) ->  Keystore {
        Keystore {
            endpoint: format!("{}/v1/kv", address)
        }
    }
    
    pub fn set_key(&self, key: String, value: String) {
        let url = format!("{}/{}", self.endpoint, key);
        let resp = http::handle()
            .put(url, &value)
            .content_type("application/json")
            .exec().unwrap();
        if resp.get_code() != 200 {
            panic!("Consul: Error setting a key!");
        }
    }
        
    pub fn get_key(&self) -> String {
        let url = format!("{}/{}", self.endpoint);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(err) => panic!("consul: Could not convert to json: {:?}", result)
        };
        super::get_string(&json_data, &["Value"])
    }

}