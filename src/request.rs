use std::collections::HashMap;
use std::convert::Into;
use url::Url;

use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
pub use reqwest::{Method, StatusCode};
use serde::Serialize;
use serde_json;

use error::{Error, Result};
use Client;

const API_VERSION: &str = "v1";

// This is a bespoke version of reqwest's RequestBuilder
pub struct Request {
    client: reqwest::Client,
    request: Option<reqwest::Request>,
    err: Option<Error>, // Will only report the first error...
}

impl Request {
    pub fn new(client: &Client, method: Method, path: &str) -> Request {
        Request::new_with_params(client, method, path, HashMap::new())
    }

    pub fn new_with_params(
        client: &Client,
        method: Method,
        path: &str,
        params: HashMap<String, String>,
    ) -> Request {
        let api = format!("{}/{}/{}", client.config.address, API_VERSION, path);
        let url = match Url::parse_with_params(&api, params.iter()) {
            Ok(url) => url,
            Err(err) => {
                return Request {
                    client: client.config.http_client.clone(),
                    request: None,
                    err: Some(crate::error::request(err)),
                };
            }
        };
        let mut request = reqwest::Request::new(method, url);
        if let Some(token) = &client.config.token {
            if let Ok(token) = HeaderValue::from_str(token.as_str()) {
                request.headers_mut().insert("X-Consul-Token", token);
            }
        }
        Request {
            client: client.config.http_client.clone(),
            request: Some(request),
            err: None,
        }
    }

    pub fn body<T: Into<reqwest::Body>>(&mut self, body: T) -> &mut Request {
        if self.err.is_none() {
            if let Some(req) = self.request.as_mut() {
                *req.body_mut() = Some(body.into());
            }
        }
        self
    }

    pub fn json<S: Serialize>(&mut self, json: &S) -> &mut Request {
        if self.err.is_none() {
            if let Some(req) = self.request.as_mut() {
                match serde_json::to_vec(json) {
                    Ok(body) => {
                        req.headers_mut()
                            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                        *req.body_mut() = Some(body.into());
                    }
                    Err(err) => self.err = Some(crate::error::request(err)),
                }
            }
        }
        self
    }

    pub fn send(&mut self) -> Result<reqwest::Response> {
        if let Some(err) = self.err.take() {
            return Err(err);
        }
        self.client
            .execute(self.request.take().expect("Request cannot be reused"))
            .map_err(crate::error::request)
    }
}
