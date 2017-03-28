//! Specific functionality for clerking.

use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use sda_protocol::*;

/// Basic tasks needed by a clerk.
pub trait Clerking {

    /// Execute clerking process once: download, process, and upload the next job pending on the service, if any.
    fn clerk_once(&self) -> SdaClientResult<bool>;

    /// Execute routine clerking chores, including registering if not done so already.
    ///
    /// Note that a negative `max_iterations` will continue the clerking process until there are no more jobs.
    fn run_chores(&self, max_iterations: isize) -> SdaClientResult<()>;

}

impl Clerking for SdaClient {

    fn clerk_once(&self) -> SdaClientResult<bool> {
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

    fn run_chores(&self, max_iterations: isize) -> SdaClientResult<()> {
        // repeatedly process jobs
        if max_iterations < 0 {
            // loop until there's no more work
            loop {
                if !self.clerk_once()? {
                    break
                }
            }
        } else {
            // loop a maximum number of times
            for _ in 0..max_iterations {
                if !self.clerk_once()? {
                    break
                }
            }
        }
        return Ok(())
    }

}

impl SdaClient {

    fn process_clerking_job(&self, job: &ClerkingJob) -> SdaClientResult<ClerkingResult> {

        let aggregation = self.service.get_aggregation(&self.agent, &job.aggregation)?
            .ok_or("Unknown aggregation")?;
        
        let committee = self.service.get_committee(&self.agent, &job.aggregation)?
            .ok_or("Unknown committee")?;

        // FIXME there is some waste in the following split between decrypting and combining
        //  - this could be improved by e.g. allowing an accumulating combiner

        // determine which one of our encryption keys were used (in turn giving the decryption key we need to use)
        let own_signed_encryption_key_id = committee.clerks_and_keys.iter().find(|&&(id,_)| id == self.agent.id)
            .ok_or("Could not find own encryption key in keyset")?.1;

        // decrypt shares from participants
        let share_decryptor = self.crypto.new_share_decryptor(&own_signed_encryption_key_id, &aggregation.committee_encryption_scheme)?;
        let partially_combined_shares = job.encryptions.iter()
            .map(|encryption| Ok(share_decryptor.decrypt(encryption)?))
            .collect::<SdaClientResult<Vec<Vec<Share>>>>()?;

        // sum up shares
        let share_combiner = self.crypto.new_share_combiner(&aggregation.committee_sharing_scheme)?;
        let fully_combined_shares: Vec<Share> = share_combiner.combine(&partially_combined_shares)?;

        // fetch recipient's encryption key and verify signature
        let recipient_id = &aggregation.recipient;
        let recipient = self.service.get_agent(&self.agent, recipient_id)?
            .ok_or("Unknown recipient")?;
        let recipient_signed_encryption_key = self.service.get_encryption_key(&self.agent, &aggregation.recipient_key)?
            .ok_or("Unknown recipient encryption key")?;
        if !recipient.signature_is_valid(&recipient_signed_encryption_key)? {
            Err("Signature verification failed for recipient key")?
        }
        let recipient_encryption_key = recipient_signed_encryption_key.body.body;
        // .. and re-encrypt summed shares
        let share_encryptor = self.crypto.new_share_encryptor(&recipient_encryption_key, &aggregation.recipient_encryption_scheme)?;
        let recipient_encryption: Encryption = share_encryptor.encrypt(&fully_combined_shares)?;

        Ok(ClerkingResult {
            job: job.id.clone(),
            clerk: job.clerk,
            encryption: recipient_encryption,
        })
    }

}
