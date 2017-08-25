use hyper::{Client, status};
use hyper::net::HttpsConnector;
use hyper::header::{Headers, ContentType};
use hyper::mime::Mime;
use hyper_openssl::OpensslClient;
use std::io::Read;
use openssl::ssl::*;
use error::ConsulResult;

#[derive(Debug)]
pub struct Handler {
    client: Client,
    url: String
}

impl Handler {
    pub fn new(url: &str) -> Handler {
        let client;
        if url.trim().starts_with("https") {
            let mut ssl_connector_builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
            {
                let mut ssl_context_builder = ssl_connector_builder.builder_mut();
                ssl_context_builder.set_verify(SSL_VERIFY_NONE);
            }
            let ssl_connector = ssl_connector_builder.build();
            let mut ssl = OpensslClient::from(ssl_connector);
            ssl.danger_disable_hostname_verification(true);
            let connector = HttpsConnector::new(ssl);
            client = Client::with_connector(connector);
        }
        else {
            client = Client::new();
        }

        Handler {
            client: client,
            url: url.to_owned()
        }
    }

    pub fn get(&self, endpoint: &str) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);
        let mut res = self.client.get(&full_url)
            .send()
            .map_err(|e| format!("{}", e))?;

        if res.status == status::StatusCode::Ok {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;
            Ok(response)
        }
        else {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;

            if !response.is_empty() {
                Ok(response)
            }
            else {
                Err(format!("Request failed with status: {:?}", res.status_raw()))
            }
        }
    }

    pub fn _post(&self, endpoint: &str, req: String) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);

        let mut res = self.client.post(&full_url)
            .body(&req)
            .send()
            .map_err(|e| format!("{}", e))?;

        if res.status == status::StatusCode::Ok {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;
            Ok(response)
        }
        else {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;

            if !response.is_empty() {
                Ok(response)
            }
            else {
                Err(format!("Request failed with status: {:?}", res.status_raw()))
            }
        }
    }

    pub fn put(&self, endpoint: &str, req: String, content_type: Option<&str>) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);

        let mut res;
        if let Some(content) = content_type {
            let mime: Mime = content.parse().unwrap();
            let mut headers = Headers::new();
            headers.set(ContentType(mime));
            res = self.client.put(&full_url)
                .headers(headers)
                .body(&req)
                .send()
                .map_err(|e| format!("{}", e))?;

        }
        else {
            res = self.client.put(&full_url)
                .body(&req)
                .send()
                .map_err(|e| format!("{}", e))?;
        }

        if res.status == status::StatusCode::Ok {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;
            Ok(response)
        }
        else {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;

            if !response.is_empty() {
                Ok(response)
            }
            else {
                Err(format!("Request failed with status: {:?}", res.status_raw()))
            }
        }

    }

    pub fn delete(&self, endpoint: &str) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);
        let mut res = self.client.delete(&full_url)
            .send()
            .map_err(|e| format!("{}", e))?;

        if res.status == status::StatusCode::Ok {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;
            Ok(response)
        }
        else {
            let mut response = String::new();
            res.read_to_string(&mut response)
                .map_err(|e| format!("{}", e))?;

            if !response.is_empty() {
                Ok(response)
            }
            else {
                Err(format!("Request failed with status: {:?}", res.status_raw()))
            }
        }
    }


}
