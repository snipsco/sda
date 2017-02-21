
use super::*;

pub trait IdentityManagement {

    fn new_profile(&mut self) -> SdaClientResult<Profile>;

    fn upload_profile(&mut self, profile: &Profile) -> SdaClientResult<Profile>;

    /// Upload a fresh set of encryption keys to the service, available for future aggregations.
    fn refresh_encryption_keys(&self) -> SdaClientResult<()>;

}

impl<T, S> IdentityManagement for SdaClient<T, S>
    where 
        S: SdaIdentityService,
        T: IdentityStore,
{

    fn create_profile(&mut self) -> Profile {
       
       // create an empty profile
       let mut profile = Profile::default();

       // create fresh signature keypair and save it in trust IdentityStore
       // -- this will be the main identity of the profile
       // TODO generation and other sensitive operations could be moved entirely to store
       let (vk, sk) = sodiumoxide::crypto::sign::gen_keypair();
       let wrapped_vk = VerificationKey::Sodium {key:vk.0.to_vec()}; // TODO avoid copying
       let wrapped_sk = SigningKey::Sodium {key:sk.0.to_vec()}; // TODO avoid copying
       self.trust_store.save_signature_keypair(&wrapped_vk, &wrapped_sk);
       profile.verification_key = wrapped_vk;

       debug_assert!(profile.owner == AgentId::default());
       debug_assert!(profile.verification_key != VerificationKey::default());

       Ok(profile)
    }

    fn upload_profile(&mut self, profile: &Profile) -> SdaClientResult<Profile> {
        Ok(self.sda_service.update_profile(&self.agent, profile)?)
    }

}
