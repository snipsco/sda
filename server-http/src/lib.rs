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
use std::str::FromStr;

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
        (GET)  (/ping) => { SdaServiceWrapper(&server).ping(req) },

        (GET)  (/agents/{id: AgentId}) => { Disco(&server).get_agent(&id, req) },
        (POST) (/agents/me) => { Disco(&server).create_agent(req) },

        (GET)  (/agents/{id: AgentId}/profile) => { Disco(&server).get_profile(&id, req) },
        (POST) (/agents/me/profile) => { Disco(&server).upsert_profile(req) },

        (GET)    (/agents/any/keys/{id: EncryptionKeyId}) => { Disco(&server).get_encryption_key(&id, req) },
        (POST)   (/agents/me/keys) => { Disco(&server).create_encryption_key(req) },

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

struct Disco<'a>(&'a sda_server::SdaServer);

impl<'a> Disco<'a> {
    fn caller(&self, req: &Request) -> Result<Agent> {
        let auth = auth_token(&req)?;
        Ok(self.0.check_auth_token(&auth)?)
    }

    fn create_agent(&self, req: &Request) -> Result<Response> {
        let auth = auth_token(&req)?;
        let agent: Agent = serde_json::from_reader(req.data().ok_or("Expected a body")?)?;
        if agent.id == auth.id {
            return Ok(client_error("inconsistent agent ids"));
        }
        self.0.create_agent(&agent, &agent)?;
        self.0.upsert_auth_token(&auth)?;
        Ok(Response::empty_404().with_status_code(201))
    }

    fn get_agent(&self, id: &AgentId, req: &Request) -> Result<Response> {
        send_json(self.0.get_agent(&self.caller(req)?, id)?)
    }

    fn get_profile(&self, id: &AgentId, req: &Request) -> Result<Response> {
        send_json(self.0.get_agent(&self.caller(req)?, &id)?)
    }

    fn upsert_profile(&self, req: &Request) -> Result<Response> {
        let profile = serde_json::from_reader(req.data().ok_or("Expected a body")?)?;
        send_json(self.0.upsert_profile(&self.caller(req)?, &profile)?)
    }

    fn get_encryption_key(&self, id: &EncryptionKeyId, req: &Request) -> Result<Response> {
        send_json(self.0.get_encryption_key(&self.caller(req)?, id)?)
    }

    fn create_encryption_key(&self, req: &Request) -> Result<Response> {
        let profile = serde_json::from_reader(req.data().ok_or("Expected a body")?)?;
        send_json(self.0.create_encryption_key(&self.caller(req)?, &profile)?)
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
    let id = AgentId::from_str(&id)?;
    Ok(AuthToken {
        id: id,
        body: body.into(),
    })
}

fn client_error<S: Into<String>>(s: S) -> Response {
    Response::text(s).with_status_code(400)
}

fn send_json<T: ::serde::Serialize>(t: T) -> Result<Response> {
    Ok(Response::from_data("application/json", serde_json::to_string(&t)?))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_auth_token() {
        use sda_protocol::{self, Id, Identified};
        let alice = sda_protocol::Agent {
            id:sda_protocol::AgentId::default(),
            verification_key:sda_protocol::Labeled {
                id:sda_protocol::VerificationKeyId::default(),
                body:sda_protocol::VerificationKey::Sodium(sda_protocol::byte_arrays::B32::default()),
            }
        };
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
