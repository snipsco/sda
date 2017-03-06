use sda_protocol::*;
use reqwest;
use reqwest::header::*;
use serde;

use errors::*;
use authtoken::*;

pub struct SdaHttpClient<S> {
    client: reqwest::Client,
    server_root: reqwest::Url,
    auth_token: S,
}

impl<S: AuthTokenStore> SdaHttpClient<S> {

    pub fn new(server_root: &str, auth_token_store: S) -> SdaHttpClientResult<SdaHttpClient<S>> {
        Ok(SdaHttpClient {
            client: reqwest::Client::new()?,
            server_root: reqwest::Url::parse(server_root)?,
            auth_token: auth_token_store,
        })
    }

    fn decorate(&self, mut request: reqwest::RequestBuilder, caller: Option<&Agent>) -> SdaHttpClientResult<reqwest::RequestBuilder> {
        // user agent
        request = request
            .header(UserAgent("SDA CLI client".to_string()));
        
        // auth token
        if let Some(agent) = caller {
            let auth_token = self.auth_token.get()?;
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
    where S: Send + Sync + AuthTokenStore
{
    fn ping(&self) -> SdaResult<Pong> {
        wrap_payload! { self.get(None, "/ping") }
    }
}

#[allow(unused_variables)]
impl<S> SdaAgentService for SdaHttpClient<S> 
    where S: Send + Sync + AuthTokenStore
{

    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        let endpoint = format!("/agents/{}", agent.id.stringify());
        wrap_empty! { self.post::<Agent, ()>(Some(caller), &endpoint, agent) }
    }

    fn get_agent(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        unimplemented!()
    }

    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()> {
        unimplemented!()
    }

    fn get_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        unimplemented!()
    }

    fn create_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()> {
        // let endpoint = format!("/agents/{}", agent.id.stringify());
        // wrap_empty! { self.post::<Agent, ()>(Some(caller), &endpoint, agent) }
        unimplemented!()
    }

    fn get_encryption_key(&self, caller: &Agent, key: &EncryptionKeyId) -> SdaResult<Option<SignedEncryptionKey>> {
        unimplemented!()
    }

}