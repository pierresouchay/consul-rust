#![allow(non_snake_case)]

use std::collections::HashMap;
use request::Handler;

use serde_json;
use error::ConsulResult;
use std::error::Error;

use super::{Service, RegisterService, TtlHealthCheck};

/// Agent can be used to query the Agent endpoints
pub struct Agent{
    handler: Handler,
    header: String
}

/// AgentMember represents a cluster member known to the agent
#[derive(Serialize, Deserialize)]
pub struct AgentMember {
	Name: String,
	Addr: String,
	Port: u16,
	Tags: HashMap<String, String>,
	Status: usize,
	ProtocolMin: u8,
	ProtocolMax: u8,
	ProtocolCur: u8,
	DelegateMin: u8,
	DelegateMax: u8,
	DelegateCur: u8
}


impl Agent {
    pub fn new(address: &str, consul_token: &str) -> Agent {
        Agent {
            handler: Handler::new(&format!("{}/v1/agent", address)),
            header: consul_token.to_string()
        }
    }

    pub fn services(&self) -> ConsulResult<HashMap<String, Service>> {
        let result = self.handler.get("services", Some(self.header.clone()))?;
        serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())
    }

    pub fn members(&self) -> ConsulResult<Vec<AgentMember>> {
        let result = self.handler.get("members", Some(self.header.clone()))?;
        serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())
    }

    pub fn register(&self, service: RegisterService) -> ConsulResult<()> {
        let json_str = serde_json::to_string(&service)
            .map_err(|e| e.description().to_owned())?;

        if let Err(e) = self.handler.put("service/register", json_str, Some("application/json"), Some(self.header.clone())) {
            Err(format!("Consul: Error registering a service. Err:{}", e))
        }
        else {
            Ok(())
        }
    }

    pub fn register_ttl_check(&self, health_check: TtlHealthCheck) -> ConsulResult<()> {
        let json_str = serde_json::to_string(&health_check)
            .map_err(|e| e.description().to_owned())?;

        if let Err(e) = self.handler.put("check/register", json_str, Some("application/json"), Some(self.header.clone())) {
            Err(format!("Consul: Error registering a health check. Err:{}", e))
        }
        else {
            Ok(())
        }
    }

    pub fn check_pass(&self, service_id: String) -> ConsulResult<()> {
        let uri = format!("check/pass/{}", service_id);
        self.handler.get(&uri, Some(self.header.clone()))?;
        Ok(())
    }

    pub fn get_self_name(&self) -> ConsulResult<Option<String>> {
        let result = self.handler.get("self", Some(self.header.clone()))?;
        let json_data = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;
        Ok(super::get_string(&json_data, &["Config", "NodeName"]))
    }

    pub fn get_self_address(&self) -> ConsulResult<Option<String>> {
        let result = self.handler.get("self", Some(self.header.clone()))?;
        let json_data = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;
        Ok(super::get_string(&json_data, &["Config", "AdvertiseAddr"]))
    }
}
