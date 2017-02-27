
//! Specific functionality for participating in aggregations.

use std::collections::HashMap;

use super::*;


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

impl<L,I,S> Participating for SdaClient<L,I,S>
    where
        L: Cache<AggregationId, Aggregation>,
        L: Cache<AggregationId, Committee>,
        L: Cache<SignedEncryptionKeyId, SignedEncryptionKey>,
        L: Cache<AgentId, Agent>,
        S: SdaDiscoveryService,
        S: SdaParticipationService,
{

    fn preload_for_participation(&mut self, aggregation_id: &AggregationId) -> SdaClientResult<()> {
        let aggregation: Aggregation = self.cached_fetch(aggregation_id)?;
        // recipient data
        let recipient = self.cached_fetch(&aggregation.recipient)?;
        let recipient_key = self.cached_fetch(&aggregation.recipient_key)?;
        // committee data
        let committee: Committee = self.cached_fetch(&aggregation.id)?;
        for (clerk_id, key_id) in committee.clerk_keys.iter() {
            let _: Agent = self.cached_fetch(clerk_id)?;
            let _: SignedEncryptionKey = self.cached_fetch(key_id)?;
        }
        Ok(())
    }

    fn new_participation(&mut self, input: &ParticipantInput, aggregation_id: &AggregationId, require_trusted: bool) -> SdaClientResult<Participation> {

        let secrets = &input.0;

        // load aggregation
        let aggregation: Aggregation = self.cached_fetch(aggregation_id)?;
        if require_trusted && !self.is_flagged_as_trusted(&aggregation.recipient)? {
            Err("Recipient is required to be trusted but is not")? 
        }
        if secrets.len() != aggregation.vector_dimension { 
            Err("The input length does not match the aggregation.")?
        }

        // load committee
        let committee: Committee = self.cached_fetch(aggregation_id)?;

        // encryptions for the participation; we'll fill this one up as we go along
        let mut encryptions: HashMap<AgentId, Encryption> = HashMap::new();

        // mask the secrets
        let mut secret_masker = aggregation.masking_scheme.new_secret_masker()?;
        let (recipient_mask, committee_masked_secrets) = secret_masker.mask_secrets(secrets);

        // fetch and verify recipient's encryption key
        let recipient_id = &aggregation.recipient;
        let recipient_signed_encryption_key = self.cached_fetch(&aggregation.recipient_key)?;
        let recipient = self.cached_fetch(recipient_id)?;
        if !recipient.signature_is_valid(&recipient_signed_encryption_key)? {
            Err("Signature verification failed for recipient key")?
        }
        let recipient_encryption_key = recipient_signed_encryption_key.key;
        // .. encrypt the recipient's mask using it
        let mask_encryptor = aggregation.recipient_encryption_scheme.new_share_encryptor(&recipient_encryption_key)?;
        let recipient_encryption: Encryption = mask_encryptor.encrypt(&*recipient_mask)?;
        // .. and add result to collection
        encryptions.insert(aggregation.recipient.clone(), recipient_encryption);

        // share the committee's masked secrets: each inner vector corresponds to the shares of a single clerk
        let mut share_generator = aggregation.committee_sharing_scheme.new_share_generator()?;
        let committee_shares_per_clerk: Vec<Vec<Share>> = share_generator.generate_shares(&committee_masked_secrets);

        // encrypt the committee's shares
        for clerk_index in 0..committee_shares_per_clerk.len() {
            let clerk_shares = &committee_shares_per_clerk[clerk_index];
            let clerk_id = &committee.clerk_order[clerk_index];

            // fetch and verify clerk's encryption key
            let clerk_signed_encryption_key_id = committee.clerk_keys.get(&clerk_id)
                .ok_or("Keyset missing encryption key for clerk")?;
            let clerk_signed_encryption_key = self.cached_fetch(clerk_signed_encryption_key_id)?;
            let clerk = self.cached_fetch(clerk_id)?;
            if !clerk.signature_is_valid(&clerk_signed_encryption_key)? {
                Err("Signature verification failed for clerk key")?
            }
            let clerk_encryption_key = clerk_signed_encryption_key.key;
            // .. encrypt the clerk's shares using it
            let share_encryptor = aggregation.committee_encryption_scheme.new_share_encryptor(&clerk_encryption_key)?;
            let clerk_encryption: Encryption = share_encryptor.encrypt(&*clerk_shares)?;
            // .. and add result to collection
            encryptions.insert(clerk_id.clone(), clerk_encryption);
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
        Ok(self.sda_service.create_participation(&self.agent, input)?)
    }

}
