
//! Specific functionality for clerking.

use super::*;


/// Basic tasks needed by a clerk.
pub trait Clerking {

    /// `force` means contacting the service even if the client believes its already registered.
    /// Return value indicates whether this was the first time the service saw this clerk.
    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool>;

    /// Execute clerking process once: download, process, and upload the next job pending on the service, if any.
    fn clerk_once(&mut self) -> SdaClientResult<bool>;

    /// Execute routine clerking chores, including registering if not done so already.
    fn run_chores(&mut self) -> SdaClientResult<()>;

}


impl<K, C, S> Clerking for SdaClient<K, C, S>
    where
        K: ExportDecryptionKey<EncryptionKeyId, (EncryptionKey, DecryptionKey)>,
        C: Cache<AggregationId, Aggregation>,
        C: Cache<AggregationId, Committee>,
        C: Cache<EncryptionKeyId, SignedEncryptionKey>,
        C: Cache<AgentId, Agent>,
        S: SdaDiscoveryService,
        S: SdaClerkingService,
{

    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool> {
        // TODO
        unimplemented!()
    }

    fn clerk_once(&mut self) -> SdaClientResult<bool> {
        let job = self.service.get_clerking_job(&self.agent, &self.agent.id)?;
        match job {
            None => {
                Ok(false)
            },
            Some(job) => {
                let result = self.process_clerking_job(&job)?;
                self.service.create_clerking_result(&self.agent, &result)?;
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


impl<K, C, S> SdaClient<K, C, S>
    where
        K: ExportDecryptionKey<EncryptionKeyId, (EncryptionKey, DecryptionKey)>,
        C: Cache<AggregationId, Aggregation>,
        C: Cache<AggregationId, Committee>,
        C: Cache<AgentId, Agent>,
        C: Cache<EncryptionKeyId, SignedEncryptionKey>,
        S: SdaDiscoveryService,
{

    fn process_clerking_job(&mut self, job: &ClerkingJob) -> SdaClientResult<ClerkingResult> {

        let aggregation: Aggregation = self.cached_fetch(&job.aggregation)?;
        let committee: Committee = self.cached_fetch(&job.aggregation)?;
        
        // TODO what is the right policy for whether we want to help with this aggregation or not?
        //  - based on aggregation and recipient?

        // TODO there is some waste in the following split between decrypting and combining 
        //  - this could be improved by e.g. allowing an accumulating combiner

        // determine which one of our encryption keys were used (in turn giving the decryption key we need to use)
        let own_signed_encryption_key_id = committee.clerk_keys.get(&self.agent.id)
            .ok_or("Could not find own encryption key in keyset")?;

        // decrypt shares from participants
        let share_decryptor = aggregation.committee_encryption_scheme.new_share_decryptor(&own_signed_encryption_key_id, &self.keystore)?;
        let partially_combined_shares = job.encryptions.iter()
            .map(|encryption| Ok(share_decryptor.decrypt(encryption)?))
            .collect::<SdaClientResult<Vec<Vec<Share>>>>()?;

        // sum up shares
        let share_combiner = aggregation.committee_sharing_scheme.new_share_combiner()?;
        let fully_combined_shares: Vec<Share> = share_combiner.combine(&partially_combined_shares);

        // fetch recipient's encryption key and verify signature
        let recipient_id = &aggregation.recipient;
        let recipient_signed_encryption_key = self.cached_fetch(&aggregation.recipient_key)?;
        let recipient = self.cached_fetch(recipient_id)?;
        if !recipient.signature_is_valid(&recipient_signed_encryption_key)? {
            Err("Signature verification failed for recipient key")?
        }
        let recipient_encryption_key = recipient_signed_encryption_key.body.body;
        // .. and re-encrypt summed shares
        let share_encryptor = aggregation.recipient_encryption_scheme.new_share_encryptor(&recipient_encryption_key)?;
        let recipient_encryption: Encryption = share_encryptor.encrypt(&fully_combined_shares)?;
        
        Ok(ClerkingResult {
            job: job.id.clone(),
            aggregation: job.aggregation.clone(),
            encryption: recipient_encryption,
        })
    }

}
