use std::collections::HashMap;

use serde_json;
use request::Handler;
use error::ConsulResult;
use std::error::Error;

/// Catalog can be used to query the Catalog endpoints
pub struct Catalog{
    handler: Handler,
    header: String
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ServiceNode {
    pub Address: String,
    pub Node: String,
    pub ServiceAddress: String,
    pub ServiceID: String,
    pub ServiceName: String,
    pub ServicePort: u16,
    pub ServiceTags: Vec<String>,
}


impl Catalog {

    pub fn new(address: &str, consul_token: &str) -> Catalog {
        Catalog {
            handler: Handler::new(&format!("{}/v1/catalog", address)),
            header: consul_token.to_string()
        }
    }

    pub fn services(&self) -> ConsulResult<HashMap<String, Vec<String>>> {
        let result = self.handler.get("services", Some(self.header.clone()))?;
        serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())
    }

    pub fn get_nodes(&self, service: String) -> ConsulResult<Vec<ServiceNode>>{
        let uri = format!("service/{}", service);
        let result = self.handler.get(&uri, Some(self.header.clone()))?;
        let nodelist: Vec<ServiceNode> = serde_json::from_str(&result).map_err(|e| format!("Error parsing consul response: {:?}\nBody:{:?}", e, &result))?;
        Ok(nodelist)
    }


}
