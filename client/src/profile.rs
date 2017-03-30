use sda_protocol::*;

use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use std::sync::Arc;

impl SdaClient {
    /// Create a new agent, including mandatory signature keypair.
    pub fn new_agent(keystore: Arc<Keystore>) -> SdaClientResult<Agent> {
        let crypto = CryptoModule::new(keystore);
        Ok(Agent {
            id: AgentId::random(),
            verification_key: crypto.new_key()?,
        })
    }
}

/// Basic tasks typically performed with respect to agent associated resources.
pub trait Maintenance {

    /// Upload agent to service.
    fn upload_agent(&self) -> SdaClientResult<()>;

    /// Create new encryption key in keystore.
    fn new_encryption_key(&self) -> SdaClientResult<EncryptionKeyId>;

    /// Upload encryption key to service.
    fn upload_encryption_key(&self, key: &EncryptionKeyId) -> SdaClientResult<()>;

}

impl Maintenance for SdaClient
{
    fn upload_agent(&self) -> SdaClientResult<()> {
        Ok(self.service.create_agent(&self.agent, &self.agent)?)
    }

    fn new_encryption_key(&self) -> SdaClientResult<EncryptionKeyId> {
        let key_id = self.crypto.new_key()?;
        Ok(key_id)
    }

    fn upload_encryption_key(&self, key: &EncryptionKeyId) -> SdaClientResult<()> {
        let signed_key = self.crypto.sign_export(&self.agent, key)?
            .ok_or("Could not sign encryption key")?;
        Ok(self.service.create_encryption_key(&self.agent, &signed_key)?)
    }
}
