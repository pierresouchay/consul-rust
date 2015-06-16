use std::str::from_utf8;
use std::collections::HashMap;
use rustc_serialize::json;

use curl::http;

pub struct Catalog{
    endpoint: String,
}

impl Catalog {

    pub fn new(address: &str) -> Catalog {
        Catalog{endpoint: format!("{}/v1/catalog", address)}
    }

    pub fn services(&self) -> HashMap<String, Vec<String>> {
        let url = format!("{}/services", self.endpoint);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

}
