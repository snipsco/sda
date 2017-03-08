//! Specific functionality for clerking.

use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use sda_protocol::*;

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


impl Clerking for SdaClient
    // where
    //     K: Keystore,
        // S: SdaAgentService,
        // S: SdaAggregationService,
        // S: SdaClerkingService,
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

impl SdaClient
    // where
    //     K: Keystore,
        // S: SdaAgentService,
        // S: SdaAggregationService,
{

    fn process_clerking_job(&mut self, job: &ClerkingJob) -> SdaClientResult<ClerkingResult> {

        let aggregation = self.service.get_aggregation(&self.agent, &job.aggregation)?.ok_or("Unknown aggregation")?;
        let committee = self.service.get_committee(&self.agent, &job.aggregation)?.ok_or("Unknown committee")?;
        
        // TODO what is the right policy for whether we want to help with this aggregation or not?
        //  - based on aggregation and recipient?

        // TODO there is some waste in the following split between decrypting and combining 
        //  - this could be improved by e.g. allowing an accumulating combiner

        // determine which one of our encryption keys were used (in turn giving the decryption key we need to use)
        let own_signed_encryption_key_id = committee.clerk_keys.get(&self.agent.id)
            .ok_or("Could not find own encryption key in keyset")?;

        // decrypt shares from participants
        let share_decryptor = self.crypto.new_share_decryptor(&own_signed_encryption_key_id, &aggregation.committee_encryption_scheme)?;
        let partially_combined_shares = job.encryptions.iter()
            .map(|encryption| Ok(share_decryptor.decrypt(encryption)?))
            .collect::<SdaClientResult<Vec<Vec<Share>>>>()?;

        // sum up shares
        let share_combiner = self.crypto.new_share_combiner(&aggregation.committee_sharing_scheme)?;
        let fully_combined_shares: Vec<Share> = share_combiner.combine(&partially_combined_shares);

        // fetch recipient's encryption key and verify signature
        let recipient_id = &aggregation.recipient;
        let recipient_signed_encryption_key = self.service.get_encryption_key(&self.agent, &aggregation.recipient_key)?.ok_or("Unknown encryption key")?;
        let recipient = self.service.get_agent(&self.agent, recipient_id)?.ok_or("Unknown recipient")?;
        if !recipient.signature_is_valid(&recipient_signed_encryption_key)? {
            Err("Signature verification failed for recipient key")?
        }
        let recipient_encryption_key = recipient_signed_encryption_key.body.body;
        // .. and re-encrypt summed shares
        let share_encryptor = self.crypto.new_share_encryptor(&recipient_encryption_key, &aggregation.recipient_encryption_scheme)?;
        let recipient_encryption: Encryption = share_encryptor.encrypt(&fully_combined_shares)?;
        
        Ok(ClerkingResult {
            job: job.id.clone(),
            aggregation: job.aggregation.clone(),
            encryption: recipient_encryption,
        })
    }

}