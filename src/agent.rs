#![allow(non_snake_case)]

use std::str::from_utf8;
use std::collections::HashMap;

use rustc_serialize::json;
use curl::http;

use super::{Service, RegisterService, TtlHealthCheck};

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
        let resp = http::handle()
            .put(url, &json_str)
            .content_type("application/json")
            .exec().unwrap();
        if resp.get_code() != 200 {
            panic!("Consul: Error registering a service!");
        }
    }
    
    pub fn register_ttl_check(&self, health_check: TtlHealthCheck) {
        let url = format!("{}/check/register", self.endpoint);
        let json_str = json::encode(&health_check).unwrap();
        let resp = http::handle()
            .put(url, &json_str)
            .content_type("application/json")
            .exec().unwrap();
        if resp.get_code() != 200 {
            panic!("Consul: Error registering a health check!");
        }
    }
    
    pub fn check_pass(&self, service_id: String) {
        let url = format!("{}/check/pass/{}", self.endpoint, service_id);
        let _resp = http::handle().get(url).exec().unwrap();
    }

    pub fn get_self_name(&self) -> String {
        let url = format!("{}/self", self.endpoint);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(_) => panic!("consul: Could not convert to json: {:?}", result)
        };
        super::get_string(&json_data, &["Config", "NodeName"])
    }

    pub fn get_self_address(&self) -> String {
        let url = format!("{}/self", self.endpoint);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(_) => panic!("consul: Could not convert to json: {:?}", result)
        };
        super::get_string(&json_data, &["Config", "AdvertiseAddr"])
    }
}
