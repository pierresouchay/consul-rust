use std::str::from_utf8;

use curl::http;
use rustc_serialize::json;

use super::{HealthService, Node};

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
    pub fn service(&self, name: &str, o_tag: Option<&str>) -> Vec<HealthService>{
        let url = match o_tag {
            Some(value) => format!("{}/service/{}?tag={}", self.endpoint, name, value),
            None => format!("{}/service/{}", self.endpoint, name)
        };
        self.request(&url)
    }

    pub fn healthy_nodes_by_service(&self, service_id: &str) -> Result<Vec<String>, String> {
        let url = format!("{}/service/{}", self.endpoint, service_id);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(err) => return Err(format!("consul: Could not convert to json: {:?}", result))
        };
        let v_nodes = json_data.as_array().unwrap();
        let mut filtered: Vec<String> = Vec::new();
        for node in v_nodes.iter() {
            let ip = match super::get_string(node, &["Node", "Address"]) {
                Some(val) => val,
                None => continue
            };
            let checks = match node.find_path(&["Checks"]) {
                Some(val) => val.as_array().unwrap(),
                None => continue
            };
            let mut healthy = true;
            for check in checks {
                let status = match super::get_string(check, &["Status"]) {
                    Some(val) => val,
                    None => {
                        healthy = false;
                        break;
                    }
                };
                if !healthy {
                    break;
                }
                if status != "passing" {
                    healthy = false;
                    break;
                }
            }
            if healthy {
                filtered.push(ip.to_owned());
            }
        }
        Ok(filtered)
    }
    
    pub fn get_healthy_nodes(&self, service_id: &str) -> Result<Vec<String>, String> {
        let url = format!("{}/checks/{}", self.endpoint, service_id);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(err) => return Err(format!("consul: Could not convert to json: {:?}", result))
        };
        let v_nodes = json_data.as_array().unwrap();
        let mut filtered: Vec<String> = Vec::new();
        for node in v_nodes.iter() {
            if let Some(status) = super::get_string(node, &["Status"]) {
                if status == "passing" {
                    if let Some(node_value) = super::get_string(node, &["Node"]) {
                        filtered.push(node_value);
                    }
                }
            }
        }
        Ok(filtered)
    }
}
