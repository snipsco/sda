#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate rouille;
extern crate sda_protocol;
extern crate sda_server;
extern crate serde;
extern crate serde_json;

use std::net::ToSocketAddrs;

use rouille::{Request, Response};

use sda_protocol::*;
use errors::*;

mod errors {
    error_chain! {
        links {
            Sda(::sda_protocol::SdaError, ::sda_protocol::SdaErrorKind);
        }
        foreign_links {
            SerdeJson(::serde_json::Error);
        }
    }
}

macro_rules! wrap {
    ($e:expr) => { match $e {
        Ok(resp) => resp,
        Err(e) => Response::text(format!("{:?}", e)).with_status_code(500),
    }}
}

pub fn listen<A>(addr: A, server: sda_server::SdaServer) -> !
    where A: ToSocketAddrs
{
    rouille::start_server(addr, move |req| {
        wrap! { router! { req,
        (GET) (/ping) => { SdaServiceWrapper(&server).ping(req) },
        (POST) (/agents/{id: String}) => { SdaDiscoveryServiceWrapper(&server).create_agent(req) },
        _ => Ok(Response::empty_404())
    } }
    })
}

struct SdaServiceWrapper<'a>(&'a sda_server::SdaServer);

impl<'a> SdaServiceWrapper<'a> {
    fn ping(&self, _req: &Request) -> Result<Response> {
        send_json(self.0.ping()?)
    }
}

struct SdaDiscoveryServiceWrapper<'a>(&'a sda_server::SdaServer);

impl<'a> SdaDiscoveryServiceWrapper<'a> {
    fn create_agent(&self, req: &Request) -> Result<Response> {
        let agent = serde_json::from_reader(req.data().ok_or("Expected a body")?)?;
        self.0.create_agent(&agent, &agent)?;
        Ok(Response::empty_404().with_status_code(201))
    }
}

fn send_json<T: ::serde::Serialize>(t: T) -> Result<Response> {
    Ok(Response::from_data("application/json", serde_json::to_string(&t)?))
}

