use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use std::io::Read;
use error::ConsulResult;
use std::string::String;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Handler {
    client: Client,
    url: String
}

impl Handler {
    pub fn new(url: &str) -> Handler {
        Handler {
            client: Client::new(),
            url: url.to_owned()
        }
    }

    pub fn get(&self, endpoint: &str) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);
        self.client.get(&full_url).send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())
    }

    pub fn _post(&self, endpoint: &str, req: String) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);
        self.client.post(&full_url)
            .body(req)
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())
    }

    pub fn put(&self, endpoint: &str, req: String, content_type: Option<&str>) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);
        if let Some(content) = content_type {
            self.client.put(&full_url)
                .body(req)
                .header(CONTENT_TYPE, content)
                .send()
                .map_err(|e| e.to_string())?
                .text()
                .map_err(|e| e.to_string())
        }
        else {
            self.client.put(&full_url)
                .body(req)
                .send()
                .map_err(|e| e.to_string())?
                .text()
                .map_err(|e| e.to_string())
        }
    }

    pub fn delete(&self, endpoint: &str) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);
        self.client.delete(&full_url)
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())
    }


}
