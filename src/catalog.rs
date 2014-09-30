use std::str::from_utf8;
use std::collections::HashMap;
use serialize::json;

use curl::http;

pub struct Catalog{
    address: &'static str,
}

impl Catalog {

    pub fn new(address: &'static str) -> Catalog {
        Catalog{address: address}
    }

    pub fn services(&self) -> HashMap<String, Vec<String>> {
        let url = self.address.to_string() + "/catalog/services";
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

}
