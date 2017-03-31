//! # SDA REST server library
//!
//! This create is the server side HTTP binding for SDA.  It is a library that
//! exposes the SDA service (provided by the `sda_server`) crate as a REST
//! interface.
//!
//! `sda_client_http` crate provides the reverse feature, reconstructing a SDA
//! service API on top of an HTTP client.
//!
//! `sda_server_cli` crate can be used to run a server from the command line
//! without writting the embedding code.
//!
//! ## Protocol methods mapping
//!
//! The HTTP endpoints translate as closely as possible to methods of the
//! `sda_protocol::methods`. The bodies are encoded in json, using `serde`
//! Serialization as setup in `sda_protocol::resources`
//!
//! ```
//! (GET)  (/v1/ping) => SdaBaseService::ping
//! 
//! (GET)  (/v1/agents/{AgentId}) => SdaAgentService::get_agent
//! (POST) (/v1/agents/me) => SdaAgentService::create_agent
//! 
//! (GET)  (/v1/agents/{AgentId}/profile) => SdaAgentService::get_profile
//! (POST) (/v1/agents/me/profile) => SdaAgentService::upsert_profile
//! 
//! (GET)   (/v1/agents/any/keys/{EncryptionKeyId}) =>
//!                         SdaAgentService::get_encryption_key
//! (POST)  (/v1/agents/me/keys) => SdaAgentService::create_encryption_key
//! 
//! (POST)  (/v1/aggregations) => SdaRecipientService::create_aggregation
//! (GET)   (/v1/aggregations) => SdaAggregationService::list_aggregations
//! (GET)   (/v1/aggregations/{AggregationId}) =>
//!                         SdaAggregationService::get_aggregation
//! (DELETE)(/v1/aggregations/{AggregationId}) =>
//!                         SdaRecipientService::delete_aggregation
//! 
//! (GET)   (/v1/aggregations/{AggregationId}/committee/suggestions) =>
//!                         SdaRecipientService::suggest_committee
//! (POST)  (/v1/aggregations/implied/committee) =>
//!                         SdaRecipientService::create_committee
//! (GET)   (/v1/aggregations/{AggregationId}/committee)
//!                         SdaAggregationService::get_committee
//! 
//! (POST)  (/v1/aggregations/participations) =>
//!                         SdaParticipationService::create_participation
//! (GET)   (/v1/aggregations/{AggregationId}/status) =>
//!                         SdaRecipientService::get_aggregation_status
//! 
//! (POST)  (/v1/aggregations/implied/snapshot) =>
//!                         SdaRecipientService::create_committee
//! 
//! (GET)   (/v1/aggregations/any/jobs) => SdaClerkingService::get_clerking_job
//! (POST)  (/v1/aggregations/implied/jobs/{id}/result) =>
//!                         SdaClerkingService::create_clerking_result
//! 
//! (GET)   (/v1/aggregations/{AggregationId}/snapshots/{SnapshotId}/result) =>
//!                         SdaRecipientService::get_snapshot_result
//! ```
//!
//! ## Authentication
//!
//! Authentication relies on the use of then Authenticate header in Basic mode.
//! The username is the caller agent id, the password used at the time of the
//! initial creation is recorded by the server and must then be reused for all
//! subsequent requests.
//!

extern crate data_encoding;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate rouille;
extern crate sda_protocol;
extern crate sda_server;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;

use std::sync;
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
    ($req:expr, $e:expr) => { match $e {
        Ok(resp) => {
            info!("{} {} ({})", $req.method(), $req.raw_url(), resp.status_code);
            resp
        }
        Err(e) => {
            let code = match e {
                Error(ErrorKind::Sda(SdaErrorKind::InvalidCredentials), _) => 401,
                Error(ErrorKind::Sda(SdaErrorKind::PermissionDenied), _) => 403,
                Error(ErrorKind::Sda(SdaErrorKind::Invalid(_)), _) => 400,
                _ => 500,
            };
            error!("{} {} {} ({})", $req.method(), $req.raw_url(), e, code);
            Response::text(format!("{}", e)).with_status_code(code)
        }
    }}
}

/// Run the SDA Rest wrapper.
pub fn listen<A>(addr: A, server: sync::Arc<sda_server::SdaServerService>) -> !
    where A: ToSocketAddrs
{
    rouille::start_server(addr, move |r| handle(&server, r))
}

