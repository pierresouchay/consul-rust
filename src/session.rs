#![allow(non_snake_case)]

use std::str::from_utf8;
use std::collections::HashMap;
use std::thread;

use rustc_serialize::json;
use curl::http;

pub const SESSION_TTL: &'static str = "15s";

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct SessionCreate {
    Name: String,
    TTL: String
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct SessionID {
    ID: String
}

pub struct Session {
    endpoint: String
}

impl Session {
    pub fn new(address: &str) ->  Session {
        Session {
            endpoint: format!("http://{}/v1/session", address)
        }
    }
    
    pub fn create(&self, name: String) -> String{
        let url = format!("{}/create", self.endpoint);
        let session = SessionCreate {
            Name: name,
            TTL: self::SESSION_TTL.to_owned()
        };
        let json_str = json::encode(&session).unwrap();
        
        let resp = http::handle()
            .put(url, &json_str)
            .content_type("application/json")
            .exec().unwrap();
        if resp.get_code() != 200 {
            panic!("Consul: Error creating a session!");
        }
        let result = from_utf8(resp.get_body()).unwrap();        
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(err) => panic!("consul: Could not convert to json: {:?}", result)
        };
        super::get_string(&json_data, &["ID"])
    }
    
    pub fn renew(&self, session_id: &String) -> bool {
        for i in 0..10 {
            let url = format!("{}/renew/{}", self.endpoint, session_id);
            let resp = http::handle()
                .put(url, "")
                .content_type("application/json")
                .exec().unwrap();
            if resp.get_code() != 200 {
                println!("Could not renew ession: {}, returned HTTP code: {:?}. Sleeping for 1 sec", session_id, resp.get_code());
                thread::sleep_ms(1000);
            }
            else {
                return true;
            }
        }
        false
    }

    pub fn end(&self, session_id: &String) {
        let url = format!("{}/destroy/{}", self.endpoint, session_id);
        let resp = http::handle()
            .put(url, "")
            .content_type("application/json")
            .exec().unwrap();
        if resp.get_code() != 200 {
            panic!("Cound not destroy session: {}", session_id);
        }
        
    }
    
    
}
