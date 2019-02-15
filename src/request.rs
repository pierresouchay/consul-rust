use std::collections::HashMap;
use url::Url;

use std::str;
use std::str::FromStr;
use std::time::Instant;

use reqwest::header::HeaderValue;
use reqwest::Client as HttpClient;
use reqwest::RequestBuilder;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;

use error::{Error, ErrorKind, Result};
use failure::ResultExt;
use {Config, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

pub fn get_vec<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&QueryOptions>,
) -> Result<(Vec<R>, QueryMeta)> {
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or_else(|| config.datacenter.as_ref());

    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }
    if let Some(options) = options {
        if let Some(index) = options.wait_index {
            params.insert(String::from("index"), index.to_string());
        }
        if let Some(wait_time) = options.wait_time {
            params.insert(String::from("wait"), format!("{}s", wait_time.as_secs()));
        }
    }

    let url_str = format!("{}{}", config.address, path);
    let url = Url::parse_with_params(&url_str, params.iter())?;
    let start = Instant::now();
    let mut r = config.http_client.get(url).send()?;
    let j = if r.status() != StatusCode::NOT_FOUND {
        r.json().context(ErrorKind::InvalidJson)?
    } else {
        Vec::new()
    };
    let x: Option<Result<u64>> =
        r.headers()
            .get("X-Consul-Index")
            .and_then(|value: &HeaderValue| {
                Some(
                    str::from_utf8(value.as_bytes())
                        .map_err(|e| Error::from(ErrorKind::Utf8Error(e)))
                        .and_then(|s| {
                            u64::from_str(s).map_err(|e| Error::from(ErrorKind::IntError(e)))
                        }),
                )
            });

    match x {
        Some(r) => Ok((j, Some(r?))),
        None => Ok((j, None)),
    }
    .map(|x: (Vec<R>, Option<u64>)| {
        (
            x.0,
            QueryMeta {
                last_index: x.1,
                request_time: Instant::now() - start,
            },
        )
    })
}

pub fn get<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&QueryOptions>,
) -> Result<(R, QueryMeta)> {
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or_else(|| config.datacenter.as_ref());

    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }
    if let Some(options) = options {
        if let Some(index) = options.wait_index {
            params.insert(String::from("index"), index.to_string());
        }
        if let Some(wait_time) = options.wait_time {
            params.insert(String::from("wait"), format!("{}s", wait_time.as_secs()));
        }
    }

    let url_str = format!("{}{}", config.address, path);
    let url = Url::parse_with_params(&url_str, params.iter())?;
    let start = Instant::now();
    let mut r = config.http_client.get(url).send()?;
    let j = r.json().context(ErrorKind::InvalidJson)?;
    let x: Option<Result<u64>> =
        r.headers()
            .get("X-Consul-Index")
            .and_then(|value: &HeaderValue| {
                Some(
                    str::from_utf8(value.as_bytes())
                        .map_err(|e| Error::from(ErrorKind::Utf8Error(e)))
                        .and_then(|s| {
                            u64::from_str(s).map_err(|e| Error::from(ErrorKind::IntError(e)))
                        }),
                )
            });
    match x {
        Some(r) => Ok((j, Some(r?))),
        None => Ok((j, None)),
    }
    .map(|x: (R, Option<u64>)| {
        (
            x.0,
            QueryMeta {
                last_index: x.1,
                request_time: Instant::now() - start,
            },
        )
    })
}

pub fn delete<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    params: HashMap<String, String>,
    options: Option<&WriteOptions>,
) -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.delete(url) };
    write_with_body(path, None as Option<&()>, config, params, options, req)
}

/*
pub fn post<T: Serialize, R: DeserializeOwned>(path: &str,
                                               body: Option<&T>,
                                               config: &Config,
                                               options: Option<&WriteOptions>)
                                               -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.post(url) };
    write_with_body(path, body, config, options, req)
}
*/
pub fn put<T: Serialize, R: DeserializeOwned>(
    path: &str,
    body: Option<&T>,
    config: &Config,
    params: HashMap<String, String>,
    options: Option<&WriteOptions>,
) -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.put(url) };
    write_with_body(path, body, config, params, options, req)
}

fn write_with_body<T: Serialize, R: DeserializeOwned, F>(
    path: &str,
    body: Option<&T>,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&WriteOptions>,
    req: F,
) -> Result<(R, WriteMeta)>
where
    F: Fn(&HttpClient, Url) -> RequestBuilder,
{
    let start = Instant::now();
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or_else(|| config.datacenter.as_ref());

    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }

    let url_str = format!("{}{}", config.address, path);
    let url = Url::parse_with_params(&url_str, params.iter())?;
    let builder = req(&config.http_client, url);
    let builder = if let Some(b) = body {
        builder.json(b)
    } else {
        builder
    };
    let mut response = builder.send()?;
    Ok((
        response.json().context(ErrorKind::InvalidJson)?,
        WriteMeta {
            request_time: Instant::now() - start,
        },
    ))
}
