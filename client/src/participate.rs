//! Specific functionality for participating in aggregations.

use SdaClient;
use crypto::*;
use trust::Policy;
use errors::SdaClientResult;

use sda_protocol::*;

pub struct ParticipantInput(pub Vec<i64>);

/// Basic tasks needed by a participant.
pub trait Participating {

    /// This will store relevant objects in cache to enable offline computation of `new_participation`.
    fn preload_for_participation(&mut self, aggregation: &AggregationId) -> SdaClientResult<()>;

    /// Create a new participation to the given aggregation.
    ///
    /// Having this as a seperate method allows background computation and retrying in case of network failure,
    /// without risk of recomputation and double participation.
    fn new_participation(&mut self, input: &ParticipantInput, aggregation: &AggregationId, enforce_trusted: bool) -> SdaClientResult<Participation>;

    /// Upload participation to the service.
    fn upload_participation(&self, input: &Participation) -> SdaClientResult<()>;

}

impl Participating for SdaClient
    // where
        // K: Store,
        // S: SdaAgentService,
        // S: SdaAggregationService,
        // S: SdaParticipationService,
{

    #[allow(unused_variables)]
    fn preload_for_participation(&mut self, aggregation_id: &AggregationId) -> SdaClientResult<()> {
        let aggregation = self.service.get_aggregation(&self.agent, aggregation_id)?.ok_or("Unknown aggregation")?;
        // recipient data
        let recipient = self.service.get_agent(&self.agent, &aggregation.recipient)?.ok_or("Unknown recipient")?;
        let recipient_key = self.service.get_encryption_key(&self.agent, &aggregation.recipient_key)?.ok_or("Unknown encryption key")?;
        // committee data
        let committee = self.service.get_committee(&self.agent, &aggregation.id)?.ok_or("Unknown committee")?;
        for &(ref clerk_id, ref key_id) in committee.clerks_and_keys.iter() {
            let _: Agent = self.service.get_agent(&self.agent, &clerk_id)?.ok_or("Unknown clerk")?;
            let _: SignedEncryptionKey = self.service.get_encryption_key(&self.agent, &key_id)?.ok_or("Unknown encryption key")?;
        }
        Ok(())
    }

    fn new_participation(&mut self, input: &ParticipantInput, aggregation_id: &AggregationId, require_trusted: bool) -> SdaClientResult<Participation> {

        let secrets = &input.0;

        // load aggregation
        let aggregation = self.service.get_aggregation(&self.agent, aggregation_id)?.ok_or("Could not find aggregation")?;
        if require_trusted && !self.trust.is_flagged_as_trusted(&aggregation.recipient)? {
            Err("Recipient is required to be trusted but is not")? 
        }
        if secrets.len() != aggregation.vector_dimension { 
            Err("The input length does not match the aggregation.")?
        }

        // load committee
        let committee: Committee = self.service.get_committee(&self.agent, aggregation_id)?.ok_or("Could not find committee")?;

        // encryptions for the participation; we'll fill this one up as we go along
        let mut encryptions: Vec<(AgentId, Encryption)> = vec!();

        // mask the secrets
        let mut secret_masker = self.crypto.new_secret_masker(&aggregation.masking_scheme)?;
        let (recipient_mask, committee_masked_secrets) = secret_masker.mask_secrets(secrets);

        // fetch and verify recipient's encryption key
        let recipient_id = &aggregation.recipient;
        let recipient_signed_encryption_key = self.service.get_encryption_key(&self.agent, &aggregation.recipient_key)?.ok_or("Unknown encryption key")?;
        let recipient = self.service.get_agent(&self.agent, recipient_id)?.ok_or("Unknown agent")?;
        if !recipient.signature_is_valid(&recipient_signed_encryption_key)? {
            Err("Signature verification failed for recipient key")?
        }
        let recipient_encryption_key = recipient_signed_encryption_key.body.body;
        // .. encrypt the recipient's mask using it
        let mask_encryptor = self.crypto.new_share_encryptor(&recipient_encryption_key, &aggregation.recipient_encryption_scheme)?;
        let recipient_encryption: Encryption = mask_encryptor.encrypt(&*recipient_mask)?;
        // .. and add result to collection
        encryptions.push((aggregation.recipient.clone(), recipient_encryption));

        // share the committee's masked secrets: each inner vector corresponds to the shares of a single clerk
        let mut share_generator = self.crypto.new_share_generator(&aggregation.committee_sharing_scheme)?;
        let committee_shares_per_clerk: Vec<Vec<Share>> = share_generator.generate_shares(&committee_masked_secrets);

        // encrypt the committee's shares
        for clerk_index in 0..committee_shares_per_clerk.len() {
            let clerk_shares = &committee_shares_per_clerk[clerk_index];
            let clerk_id = &committee.clerks_and_keys[clerk_index].0;

            // fetch and verify clerk's encryption key
            let clerk_signed_encryption_key_id = committee.clerks_and_keys[clerk_index].1;
            let clerk_signed_encryption_key = self.service.get_encryption_key(&self.agent, &clerk_signed_encryption_key_id)?.ok_or("Unknown encryption key")?;
            let clerk = self.service.get_agent(&self.agent, clerk_id)?.ok_or("Unknown clerk")?;
            if !clerk.signature_is_valid(&clerk_signed_encryption_key)? {
                Err("Signature verification failed for clerk key")?
            }
            let clerk_encryption_key = clerk_signed_encryption_key.body.body;
            // .. encrypt the clerk's shares using it
            let share_encryptor = self.crypto.new_share_encryptor(&clerk_encryption_key, &aggregation.committee_encryption_scheme)?;
            let clerk_encryption: Encryption = share_encryptor.encrypt(&*clerk_shares)?;
            // .. and add result to collection
            encryptions.push((clerk_id.clone(), clerk_encryption));
        }

        // generate fresh id for this participation
        let participation_id = ParticipationId::new();

        Ok(Participation {
            id: participation_id,
            participant: self.agent.id.clone(),
            aggregation: aggregation.id.clone(),
            encryptions: encryptions,
        })
    }

    fn upload_participation(&self, input: &Participation) -> SdaClientResult<()> {
        Ok(self.service.create_participation(&self.agent, input)?)
    }

}
