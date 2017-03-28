//! Specific functionality for participating in aggregations.

use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use sda_protocol::*;

pub struct ParticipantInput(pub Vec<i64>);

/// Basic tasks needed by a participant.
pub trait Participating {

    /// Create a new participation to the given aggregation.
    ///
    /// Having this as a seperate method allows background computation and retrying in case of network failure,
    /// without risk of recomputation and double participation.
    fn new_participation(&self, input: &ParticipantInput, aggregation: &AggregationId) -> SdaClientResult<Participation>;

    /// Upload participation to the service.
    fn upload_participation(&self, input: &Participation) -> SdaClientResult<()>;

    fn participate(&self, input: Vec<i64>, aggregation: &AggregationId) -> SdaClientResult<()>;

}

impl Participating for SdaClient {

    fn participate(&self, input: Vec<i64>, aggregation: &AggregationId) -> SdaClientResult<()> {
        let input = ParticipantInput(input);
        let participation = self.new_participation(&input, &aggregation)?;
        self.upload_participation(&participation)
    }

    fn new_participation(&self, input: &ParticipantInput, aggregation_id: &AggregationId) -> SdaClientResult<Participation> {

        let secrets = &input.0;

        // load aggregation
        let aggregation = self.service.get_aggregation(&self.agent, aggregation_id)?
            .ok_or("Could not find aggregation")?;
        if secrets.len() != aggregation.vector_dimension {
            Err("The input length does not match the aggregation.")?
        }

        // load committee
        let committee: Committee = self.service.get_committee(&self.agent, aggregation_id)?
            .ok_or("Could not find committee")?;
        
        // mask the secrets
        let mut secret_masker = self.crypto.new_secret_masker(&aggregation.masking_scheme)?;
        let (recipient_mask, committee_masked_secrets) = secret_masker.mask(secrets);

        let recipient_encryption: Option<Encryption> = if recipient_mask.len() == 0 {
            None
        } else {
            // fetch and verify recipient's encryption key
            let recipient_id = &aggregation.recipient;
            let recipient_signed_encryption_key = self.service.get_encryption_key(&self.agent, &aggregation.recipient_key)?
                .ok_or("Unknown recipient encryption key")?;
            let recipient = self.service.get_agent(&self.agent, recipient_id)?
                .ok_or("Unknown recipient")?;
            if !recipient.signature_is_valid(&recipient_signed_encryption_key)? {
                Err("Signature verification failed for recipient key")?
            }
            let recipient_encryption_key = recipient_signed_encryption_key.body.body;
            // .. encrypt the recipient's mask using it
            let mask_encryptor = self.crypto.new_share_encryptor(&recipient_encryption_key, &aggregation.recipient_encryption_scheme)?;
            Some(mask_encryptor.encrypt(&*recipient_mask)?)
        };

        // share the committee's masked secrets: each inner vector corresponds to the shares of a single clerk
        let mut share_generator = self.crypto.new_share_generator(&aggregation.committee_sharing_scheme)?;
        let committee_shares_per_clerk: Vec<Vec<Share>> = share_generator.generate(&committee_masked_secrets);

        // encryptions for the participation; we'll fill this one up as we go along
        let mut clerk_encryptions: Vec<(AgentId, Encryption)> = vec![];

        // encrypt the committee's shares
        for clerk_index in 0..committee_shares_per_clerk.len() {
            let clerk_shares = &committee_shares_per_clerk[clerk_index];
            let clerk_id = &committee.clerks_and_keys[clerk_index].0;

            // fetch and verify clerk's encryption key
            let clerk_signed_encryption_key_id = committee.clerks_and_keys[clerk_index].1;
            let clerk_signed_encryption_key = self.service.get_encryption_key(&self.agent, &clerk_signed_encryption_key_id)?
                .ok_or("Unknown clerk encryption key")?;
            let clerk = self.service.get_agent(&self.agent, clerk_id)?
                .ok_or("Unknown clerk")?;
            if !clerk.signature_is_valid(&clerk_signed_encryption_key)? {
                Err("Signature verification failed for clerk key")?
            }
            let clerk_encryption_key = clerk_signed_encryption_key.body.body;
            // .. encrypt the clerk's shares using it
            let share_encryptor = self.crypto.new_share_encryptor(&clerk_encryption_key, &aggregation.committee_encryption_scheme)?;
            let clerk_encryption: Encryption = share_encryptor.encrypt(&*clerk_shares)?;
            // .. and add result to collection
            clerk_encryptions.push((clerk_id.clone(), clerk_encryption));
        }

        // generate fresh id for this participation
        let participation_id = ParticipationId::random();

        Ok(Participation {
            id: participation_id,
            participant: self.agent.id.clone(),
            aggregation: aggregation.id.clone(),
            recipient_encryption: recipient_encryption,
            clerk_encryptions: clerk_encryptions,
        })
    }

    fn upload_participation(&self, input: &Participation) -> SdaClientResult<()> {
        Ok(self.service.create_participation(&self.agent, input)?)
    }

}
