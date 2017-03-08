use sda_protocol::*;
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

    /// FIXME: very temporary interface. As logic needs to be adapted to each store
    /// capabilities, no real need to abstract this in server, but we do need to
    /// give more information about what is needed (supported keys, liveliness,
    /// number, ...)
    fn suggest_committee(&self) -> SdaServerResult<Vec<ClerkCandidate>>;
}

pub trait AggregationsStore: BaseStore {
    /// Search for aggregations by title and/or by recipient.
    fn list_aggregations(&self, filter: Option<&str>, recipient:Option<&AgentId>) -> SdaServerResult<Vec<AggregationId>>;

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

    fn create_participation(&self, participation: &Participation) -> SdaServerResult<()>;
}
