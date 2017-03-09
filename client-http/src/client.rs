use sda_protocol::*;
use reqwest::{self, Url, StatusCode, Method, RequestBuilder, Response};
use reqwest::header::*;
use serde;

use errors::*;
use tokenstore::*;

pub struct SdaHttpClient<S> {
    client: reqwest::Client,
    server_root: Url,
    token_store: S,
}

impl<S: TokenStore> SdaHttpClient<S> {

    pub fn new(server_root: &str, token_store: S) -> SdaHttpClientResult<SdaHttpClient<S>> {
        Ok(SdaHttpClient {
            client: reqwest::Client::new()?,
            server_root: Url::parse(server_root)?,
            token_store: token_store,
        })
    }

    fn decorate(&self, mut request: RequestBuilder, caller: Option<&Agent>) -> SdaHttpClientResult<RequestBuilder> {
        // user agent
        request = request
            .header(UserAgent("SDA CLI client".to_string()));

        // auth token
        if let Some(agent) = caller {
            let auth_token = self.token_store.get()?;
            request = request
                .header(Authorization(Basic {
                    username: agent.id.stringify(),
                    password: Some(auth_token),
                }));
        }

        Ok(request)
    }

    fn process<U>(&self, mut response: Response) -> SdaHttpClientResult<Option<U>>
        where U: serde::Deserialize
    {
        let content_length = match response.headers().get::<ContentLength>() {
            None => 0,
            Some(length) => length.0,
        };

        let status = *response.status();
        match status {

              StatusCode::Ok
            | StatusCode::Created
            => {
                if content_length > 0 {
                    let obj = response.json()?;
                    Ok(Some(obj))
                } else {
                    Ok(None)
                }
            },

            StatusCode::NotFound
            => {
                if response.headers().get_raw("Resource-not-found").is_some() {
                    Ok(None)
                } else {
                    Err("HTTP/REST route not found")?
                }
            },

            StatusCode::Unauthorized => { Err(SdaHttpClientErrorKind::Sda(SdaErrorKind::InvalidCredentials).into()) }
            StatusCode::Forbidden => { Err(SdaHttpClientErrorKind::Sda(SdaErrorKind::PermissionDenied).into()) }

            _ => {
                use std::io::Read;
                let mut payload = String::new();
                match response.read_to_string(&mut payload) {
                    Ok(_) => {
                        Err(format!("HTTP/REST error: {} {}", response.status(), payload))
                    },
                    Err(_) => {
                        Err(format!("HTTP/REST error: {}", response.status()))
                    }
                }?
            }
        }
    }

    pub fn get<U>(&self, caller: Option<&Agent>, url: Url) -> SdaHttpClientResult<Option<U>>
        where U: serde::Deserialize
    {
        let request = self.client
            .get(url);

        let response = self.decorate(request, caller)?.send()?;
        self.process(response)
    }

    pub fn post<T, U>(&self, caller: Option<&Agent>, url: Url, body: &T) -> SdaHttpClientResult<Option<U>>
        where
            T: serde::Serialize,
            U: serde::Deserialize,
    {
        let request = self.client
            .post(url)
            .json(body);

        let response = self.decorate(request, caller)?.send()?;
        self.process(response)
    }

    pub fn delete<U>(&self, caller: Option<&Agent>, url: Url) -> SdaHttpClientResult<Option<U>>
        where U: serde::Deserialize
    {
        let request = self.client
            .request(Method::Delete, url);

        let response = self.decorate(request, caller)?.send()?;
        self.process(response)
    }

    pub fn url<P: AsRef<str>>(&self, path: P) -> SdaResult<Url> {
        Ok(
            self.server_root.join(path.as_ref())
            .map_err(|e| format!("Url formatting error {:?}", e))?
        )
    }

}

macro_rules! wrap_empty {
    ($e:expr) => {
        match $e {
            Ok(Some(_)) => Err("Extra response payload".into()),
            Ok(None) => Ok(()),
            Err(SdaHttpClientError(SdaHttpClientErrorKind::Sda(e), _)) => Err(e.into()),
            Err(err) => Err(format!("HTTP/REST error: {}", err).into()),
        }
    }
}

macro_rules! wrap_payload {
    ($e:expr) => {
        match $e {
            Ok(Some(obj)) => Ok(obj),
            Ok(None) => Err("Missing response payload".into()),
            Err(SdaHttpClientError(SdaHttpClientErrorKind::Sda(e), _)) => Err(e.into()),
            Err(err) => Err(format!("HTTP/REST error: {}", err).into()),
        }
    }
}

macro_rules! wrap_option_payload {
    ($e:expr) => {
        match $e {
            Ok(Some(obj)) => Ok(obj),
            Ok(None) => Ok(None),
            Err(SdaHttpClientError(SdaHttpClientErrorKind::Sda(e), _)) => Err(e.into()),
            Err(err) => Err(format!("HTTP/REST error: {}", err).into()),
        }
    }
}

impl<S> SdaService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{}

