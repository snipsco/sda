use sda_protocol::*;

use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use std::sync::Arc;

pub fn new_agent(keystore: Arc<Keystore>) -> SdaClientResult<Agent> {
    let crypto = CryptoModule::new(keystore);
    Ok(Agent {
        id: AgentId::new(),
        verification_key: crypto.new_key()?,
    })
}

pub trait Maintenance {

    fn upload_agent(&self) -> SdaClientResult<()>;

    fn new_encryption_key(&self) -> SdaClientResult<EncryptionKeyId>;

    fn upload_encryption_key(&self, key: &EncryptionKeyId) -> SdaClientResult<()>;

    // fn new_profile(&mut self) -> SdaClientResult<Profile>;

    // fn upload_profile(&mut self, profile: &Profile) -> SdaClientResult<Profile>;

    // /// Upload a fresh encryption key to the service, available for future aggregations.
    // fn refresh_encryption_keys(&self) -> SdaClientResult<()>;

}

impl Maintenance for SdaClient
    // where
        // K: Store,
        // S: SdaAgentService,
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




// impl<L, I, S> IdentityManagement for SdaClient<L, I, S>
//     where 
//         S: SdaIdentityService,
//         I: IdentityStore,
// {

//     fn create_profile(&mut self) -> Profile {
       
//        // create an empty profile
//        let mut profile = Profile::default();

//        // create fresh signature keypair and save it in trust IdentityStore
//        // -- this will be the main identity of the profile
//        // TODO generation and other sensitive operations could be moved entirely to store
//        let (vk, sk) = sodiumoxide::crypto::sign::gen_keypair();
//        let wrapped_vk = VerificationKey::Sodium {key:vk.0.to_vec()}; // TODO avoid copying
//        let wrapped_sk = SigningKey::Sodium {key:sk.0.to_vec()}; // TODO avoid copying
//        self.trust_store.save_signature_keypair(&wrapped_vk, &wrapped_sk);
//        profile.verification_key = wrapped_vk;

//        debug_assert!(profile.owner == AgentId::default());
//        debug_assert!(profile.verification_key != VerificationKey::default());

//        Ok(profile)
//     }

//     fn upload_profile(&mut self, profile: &Profile) -> SdaClientResult<Profile> {
//         Ok(self.sda_service.update_profile(&self.agent, profile)?)
//     }

// }

// impl<L,I,S> SdaClient<L,I,S> {

//     fn register_new_encryption_key(&mut self, scheme: AdditiveEncryptionScheme) -> SdaClientResult<()> {
        
//         //
//         let (pk, sk) = sodiumoxide::crypto::box_::gen_keypair();

//     }

// }