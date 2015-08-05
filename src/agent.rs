#![allow(non_snake_case)]

use std::str::from_utf8;
use std::collections::HashMap;

use rustc_serialize::json;
use curl::{http, ErrCode};

use super::Service;

/// Agent can be used to query the Agent endpoints
pub struct Agent{
    endpoint: String,
}

/// AgentMember represents a cluster member known to the agent
#[derive(RustcDecodable, RustcEncodable)]
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

/// AgentServiceRegistration is used to register a new service
#[derive(RustcDecodable, RustcEncodable)]
pub struct AgentServiceRegistration{
    ID: String,
    Name: String,
    Tags: Vec<String>,
    Port: u16,
    Address: String
}

impl Agent {

    pub fn new(address: &str) -> Agent {
        Agent{endpoint: format!("{}/v1/agent", address)}
    }

    pub fn services(&self) -> HashMap<String, Service> {
        let url = format!("{}/services", self.endpoint);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

    pub fn members(&self) -> Vec<AgentMember> {
        let url = format!("{}/members", self.endpoint);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

    pub fn service_register(&self, service: AgentServiceRegistration) -> Result<(), ErrCode> {
        let url = format!("{}/service/register", self.endpoint);
        let service_string :String = json::encode(&service).unwrap();
        let result = match http::handle().put(url, &service_string[..]).exec() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
        result

    }
}
