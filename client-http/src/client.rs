use sda_protocol::*;
use reqwest;
use serde;

use errors::*;

pub struct SdaHttpClient {
    server_root: reqwest::Url,
    client: reqwest::Client,
}

impl SdaHttpClient {

    pub fn new(server_root: &str) -> SdaHttpClientResult<SdaHttpClient> {
        Ok(SdaHttpClient {
            server_root: reqwest::Url::parse(server_root)?,
            client: reqwest::Client::new()?,
        })
    }

    fn get<U>(&self, path: &str) -> SdaHttpClientResult<U>
        where U: serde::Deserialize
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

    fn post<T, U>(&self, path: &str, body: &T) -> SdaHttpClientResult<U>
        where 
            T: serde::Serialize,
            U: serde::Deserialize,
    {
        let url = self.server_root.join(path)?;
        let mut response = self.client.post(url)
            .json(body)
            .send()?;
            
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

#[allow(unused_variables)]
impl SdaDiscoveryService for SdaHttpClient {

    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        let endpoint = format!("/agents/{}", agent.id.stringify());
        wrap! { self.post(&endpoint, agent) }
    }

    fn get_agent(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        unimplemented!()
    }

    fn list_aggregations_by_title(&self, caller: &Agent, filter: &str) -> SdaResult<Vec<AggregationId>> {
        unimplemented!()
    }

    fn list_aggregations_by_recipient(&self, caller: &Agent, recipient: &AgentId) -> SdaResult<Vec<AggregationId>> {
        unimplemented!()
    }

    fn get_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Aggregation>> {
        unimplemented!()
    }

    fn get_committee(&self, caller: &Agent, owner: &AggregationId) -> SdaResult<Option<Committee>> {
        unimplemented!()
    }

    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()> {
        unimplemented!()
    }

    fn get_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        unimplemented!()
    }

    fn create_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()> {
        unimplemented!()
    }

    fn get_encryption_key(&self, caller: &Agent, key: &EncryptionKeyId) -> SdaResult<Option<SignedEncryptionKey>> {
        unimplemented!()
    }

}