/// Non blocking `rouille` handler to be used in a loop.
///
/// This is useful for integration tests, not meant to be used as the main API.
pub fn handle(server: &sda_server::SdaServerService, req: &Request) -> Response {
    debug!("Incoming {} {}", req.method(), req.raw_url());
    wrap! { req, router! { req,
        (GET)  (/v1/ping) => { H(&server).ping(req) },

        (GET)  (/v1/agents/{id: AgentId}) => { H(&server).get_agent(&id, req) },
        (POST) (/v1/agents/me) => { H(&server).create_agent(req) },

        (GET)  (/v1/agents/{id: AgentId}/profile) => { H(&server).get_profile(&id, req) },
        (POST) (/v1/agents/me/profile) => { H(&server).upsert_profile(req) },

        (GET)    (/v1/agents/any/keys/{id: EncryptionKeyId}) =>
            { H(&server).get_encryption_key(&id, req) },
        (POST)   (/v1/agents/me/keys) => { H(&server).create_encryption_key(req) },

        (POST)  (/v1/aggregations) => { H(&server).create_aggregation(req) },
        (GET)   (/v1/aggregations) => { H(&server).list_aggregations(req) },
        (GET)   (/v1/aggregations/{id: AggregationId}) => { H(&server).get_aggregation(&id, req) },
        (DELETE)(/v1/aggregations/{id: AggregationId}) => { H(&server).delete_aggregation(&id, req) },

        (GET)   (/v1/aggregations/{id: AggregationId}/committee/suggestions) =>
            { H(&server).suggest_committee(&id, req) },
        (POST)  (/v1/aggregations/implied/committee) => { H(&server).create_committee(req) },
        (GET)   (/v1/aggregations/{id: AggregationId}/committee) =>
            { H(&server).get_committee(&id, req) },

        (POST)  (/v1/aggregations/participations) => { H(&server).create_participation(req) },
        (GET)   (/v1/aggregations/{id: AggregationId}/status) =>
            { H(&server).get_aggregation_status(&id, req) },

        (POST)  (/v1/aggregations/implied/snapshot) => { H(&server).create_snapshot(req) },

        // FIXME. should we revisit these 3 ? the urls feel a bit awkward
        (GET)   (/v1/aggregations/any/jobs) => { H(&server).get_clerking_job(req) },
        (POST)  (/v1/aggregations/implied/jobs/{id}/result) => { H(&server).create_clerking_result(&id, req) },

        (GET)   (/v1/aggregations/{aid}/snapshots/{sid}/result) =>
            { H(&server).get_snapshot_result(&aid, &sid, req) },

        _ => {
            error!("Route not found: {} {}", req.method(), req.raw_url());
            Ok(Response::empty_404())
        }
    } }
}

struct H<'a>(&'a sda_server::SdaServerService);

impl<'a> H<'a> {
    fn caller(&self, req: &Request) -> Result<Agent> {
        let auth = auth_token(&req)?;
        Ok((self.0).0.check_auth_token(&auth)?)
    }

    fn ping(&self, _req: &Request) -> Result<Response> {
        send_json_option(Some(self.0.ping()?))
    }

    fn create_agent(&self, req: &Request) -> Result<Response> {
        let auth = auth_token(&req)?;
        let agent: Agent = read_json(&req)?;
        if agent.id != auth.id {
            return Ok(client_error("inconsistent agent ids"));
        }
        self.0.create_agent(&agent, &agent)?;
        (self.0).0.upsert_auth_token(&auth)?;
        send_empty_201()
    }

    fn get_agent(&self, id: &AgentId, req: &Request) -> Result<Response> {
        send_json_option(self.0.get_agent(&self.caller(req)?, id)?)
    }

    fn get_profile(&self, id: &AgentId, req: &Request) -> Result<Response> {
        send_json_option(self.0.get_profile(&self.caller(req)?, &id)?)
    }

    fn upsert_profile(&self, req: &Request) -> Result<Response> {
        self.0.upsert_profile(&self.caller(req)?, &read_json(req)?)?;
        send_empty_201()
    }

    fn get_encryption_key(&self, id: &EncryptionKeyId, req: &Request) -> Result<Response> {
        send_json_option(self.0.get_encryption_key(&self.caller(req)?, id)?)
    }

