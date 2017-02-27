use std::path;

use jfs;
use uuid;

use sda_protocol::{Agent, AgentId};

use SdaServerResult;
use stores::{BaseStore, AgentStore};

pub struct JfsAgentStore(pub jfs::Store);

impl JfsAgentStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsAgentStore> {
        let store = prefix.as_ref().join("agents");
        Ok(JfsAgentStore(jfs::Store::new(store.to_str().ok_or("pathbuf to string")?)?))
    }

    fn id_as_str(uuid: &uuid::Uuid) -> String {
        format!("{}", uuid.simple())
    }
}

impl BaseStore for JfsAgentStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl AgentStore for JfsAgentStore {
    fn create_agent(&self, agent: &Agent) -> SdaServerResult<()> {
        self.0.save_with_id(agent, &Self::id_as_str(&agent.id.0))?;
        Ok(())
    }

    fn get_agent(&self, id: &AgentId) -> SdaServerResult<Option<Agent>> {
        match self.0.get(&Self::id_as_str(&id.0)) {
            Ok(it) => Ok(Some(it)),
            Err(io) => {
                if io.kind() == ::std::io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(io)?
                }
            }
        }
    }
}
