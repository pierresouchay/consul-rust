use std::str::from_utf8;
use std::collections::HashMap;

use rustc_serialize::json;
use curl::http;
use structs::Node;

/// Catalog can be used to query the Catalog endpoints
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
    
    pub fn get_nodes(&self, service: String) -> Vec<Node>{
        let url = format!("{}/service/{}", self.endpoint, service);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(_) => panic!("consul: Could not convert to json: {:?}", result)
        };
        let v_nodes = json_data.as_array().unwrap();
        let mut filtered: Vec<Node> = Vec::new();
        for node in v_nodes.iter() {
            let node_value = super::get_string(node, &["Node"]);
            let address = super::get_string(node, &["Address"]);
            filtered.push(Node {
               Node: node_value,
               Address: address
            });
        }
        filtered
    }
    

}
