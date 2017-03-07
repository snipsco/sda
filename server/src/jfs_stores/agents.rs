use jfs;

use std::path;

use sda_protocol::{Id, Identified};
use sda_protocol::{Agent, AgentId, ClerkCandidate, Profile, SignedEncryptionKey, EncryptionKeyId};

use SdaServerResult;
use stores::{BaseStore, AgentStore};

use itertools::Itertools;

pub struct JfsAgentStore {
    agents: jfs::Store,
    profiles: jfs::Store,
    encryption_keys: jfs::Store,
}

impl JfsAgentStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsAgentStore> {
        let agents = prefix.as_ref().join("agents");
        let profiles = prefix.as_ref().join("profiles");
        let encryption_keys = prefix.as_ref().join("encryption_keys");
        Ok(JfsAgentStore {
            agents: jfs::Store::new(agents.to_str().ok_or("pathbuf to string")?)?,
            profiles: jfs::Store::new(profiles.to_str().ok_or("pathbuf to string")?)?,
            encryption_keys: jfs::Store::new(encryption_keys.to_str().ok_or("pathbuf to string")?)?,
        })
    }
}

impl BaseStore for JfsAgentStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl AgentStore for JfsAgentStore {
    fn create_agent(&self, agent: &Agent) -> SdaServerResult<()> {
        self.agents.save_with_id(agent, &agent.id().stringify())?;
        Ok(())
    }

    fn get_agent(&self, id: &AgentId) -> SdaServerResult<Option<Agent>> {
        super::get_option(&self.agents, &id.stringify())
    }

    fn upsert_profile(&self, profile: &Profile) -> SdaServerResult<()> {
        self.profiles.save_with_id(profile, &profile.owner.stringify())?;
        Ok(())
    }

    fn get_profile(&self, owner: &AgentId) -> SdaServerResult<Option<Profile>> {
        super::get_option(&self.profiles, &owner.stringify())
    }

    fn create_encryption_key(&self, key: &SignedEncryptionKey) -> SdaServerResult<()> {
        self.encryption_keys.save_with_id(key, &key.id().stringify())?;
        Ok(())
    }

    fn get_encryption_key(&self,
                          key: &EncryptionKeyId)
                          -> SdaServerResult<Option<SignedEncryptionKey>> {
        super::get_option(&self.encryption_keys, &key.stringify())
    }

    fn suggest_committee(&self) -> SdaServerResult<Vec<ClerkCandidate>> {
        let keys = self.encryption_keys.all::<SignedEncryptionKey>()?;
        let candidates = keys.into_iter()
            .map(|(_, v)| v)
            .sorted_by(|a, b| a.signer.0.cmp(&b.signer.0))
            .into_iter()
            .group_by(|v| v.signer)
            .into_iter()
            .map(|(k, v)| {
                ClerkCandidate {
                    id: k,
                    keys: v.map(|sek| sek.body.id().clone()).collect()
                }
            })
            .collect();
        Ok(candidates)
    }
}
