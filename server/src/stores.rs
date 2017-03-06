use sda_protocol::{Agent, AgentId, Labeled, Profile, EncryptionKeyId, SignedEncryptionKey};
use sda_protocol::{Aggregation, AggregationId, Committee};
use SdaServerResult;

pub trait BaseStore : Sync + Send {
    fn ping(&self) -> SdaServerResult<()>;
}

pub type AuthToken = Labeled<AgentId, String>;

pub trait AuthStore: BaseStore {
    /// Save an auth token
    fn upsert_auth_token(&self, token:&AuthToken) -> SdaServerResult<()>;

    /// Retrieve an auth token
    fn get_auth_token(&self, id:&AgentId) -> SdaServerResult<Option<AuthToken>>;

    /// Delete an auth token
    fn delete_auth_token(&self, id:&AgentId) -> SdaServerResult<()>;
}

pub trait AgentStore: BaseStore {
    /// Create an agent
    fn create_agent(&self, agent: &Agent) -> SdaServerResult<()>;

    /// Retrieve the agent description.
    fn get_agent(&self, id: &AgentId) -> SdaServerResult<Option<Agent>>;

    /// Register the given public profile; updates any existing profile.
    fn upsert_profile(&self, profile: &Profile) -> SdaServerResult<()>;

    /// Retrieve the associated public profile.
    fn get_profile(&self, owner: &AgentId) -> SdaServerResult<Option<Profile>>;

    /// Register new encryption key for agent.
    fn create_encryption_key(&self, key: &SignedEncryptionKey) -> SdaServerResult<()>;

    /// Retrieve agent encryption key.
    fn get_encryption_key(&self, key: &EncryptionKeyId) -> SdaServerResult<Option<SignedEncryptionKey>>;
}

pub trait AggregationsStore: BaseStore {
    /// Search for aggregations with titles matching the filter.
    fn list_aggregations_by_title(&self, filter: &str) -> SdaServerResult<Vec<AggregationId>>;

    /// Search for aggregations with specific recipient.
    fn list_aggregations_by_recipient(&self, recipient: &AgentId) -> SdaServerResult<Vec<AggregationId>>;

    /// Create a new aggregation on the service (without any associated result).
    /// If successful, the original id has been replaced by the returned id.
    fn create_aggregation(&self, aggregation: &Aggregation) -> SdaServerResult<()>;

    /// Retrieve an aggregation and its description.
    fn get_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Aggregation>>;

    /// Delete all information (including results) regarding an aggregation.
    fn delete_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<()>;

    /// Retrieve the associated committee.
    fn get_committee(&self, owner: &AggregationId) -> SdaServerResult<Option<Committee>>;

    fn create_committee(&self, committee: &Committee) -> SdaServerResult<()>;
}
