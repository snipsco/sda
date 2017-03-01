#[macro_use]
extern crate rouille;
extern crate sda_protocol;
extern crate sda_server;
#[macro_use]
extern crate serde_json;

use std::net::ToSocketAddrs;

use rouille::{Request, Response};

use sda_protocol::*;

macro_rules! wrap {
    ($e:expr) => { match $e {
        Ok(resp) => resp,
        Err(e) => Response::text(format!("{:?}", e)).with_status_code(500),
    }}
}

pub fn listen<A>(addr: A, server: sda_server::SdaServer) -> !
    where A: ToSocketAddrs {
    rouille::start_server(addr, move |req| wrap! { router! { req,
        (GET) (/ping) => { SdaServiceWrapper(&server).ping(req) },
        _ => Ok(Response::empty_404())
    } })
}

struct SdaServiceWrapper<'a>(&'a sda_server::SdaServer);

impl<'a> SdaServiceWrapper<'a> {
    fn ping(&self, _req:&Request) -> SdaResult<Response> {
        self.0.ping()?;
        Ok(Response::from_data("application/json", json!({"running": true}).to_string()))
    }
}
