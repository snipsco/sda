
//! Specific functionality for clerking.

use super::*;

/// Basic tasks for a clerk.
pub trait Clerk {

    /// `force` means contacting the service even if the client believes its already registered.
    /// Return value indicates whether this was the first time the service saw this clerk.
    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool>;

    fn register_new_keypair(&mut self, scheme: AdditiveEncryptionScheme) -> SdaClientResult<()>;

    /// Execute clerking process once: download, process, and upload the next job pending on the service, if any.
    fn clerk_once(&mut self) -> SdaClientResult<bool>;

    /// Execute routine clerking chores, including registering if not done so already.
    fn run_chores(&mut self) -> SdaClientResult<()>;

}

impl<L,I,S> Clerk for SdaClient<L,I,S>
    where
        L: Store<AggregationId, Aggregation>,
        L: Store<KeysetId, Keyset>,
        L: Store<AgentId, Profile>,
        I: ExportDecryptionKey,
        S: SdaDiscoveryService,
        S: SdaClerkingService,
{

    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool> {
        // TODO
        unimplemented!()
    }

    fn register_new_keypair(&mut self, scheme: AdditiveEncryptionScheme) -> SdaClientResult<()> {

        let (pk, sk) = sodiumoxide::crypto::box_::gen_keypair();

    }

    fn clerk_once(&mut self) -> SdaClientResult<bool> {
        let job = self.sda_service.pull_clerking_job(&self.agent, &self.agent.id)?;
        match job {
            None => {
                Ok(false)
            },
            Some(job) => {
                let result = self.process_job(&job)?;
                self.sda_service.push_clerking_result(&self.agent, &result)?;
                Ok(true)
            }
        }
    }

    fn run_chores(&mut self) -> SdaClientResult<()> {
        // register if we haven't done so already
        self.register_as_clerk(false)?;
        // repeatedly process jobs
        let max_iterations = 10;
        for _ in 0..max_iterations {
            if self.clerk_once()? { 
                continue
            } else {
                break
            }
        }
        Ok(())
    }

}



impl<L,I,S> SdaClient<L,I,S>
    where
        L: Store<AggregationId, Aggregation>,
        L: Store<KeysetId, Keyset>,
        L: Store<AgentId, Profile>,
        I: ExportDecryptionKey,
        S: SdaDiscoveryService,
 {

    fn process_job(&mut self, job: &ClerkingJob) -> SdaClientResult<ClerkingResult> {

        let aggregation = self.cached_fetch(&job.aggregation)?;

        // TODO what is the right policy for whether we want to help with this aggregation or not?

        let keyset = self.cached_fetch(&aggregation.keyset)?;
        let recipient = self.cached_fetch(&aggregation.recipient)?;
        
        // determine which one of our encryption keys were used (in turn giving the decryption key we need to use)
        let own_encryption_key = &keyset.keys.get(&self.agent.id)
            .ok_or("Could not find own encryption key in keyset")?;

        // extract encryption key for recipient and verify signature
        let recipient_encryption_key = &keyset.keys.get(&recipient.owner)
            .ok_or("Could not find encryption key for recipient in keyset")?;
        if !recipient.signature_is_valid(&recipient_encryption_key)? {
            Err("Signature verification failed for recipient encryption key")?
        }

        // TODO there is some waste in the following split between decrypting and combining 
        //  - this could be improved by e.g. allowing an accumulating combiner

        // decrypt shares from participants
        let share_decryptor = aggregation.committee_encryption_scheme.new_share_decryptor(&own_encryption_key.key, &self.identity)?;
        let partially_combined_shares = job.encryptions.iter()
            .map(|encryption| Ok(share_decryptor.decrypt(encryption)?))
            .collect::<SdaClientResult<Vec<Vec<Share>>>>()?;

        // sum up shares
        let share_combiner = aggregation.committee_sharing_scheme.new_share_combiner()?;
        let fully_combined_shares: Vec<Share> = share_combiner.combine(&partially_combined_shares);

        // re-encrypt shares for recipient
        let share_encryptor = aggregation.recipient_encryption_scheme.new_share_encryptor(&recipient_encryption_key.key)?;
        let recipient_encryption: Encryption = share_encryptor.encrypt(&fully_combined_shares)?;
        
        Ok(ClerkingResult {
            clerk: self.agent.id.clone(),
            aggregation: job.aggregation.clone(),
            encryption: recipient_encryption,
        })
    }

}
