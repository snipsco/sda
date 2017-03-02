use sda_protocol::*;
use reqwest;
use serde;

use errors::*;

pub struct SdaHttpClient {
    server_root: reqwest::Url,
}

impl SdaHttpClient {

    pub fn new(server_root: &str) -> SdaHttpClientResult<SdaHttpClient> {
        Ok(SdaHttpClient {
            server_root: reqwest::Url::parse(server_root)?
        })
    }

    fn get<T>(&self, path: &str) -> SdaHttpClientResult<T>
        where T: serde::Deserialize
    {
        let url = self.server_root.join(path)?;
        let mut response = reqwest::get(url)?;      
        match *response.status() {

            reqwest::StatusCode::Ok => {
                let obj = response.json()?;
                Ok(obj)
            },

            _ => Err(format!("HTTP/REST status error: {}", response.status()))?,
        }
    }

}

macro_rules! wrap {
    ($e:expr) => {
        match $e {
            Ok(ok) => Ok(ok),
            Err(err) => Err(format!("HTTP/REST error: {}", err).into()),
        }
    }
}

impl SdaService for SdaHttpClient {
    fn ping(&self) -> SdaResult<Pong> {
        wrap! { self.get("/ping") }
    }
}