impl<S> SdaBaseService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{
    fn ping(&self) -> SdaResult<Pong> {
        wrap_payload! { self.get(
            None,
            self.url("/ping")?
        ) }
    }
}

impl<S> SdaAgentService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{

    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        wrap_empty! { self.post::<Agent, ()>(
            Some(caller),
            self.url("/agents/me")?,
            agent
        ) }
    }

    fn get_agent(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        wrap_option_payload! { self.get(
            Some(caller),
            self.url(format!("/agents/{}", owner.stringify()))?
        ) }
    }

    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()> {
        wrap_empty! { self.post::<Profile, ()>(
            Some(caller),
            self.url("/agents/me/profile")?,
            profile
        ) }
    }

    fn get_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        wrap_option_payload! { self.get(
            Some(caller),
            self.url(format!("/agents/{}/profile", owner.stringify()))?
        ) }
    }

    fn create_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()> {
        wrap_empty! { self.post::<SignedEncryptionKey, ()>(
            Some(caller),
            self.url("/agents/me/keys")?,
            key
        ) }
    }

    fn get_encryption_key(&self, caller: &Agent, key: &EncryptionKeyId) -> SdaResult<Option<SignedEncryptionKey>> {
        wrap_option_payload! { self.get(
            Some(caller),
            self.url(format!("/agents/any/keys/{}", key.stringify()))?
        ) }
    }

}

impl<S> SdaAggregationService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{

    fn list_aggregations(&self, caller: &Agent, filter: Option<&str>, recipient: Option<&AgentId>) -> SdaResult<Vec<AggregationId>> {

        let mut url = self.server_root.clone();
        url.path_segments_mut().map_err(|e| format!("Url formatting error {:?}", e))?
            .push("aggregations");

        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("title", filter);
        }
        if let Some(recipient) = recipient {
            url.query_pairs_mut().append_pair("recipient", &recipient.stringify());
        }

        wrap_payload! { self.get(
            Some(caller),
            url
        ) }
    }

    fn get_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Aggregation>> {
        wrap_option_payload! { self.get(
            Some(caller),
            self.url(format!("/aggregations/{}", aggregation.stringify()))?
        ) }
    }

    fn get_committee(&self, caller: &Agent, owner: &AggregationId) -> SdaResult<Option<Committee>> {
        wrap_option_payload! { self.get(
            Some(caller),
            self.url(format!("/aggregations/{}/committee", owner.stringify()))?
        ) }
    }

}

impl<S> SdaParticipationService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{

    fn create_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()> {
        wrap_empty! { self.post::<Participation, ()>(
            Some(caller),
            self.url("/aggregations/participations")?,
            participation
        ) }
    }

}

impl<S> SdaClerkingService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{

    fn get_clerking_job(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkingJob>> {
        wrap_option_payload! { self.get(
            Some(caller),
            self.url(format!("/aggregations/any/jobs/{}", clerk.stringify()))?
        ) }
    }

    fn create_clerking_result(&self, caller: &Agent, result: &ClerkingResult) -> SdaResult<()> {
        wrap_empty! { self.post::<ClerkingResult, ()>(
            Some(caller),
            self.url(format!("/aggregations/{}/jobs/{}/result", result.aggregation.stringify(), result.job.stringify()))?,
            result
        ) }
    }

}

impl<S> SdaRecipientService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{

    fn create_aggregation(&self, caller: &Agent, aggregation: &Aggregation) -> SdaResult<()> {
        wrap_empty! { self.post::<Aggregation, ()>(
            Some(caller),
            self.url("/aggregations")?,
            aggregation
        ) }
    }

    fn suggest_committee(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Vec<ClerkCandidate>> {
        wrap_payload! { self.get(
            Some(caller),
            self.url(format!("/aggregations/{}/committee/suggestions", aggregation.stringify()))?
        ) }
    }

    fn create_committee(&self, caller: &Agent, committee: &Committee) -> SdaResult<()> {
        wrap_empty! { self.post::<Committee, ()>(
            Some(caller),
            self.url("/aggregations/implied/committee")?,
            committee
        ) }
    }

    fn get_aggregation_status(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<AggregationStatus>> {
        wrap_option_payload! { self.get(
            Some(caller), 
            self.url(format!("/aggregations/{}/status", aggregation.stringify()))?
        ) }
    }

    fn create_snapshot(&self, caller: &Agent, snapshot:&Snapshot) -> SdaResult<()> {
        wrap_empty! { self.post::<Snapshot, ()>(
            Some(caller),
            self.url("/aggregations/implied/snapshot")?,
            snapshot
        ) }
    }

    fn get_aggregation_results(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Vec<AggregationResult>> {
        wrap_payload! { self.get(
            Some(caller), 
            self.url(format!("/aggregations/{}/results", aggregation.stringify()))?
        ) }
    }


    fn delete_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<()> {
        wrap_empty! { self.delete::<()>(
            Some(caller),
            self.url(format!("/aggregations/{}", aggregation.stringify()))?
        ) }
    }

}
