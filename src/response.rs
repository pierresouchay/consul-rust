use reqwest::{self, StatusCode};
use serde::de::DeserializeOwned;

pub trait ResponseHelper {
    fn parse(&mut self) -> crate::Result<()>;
    fn parse_json<R>(&mut self) -> crate::Result<R>
    where
        R: DeserializeOwned;
}

impl ResponseHelper for reqwest::Response {
    fn parse(&mut self) -> crate::Result<()> {
        if self.status() != StatusCode::OK {
            return Err(crate::error::unexpected_response(
                self.text().unwrap_or(String::from("")),
            ));
        }
        Ok(())
    }

    fn parse_json<R>(&mut self) -> crate::Result<R>
    where
        R: DeserializeOwned,
    {
        if self.status() != StatusCode::OK {
            return Err(crate::error::unexpected_response(
                self.text().unwrap_or(String::from("")),
            ));
        }
        self.json().map_err(crate::error::invalid_response)
    }
}
