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
    where A: ToSocketAddrs {
    rouille::start_server(addr, move |req| wrap! { router! { req,
        (GET) (/ping) => { SdaServiceWrapper(&server).ping(req) },
        _ => Ok(Response::empty_404())
    } })
}

struct SdaServiceWrapper<'a>(&'a sda_server::SdaServer);

impl<'a> SdaServiceWrapper<'a> {
    fn ping(&self, _req:&Request) -> Result<Response> {
        send_json(self.0.ping()?)
    }
}

fn send_json<T: ::serde::Serialize>(t:T) -> Result<Response> {
    Ok(Response::from_data("application/json", serde_json::to_string(&t)?))
}
