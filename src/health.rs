use request::Handler;
use serde_json::{self, Value};
use ::find_path;
use error::ConsulResult;
use std::error::Error;

use super::HealthService;

/// Health can be used to query the Health endpoints
pub struct Health{
    handler: Handler
}


impl Health {

    pub fn new(address: &str) -> Health {
        Health {
            handler: Handler::new(&format!("{}/v1/health", address))
        }
    }

    fn request(&self, uri: &str) -> ConsulResult<Vec<HealthService>> {
        let result = self.handler.get(uri)?;
        serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())
    }

    pub fn service(&self, name: &str, o_tag: Option<&str>) -> ConsulResult<Vec<HealthService>> {
        let uri = match o_tag {
            Some(value) => format!("service/{}?tag={}", name, value),
            None => format!("service/{}", name)
        };
        self.request(&uri)
    }

    pub fn healthy_nodes_by_service(&self, service_id: &str) -> ConsulResult<Vec<String>> {
        let uri = format!("service/{}", service_id);
        let result = self.handler.get(&uri)?;
        let json_data: Value = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;
        let v_nodes = json_data.as_array()
            .ok_or("Cannot get Node array")?;
        let mut filtered: Vec<String> = Vec::new();
        for node in v_nodes.iter() {
            let ip = match super::get_string(node, &["Node", "Address"]) {
                Some(val) => val,
                None => continue
            };
            let checks = match find_path(node, &["Checks"]) {
                Some(val) => val.as_array().ok_or("Cannot get Checks array")?,
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
        let uri = format!("checks/{}", service_id);
        let result = self.handler.get(&uri)?;
        let json_data: Value = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;
        let v_nodes = json_data.as_array()
            .ok_or("Cannot get Node array")?;
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
