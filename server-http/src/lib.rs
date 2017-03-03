extern crate data_encoding;
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
use sda_server::stores::AuthToken;
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
        (GET) (/agents/{id: String}) => { SdaDiscoveryServiceWrapper(&server).get_agent(&*id, req) },
        (POST) (/agents/{id: String}) => { SdaDiscoveryServiceWrapper(&server).create_agent(&*id, req) },
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
    fn create_agent(&self, id:&str, req: &Request) -> Result<Response> {
        let id = AgentId::destringify(&id)?;
        let auth = auth_token(&req)?;
        let agent:Agent = serde_json::from_reader(req.data().ok_or("Expected a body")?)?;
        if agent.id != id || auth.id != id {
            return Ok(client_error("inconsistent agent ids"))
        }
        self.0.create_agent(&agent, &agent)?;
        self.0.upsert_auth_token(&auth)?;
        Ok(Response::empty_404().with_status_code(201))
    }

    fn get_agent(&self, id:&str, req:&Request) -> Result<Response> {
        let auth = auth_token(&req)?;
        let caller = self.0.check_auth_token(&auth)?;
        let id = AgentId::destringify(id)?;
        send_json(self.0.get_agent(&caller, &id)?)
    }
}

fn auth_token(req: &Request) -> Result<AuthToken> {
    let header = req.header("Authorization").ok_or("Mandatory Authorization header")?.trim();
    if !header.starts_with("Basic ") {
        Err("Basic Authorization required")?;
    }
    let value = header.replace("Basic ", "");
    let decoded = data_encoding::base64::decode(value.as_bytes())
        .map_err(|e| format!("Invalid Auth header (base64: {:?})",e))?;
    let string = String::from_utf8(decoded).map_err(|_| "Invalid Auth header(not utf8)")?;
    let mut split = string.split(":");
    let id = split.next().ok_or("Invalid Auth header")?;
    let body = split.next().ok_or("Invalid Auth header")?;
    let id = AgentId::destringify(&id)?;
    Ok(AuthToken {
        id: id,
        body: body.into(),
    })
}

fn client_error<S:Into<String>>(s:S) -> Response {
    Response::text(s).with_status_code(400)
}

fn send_json<T: ::serde::Serialize>(t: T) -> Result<Response> {
    Ok(Response::from_data("application/json", serde_json::to_string(&t)?))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_auth_token() {
        use sda_protocol::{Id, Identified};
        let alice = ::sda_protocol::Agent::default();
        let secret = "s0m3_s3cr3t_t0k3n";
        let authorization_raw = format!("{}:{}", alice.id().stringify(), secret);
        let header = format!("Basic {}",
                             ::data_encoding::base64::encode(authorization_raw.as_bytes()));
        let req = ::rouille::Request::fake_http("GET",
                                                "/",
                                                vec![("Authorization".into(), header)],
                                                vec![]);
        let auth_token = super::auth_token(&req).unwrap();
        assert_eq!(::sda_server::stores::AuthToken {
                       id: *alice.id(),
                       body: secret.to_string(),
                   },
                   auth_token);
    }

}
