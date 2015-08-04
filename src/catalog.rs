use std::str::from_utf8;
use std::collections::HashMap;

use rustc_serialize::json;
use curl::http;
use structs::Node;

/// Catalog can be used to query the Catalog endpoints
pub struct Catalog{
    endpoint: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ServiceNode {
    Address: String,
    Node: String,
    ServiceAddress: String,
    ServiceID: String,
    ServiceName: String,
    ServicePort: u16,
    ServiceTags: Vec<String>,
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
        let nodes: Vec<ServiceNode> = json::decode(result).unwrap();
        let mut filtered: Vec<Node> = Vec::with_capacity(nodes.len());
        for node in nodes {
            filtered.push(Node {
                Node: node.Node,
                Address: node.Address
            });
        }
        filtered
    }

}
