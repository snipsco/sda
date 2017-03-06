use sda_protocol::*;
use reqwest;
use reqwest::header::*;
use serde;

use errors::*;
use tokenstore::*;

pub struct SdaHttpClient<S> {
    client: reqwest::Client,
    server_root: reqwest::Url,
    token_store: S,
}

impl<S: TokenStore> SdaHttpClient<S> {

    pub fn new(server_root: &str, token_store: S) -> SdaHttpClientResult<SdaHttpClient<S>> {
        Ok(SdaHttpClient {
            client: reqwest::Client::new()?,
            server_root: reqwest::Url::parse(server_root)?,
            token_store: token_store,
        })
    }

    fn decorate(&self, mut request: reqwest::RequestBuilder, caller: Option<&Agent>) -> SdaHttpClientResult<reqwest::RequestBuilder> {
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

    fn process<U>(&self, mut response: reqwest::Response) -> SdaHttpClientResult<Option<U>>
        where U: serde::Deserialize
    {
        let content_length = match response.headers().get::<reqwest::header::ContentLength>() {
            None => 0,
            Some(length) => length.0,
        };

        let status = *response.status();
        match status {

              reqwest::StatusCode::Ok
            | reqwest::StatusCode::Created
            => {
                if content_length > 0 {
                    let obj = response.json()?;
                    Ok(Some(obj))
                } else {
                    Ok(None)
                }
            },

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

    pub fn get<U>(&self, caller: Option<&Agent>, path: &str) -> SdaHttpClientResult<Option<U>>
        where U: serde::Deserialize
    {
        let url = self.server_root.join(path)?;
        let request = self.client
            .get(url);

        let response = self.decorate(request, caller)?.send()?;
        self.process(response)
    }

    pub fn post<T, U>(&self, caller: Option<&Agent>, path: &str, body: &T) -> SdaHttpClientResult<Option<U>>
        where 
            T: serde::Serialize,
            U: serde::Deserialize,
    {
        let url = self.server_root.join(path)?;
        let request = self.client
            .post(url)
            .json(body);

        let response = self.decorate(request, caller)?.send()?;
        self.process(response)
    }

}

macro_rules! wrap_empty {
    ($e:expr) => {
        match $e {
            Ok(Some(_)) => Err("Extra response payload".into()),
            Ok(None) => Ok(()),
            Err(err) => Err(format!("HTTP/REST error: {}", err).into()),
        }
    }
}

macro_rules! wrap_payload {
    ($e:expr) => {
        match $e {
            Ok(Some(obj)) => Ok(obj),
            Ok(None) => Err("Missing response payload".into()),
            Err(err) => Err(format!("HTTP/REST error: {}", err).into()),
        }
    }
}

impl<S> SdaService for SdaHttpClient<S>
    where S: Send + Sync + TokenStore
{
    fn ping(&self) -> SdaResult<Pong> {
        wrap_payload! { self.get(
            None, 
            "/ping"
        ) }
    }
}

impl<S> SdaAgentService for SdaHttpClient<S> 
    where S: Send + Sync + TokenStore
{

    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        wrap_empty! { self.post::<Agent, ()>(
            Some(caller),
            &format!("/agents/me"), 
            agent
        ) }
    }

    fn get_agent(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/agents/{}", owner.stringify())
        ) }
    }

    #[allow(unused_variables)]
    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()> {
        wrap_payload! { self.post(
            Some(caller), 
            &format!("/agents/me/profile"),
            profile
        ) }
    }

    fn get_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/agents/{}/profile", owner.stringify())
        ) }
    }

    fn create_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()> {
        wrap_empty! { self.post::<SignedEncryptionKey, ()>(
            Some(caller), 
            &format!("/agents/me/keys"), 
            key
        ) }
    }

    fn get_encryption_key(&self, caller: &Agent, key: &EncryptionKeyId) -> SdaResult<Option<SignedEncryptionKey>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/agents/any/keys/{}", key.stringify())
        ) }
    }

}

impl<S> SdaAggregationService for SdaHttpClient<S> 
    where S: Send + Sync + TokenStore
{

    #[allow(unused_variables)]
    fn list_aggregations_by_title(&self, caller: &Agent, filter: &str) -> SdaResult<Vec<AggregationId>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn list_aggregations_by_recipient(&self, caller: &Agent, recipient: &AgentId) -> SdaResult<Vec<AggregationId>> {
        unimplemented!()
    }

    fn get_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Aggregation>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/aggregations/{}", aggregation.stringify())
        ) }
    }

    fn get_committee(&self, caller: &Agent, owner: &AggregationId) -> SdaResult<Option<Committee>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/aggregations/{}/committee", owner.stringify())
        ) }
    }

}

impl<S> SdaParticipationService for SdaHttpClient<S> 
    where S: Send + Sync + TokenStore
{

    fn create_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()> {
        wrap_empty! { self.post::<Participation, ()>(
            Some(caller), 
            &format!("/aggregations/{}/participations", participation.aggregation.stringify()),
            participation
        ) }
    }

}

impl<S> SdaClerkingService for SdaHttpClient<S> 
    where S: Send + Sync + TokenStore
{

    fn get_clerking_job(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkingJob>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/aggregations/any/jobs/{}", clerk.stringify())
        ) }
    }

    fn create_clerking_result(&self, caller: &Agent, result: &ClerkingResult) -> SdaResult<()> {
        wrap_empty! { self.post::<ClerkingResult, ()>(
            Some(caller), 
            &format!("/aggregations/{}/jobs/{}/result", result.aggregation.stringify(), result.job.stringify()),
            result
        ) }
    }

}

impl<S> SdaRecipientService for SdaHttpClient<S> 
    where S: Send + Sync + TokenStore
{

    fn get_aggregation_status(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<AggregationStatus>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/aggregations/{}/status", aggregation.stringify())
        ) }
    }

    fn get_aggregation_results(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Vec<AggregationResult>> {
        wrap_payload! { self.get(
            Some(caller), 
            &format!("/aggregations/{}/results", aggregation.stringify())
        ) }
    }

}