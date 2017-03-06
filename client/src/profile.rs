
use sda_protocol::*;
use sda_client_store::Store;

use SdaClient;
use errors::SdaClientResult;
use keystore::*;



// NOTE outside of SdaClient due to type instantiation error
pub fn load_agent<K: Store>(identitystore: &K) -> SdaClientResult<Option<Agent>> {
    let agent: Option<Agent> = identitystore.get_aliased("agent")?;
    Ok(agent)
}

pub fn new_agent<K: Store>(identitystore: &K) -> SdaClientResult<Agent> {
    let agent = Agent {
        id: AgentId::new(),
        verification_key: identitystore.new_key()?,
    };
    identitystore.put_aliased("agent", &agent)?;
    Ok(agent)
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

impl<K, C, S> Maintenance for SdaClient<K, C, S> 
    where
        S: SdaAgentService,
        K: KeyGeneration<EncryptionKeyId>,
        K: SignExport<EncryptionKeyId, Labeled<EncryptionKeyId, EncryptionKey>>,
{
    fn upload_agent(&self) -> SdaClientResult<()> {
        Ok(self.service.create_agent(&self.agent, &self.agent)?)
    }

    fn new_encryption_key(&self) -> SdaClientResult<EncryptionKeyId> {
        let key_id = self.keystore.new_key()?;
        Ok(key_id)
    }

    fn upload_encryption_key(&self, key: &EncryptionKeyId) -> SdaClientResult<()> {
        let signed_key = self.keystore.sign_export(&self.agent, key)?
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