
//! This crate provides HTTP access to the SDA services for clients.

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

extern crate sda_protocol;


use sda_protocol::*;


error_chain!{
    types {
        SdaHttpClientError, SdaHttpClientErrorKind, SdaHttpClientResultExt, SdaHttpClientResult;
    }
    foreign_links {
        Protocol(::sda_protocol::SdaError);
        SerdeJson(::serde_json::Error);
        Http(::reqwest::Error);
        Url(::reqwest::UrlError);
    }

}


macro_rules! wrap {
    ($e:expr) => {
        match $e {
            Ok(ok) => Ok(ok),
            Err(err) => Err(format!("HTTP/REST error: {:?}", err).into()),
        }
    }
}


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


impl SdaService for SdaHttpClient {
    fn ping(&self) -> SdaResult<Pong> {
        wrap! { self.get("/ping") }
    }
}
