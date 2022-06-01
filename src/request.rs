use std::{collections::HashMap, future::Future, str};

use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::{payload::QueryOptions, Client, ConsulError, ConsulResult};

#[async_trait]
trait AndThenAsync<T: Send, E: Send> {
    async fn and_then_async<U, F, Fut>(self, f: F) -> std::result::Result<U, E>
    where
        F: FnOnce(T) -> Fut + Send,
        Fut: Future<Output = Result<U, E>> + Send,
        Self: Sized;
}

#[async_trait]
impl<T: Send, E: Send> AndThenAsync<T, E> for std::result::Result<T, E> {
    async fn and_then_async<U, F, Fut>(self, f: F) -> std::result::Result<U, E>
    where
        F: FnOnce(T) -> Fut + Send,
        Fut: Future<Output = Result<U, E>> + Send,
        Self: Sized,
    {
        match self {
            Ok(inner) => Ok(f(inner).await?),
            Err(e) => Err(e),
        }
    }
}

impl Client {
    pub(crate) async fn send_with_empty<
        Path: AsRef<str>,
        Body: Serialize,
        Response: DeserializeOwned,
    >(
        &self,
        method: Method,
        path: Path,
        params: Option<HashMap<String, String>>,
        body: Option<Body>,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Option<Response>> {
        // unwrap parameters
        let mut params = params.unwrap_or_default();
        // if datacenter option is specified, set
        let datacenter: Option<String> = options
            .and_then(|o| o.datacenter)
            .or_else(|| self.config.datacenter.as_ref().map(|s| s.clone()));
        if let Some(dc) = datacenter {
            params.insert(String::from("dc"), dc.to_owned());
        }
        // parse url and create builder
        let url = Url::parse_with_params(
            &format!("{}{}", self.config.address, path.as_ref()),
            params.iter(),
        )
        .unwrap();
        let builder = self.config.http_client.request(method, url);
        // add body if specified
        let builder = if let Some(b) = body { builder.json(&b) } else { builder };
        // add query options
        let builder = match &self.config.token {
            Some(val) => builder.header("X-Consul-Token", val),
            None => builder,
        };
        // send request
        let response = builder.send().await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !response.status().is_success() {
            return Err(ConsulError::RequestFailed(response.status()));
        }
        let response = response.text().await?;
        if response.is_empty() {
            return Ok(None);
        }
        let response = serde_json::from_str(&response).map_err(|e| ConsulError::DecodeError(e))?;
        Ok(response)
    }

    /// This method sends a request to the Consul API.
    ///
    /// The request is sent to the Consul API at the given path using the
    /// provided method. If params exists, the request will be sent with the
    /// given parameters, plus any defined in the client options.
    ///
    /// This method will error if the request fails, and will panic if the
    /// URL or parameters are invalid.
    pub(crate) async fn send<Path: AsRef<str>, Body: Serialize, Response: DeserializeOwned>(
        &self,
        method: Method,
        path: Path,
        params: Option<HashMap<String, String>>,
        body: Option<Body>,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Response> {
        // unwrap parameters
        let mut params = params.unwrap_or_default();
        // if datacenter option is specified, set
        let datacenter: Option<String> = options
            .and_then(|o| o.datacenter)
            .or_else(|| self.config.datacenter.as_ref().map(|s| s.clone()));
        if let Some(dc) = datacenter {
            params.insert(String::from("dc"), dc.to_owned());
        }
        // parse url and create builder
        let url = Url::parse_with_params(
            &format!("{}{}", self.config.address, path.as_ref()),
            params.iter(),
        )
        .unwrap();
        let builder = self.config.http_client.request(method, url);
        // add body if specified
        let builder = if let Some(b) = body { builder.json(&b) } else { builder };
        // add query options
        let builder = match &self.config.token {
            Some(val) => builder.header("X-Consul-Token", val),
            None => builder,
        };
        builder
            .send()
            .await
            .and_then_async(|x| async { x.json::<Response>().await })
            .await
            .map_err(|err| ConsulError::HttpError(err))
    }
    /// This method makes a GET request with query parameters to the given path.
    pub(crate) async fn get_with_params<Path: AsRef<str>, T: DeserializeOwned>(
        &self,
        path: Path,
        params: Option<HashMap<String, String>>,
        options: Option<QueryOptions>,
    ) -> ConsulResult<T> {
        self.send::<Path, (), T>(Method::GET, path, params, None, options).await
    }

    /// This method makes a GET request to the given path.
    pub(crate) async fn get<Path: AsRef<str>, T: DeserializeOwned>(
        &self,
        path: Path,
        options: Option<QueryOptions>,
    ) -> ConsulResult<T> {
        self.get_with_params(path, None, options).await
    }

    /// This method makes a PUT request to the given path.
    pub(crate) async fn put<Path: AsRef<str>, Body: Serialize, Response: DeserializeOwned>(
        &self,
        path: Path,
        body: Body,
        params: Option<HashMap<String, String>>,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Response> {
        self.send::<Path, Body, Response>(Method::PUT, path, params, Some(body), options).await
    }

    /// This method makes a DELETE request to the given path.
    pub(crate) async fn delete<Path: AsRef<str>, Response: DeserializeOwned>(
        &self,
        path: Path,
        params: Option<HashMap<String, String>>,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Response> {
        self.send::<Path, (), Response>(Method::DELETE, path, params, None, options).await
    }
}
