use std::collections::HashMap;

use serde_json::{self, Value};
use request::Handler;
use structs::Node;
use error::ConsulResult;
use std::error::Error;

/// Catalog can be used to query the Catalog endpoints
pub struct Catalog{
    handler: Handler
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
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
        Catalog {
            handler: Handler::new(&format!("{}/v1/catalog", address))
        }
    }

    pub fn services(&self) -> ConsulResult<HashMap<String, Vec<String>>> {
        let result = self.handler.get("services")?;
        serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())
    }

    pub fn get_nodes(&self, service: String) -> ConsulResult<Vec<Node>>{
        let uri = format!("service/{}", service);
        let result = self.handler.get(&uri)?;
        let json_data: Value = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;
        let v_nodes: &Vec<Value> = json_data.as_array()
            .ok_or("Cannot get Node array")?;
        let mut filtered: Vec<Node> = Vec::new();
        for node in v_nodes.iter() {
            let node_value = match super::get_string(node, &["Node"]) {
                Some(val) => val,
                None => return Err(format!("consul: Could not find 'Node' in: {:?}", &node))
            };
            let address = match super::get_string(node, &["Address"]) {
                Some(val) => val,
                None => return Err(format!("consul: Could not find 'Address' in: {:?}", &node))
            };
            filtered.push(Node {
               Node: node_value,
               Address: address
            });
        }
        Ok(filtered)
    }


}
