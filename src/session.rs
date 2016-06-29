#![allow(non_snake_case)]

use std::str::from_utf8;
use std::thread;
use std::time::Duration;

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
    
    pub fn create(&self, name: String) -> Option<String> {
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
            panic!("Consul: Error creating a session! Response: {}", resp);
        }
        let result = from_utf8(resp.get_body()).unwrap();        
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(err) => panic!("consul: Could not convert to json: {:?}. Err: {}", result, err)
        };
        super::get_string(&json_data, &["ID"])
    }
    
    pub fn renew(&self, session_id: &String) -> bool {
        for _ in 0..10 {
            let url = format!("{}/renew/{}", self.endpoint, session_id);
            let resp = http::handle()
                .put(url, "")
                .content_type("application/json")
                .exec().unwrap();
            if resp.get_code() != 200 {
                if resp.get_code() == 404 {
                    println!("Could not renew session: {}, returned HTTP code: {:?}. Returning false.", session_id, resp.get_code());
                    return false;
                }
                else {
                    println!("Could not renew session: {}, returned HTTP code: {:?}. Sleeping for 2 seconds", session_id, resp.get_code());
                    thread::sleep(Duration::from_millis(2000u64));
                }
            }
            else {
                return true;
            }
        }
        panic!("Could not renew session: {} after 10 tries.", session_id);
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
