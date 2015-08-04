#![allow(non_snake_case)]

use std::str::from_utf8;
use std::collections::HashMap;

use rustc_serialize::json;
use curl::http;

use super::{Service, RegisterService};

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

    pub fn register(&self, service: RegisterService) {
        let url = format!("{}/service/register", self.endpoint);
        let json_str = json::encode(&service).unwrap();
        println!("About to send: {:?}", json_str);
        
        let resp = http::handle()
            .put(url, &json_str)
            .content_type("application/json")
            .exec().unwrap();
        println!("Resp Code: {:?}, body: {:?}", resp.get_code(), resp.get_body());
//         let result = from_utf8(resp.get_body()).unwrap();
//         json::decode(result).unwrap()
    }
    
}
