#![allow(non_snake_case)]

use serde_json;
use std::thread;
use std::time::Duration;

use request::Handler;
use error::ConsulResult;
use std::error::Error;

pub const SESSION_TTL: &'static str = "15s";

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionCreate {
    Name: String,
    TTL: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionID {
    ID: String
}

pub struct Session {
    handler: Handler,
    header: String
}

impl Session {
    pub fn new(address: &str, consul_token: &str) ->  Session {
        Session {
            handler: Handler::new(&format!("{}/v1/session", address)),
            header: consul_token.to_string()
        }
    }

    pub fn create(&self, name: String) -> ConsulResult<Option<String>> {
        let session = SessionCreate {
            Name: name,
            TTL: self::SESSION_TTL.to_owned()
        };
        let json_str = serde_json::to_string(&session)
            .map_err(|e| e.description().to_owned())?;

        let result = self.handler.put("create", json_str, Some("application/json"), Some(self.header.clone()))?;

        let json_data = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;
        Ok(super::get_string(&json_data, &["ID"]))
    }

    pub fn renew(&self, session_id: &String) -> ConsulResult<bool> {
        for _ in 0..10 {
            let uri = format!("renew/{}", session_id);
            match self.handler.put(&uri, "".to_owned(), Some("application/json"), Some(self.header.clone())) {
                Ok(_) => return Ok(true),
                Err(e) => {
                    println!("Could not renew session: {}, returned error: {}. Sleeping for 2 seconds", session_id, e);
                    thread::sleep(Duration::from_millis(2000u64));
                }
            };
        }
        Err(format!("Could not renew session: {} after 10 tries.", session_id))
    }

    pub fn end(&self, session_id: &String) -> ConsulResult<()> {
        let uri = format!("destroy/{}", session_id);
        self.handler.put(&uri, "".to_owned(), Some("application/json"), Some(self.header.clone()))?;
        Ok(())
    }


}
