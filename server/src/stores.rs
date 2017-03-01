use sda_protocol::{Agent, AgentId, Profile, EncryptionKeyId, SignedEncryptionKey};
use SdaServerResult;

pub trait BaseStore : Sync + Send {
    fn ping(&self) -> SdaServerResult<()>;
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
