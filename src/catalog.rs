use std::str::from_utf8;
use std::collections::HashMap;
use rustc_serialize::json;

use curl::http;

pub struct Catalog{
    address: String,
}

impl Catalog {

    pub fn new(address: &str) -> Catalog {
        Catalog{address: address.to_string()}
    }

    pub fn services(&self) -> HashMap<String, Vec<String>> {
        let url = format!("{}/v1/catalog/services", self.address);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

}