    fn create_encryption_key(&self, req: &Request) -> Result<Response> {
        self.0.create_encryption_key(&self.caller(req)?, &read_json(req)?)?;
        send_empty_201()
    }

    fn get_aggregation(&self, id: &AggregationId, req: &Request) -> Result<Response> {
        send_json_option(self.0.get_aggregation(&self.caller(req)?, id)?)
    }

    fn create_aggregation(&self, req: &Request) -> Result<Response> {
        self.0.create_aggregation(&self.caller(req)?, &read_json(&req)?)?;
        send_empty_201()
    }

    fn list_aggregations(&self, req: &Request) -> Result<Response> {
        let filter = req.get_param("title");
        let recipient = if let Some(p) = req.get_param("recipient") {
            Some(p.parse()?)
        } else {
            None
        };
        send_json_option(Some(self.0
            .list_aggregations(&self.caller(req)?,
                               filter.as_ref().map(|s| &**s),
                               recipient.as_ref())?))
    }

    fn delete_aggregation(&self, id: &AggregationId, req: &Request) -> Result<Response> {
        self.0.delete_aggregation(&self.caller(req)?, id)?;
        send_empty_200()
    }

    fn suggest_committee(&self, id: &AggregationId, req: &Request) -> Result<Response> {
        send_json(self.0.suggest_committee(&self.caller(req)?, &id)?)
    }

    fn create_committee(&self, req: &Request) -> Result<Response> {
        self.0.create_committee(&self.caller(req)?, &read_json(&req)?)?;
        send_empty_201()
    }

    fn get_committee(&self, id: &AggregationId, req: &Request) -> Result<Response> {
        send_json_option(self.0.get_committee(&self.caller(req)?, id)?)
    }

    fn create_participation(&self, req: &Request) -> Result<Response> {
        self.0.create_participation(&self.caller(req)?, &read_json(&req)?)?;
        send_empty_201()
    }

    fn get_aggregation_status(&self, id: &AggregationId, req: &Request) -> Result<Response> {
        send_json_option(self.0.get_aggregation_status(&self.caller(req)?, id)?)
    }

    fn create_snapshot(&self, req: &Request) -> Result<Response> {
        self.0.create_snapshot(&self.caller(req)?, &read_json(&req)?)?;
        send_empty_201()
    }

    fn get_clerking_job(&self, req: &Request) -> Result<Response> {
        let caller = self.caller(req)?;
        send_json_option(self.0.get_clerking_job(&caller, &caller.id)?)
    }

    fn create_clerking_result(&self, _id: &ClerkingJobId, req: &Request) -> Result<Response> {
        self.0.create_clerking_result(&self.caller(req)?, &read_json(&req)?)?;
        send_empty_201()
    }

    fn get_snapshot_result(&self,
                           aggregation: &AggregationId,
                           snapshot: &SnapshotId,
                           req: &Request)
                           -> Result<Response> {
        send_json_option(self.0.get_snapshot_result(&self.caller(req)?, aggregation, snapshot)?)
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

#[allow(dead_code)]
fn send_empty_200() -> Result<Response> {
    Ok(Response::empty_404().with_status_code(200))
}

fn send_empty_201() -> Result<Response> {
    Ok(Response::empty_404().with_status_code(201))
}

fn read_json<T: ::serde::Deserialize>(req: &Request) -> Result<T> {
    Ok(serde_json::from_reader(req.data().ok_or("Expected a body")?)?)
}

fn send_json<T: ::serde::Serialize>(t: T) -> Result<Response> {
    send_json_option(Some(t))
}

fn send_json_option<T: ::serde::Serialize>(t: Option<T>) -> Result<Response> {
    match t {
        None => Ok(Response::empty_404().with_additional_header("Resource-not-found", "true")),
        Some(t) => Ok(Response::from_data("application/json", serde_json::to_string(&t)?)),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_auth_token() {
        use sda_protocol::{self, Id, Identified};
        let alice = sda_protocol::Agent {
            id: sda_protocol::AgentId::default(),
            verification_key: sda_protocol::Labelled {
                id: sda_protocol::VerificationKeyId::default(),
                body:
                    sda_protocol::VerificationKey::Sodium(sda_protocol::byte_arrays::B32::default()),
            },
        };
        let secret = "s0m3_s3cr3t_t0k3n";
        let authorization_raw = format!("{}:{}", alice.id().to_string(), secret);
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
