
//! Specific functionality for participating in aggregations.

use std::collections::HashMap;

use super::*;


pub trait Participate {

    fn quickjoin_aggregation(&mut self, aggregation: &AggregationId) -> SdaClientResult<()>;

    /// Create a new participation to the given aggregation.
    ///
    /// Having this as a seperate method allows retrying in case of network failure without risk
    /// of recomputation and double participation.
    fn new_participation(&self, input: &ParticipantInput, aggregation: &AggregationId) -> SdaClientResult<Participation>;

    /// Upload participation to the service.
    fn upload_participation(&self, input: &Participation) -> SdaClientResult<()>;

}

impl<L, I, S> Participate for SdaClient<L, I, S>
    where
        L: Store<AggregationId, Aggregation>,
        L: Store<CommitteeId, Committee>,
        L: Store<KeysetId, Keyset>,
        L: Store<AgentId, Profile>,
        S: SdaDiscoveryService,
        S: SdaParticipationService,
{

    fn quickjoin_aggregation(&mut self, aggregation_id: &AggregationId) -> SdaClientResult<()> {

        // fetch objects of interest and save if successful
        let aggregation = self.fetch_and_save(aggregation_id)?;
        let committee = self.fetch_and_save(&aggregation.committee)?;
        let keyset = self.fetch_and_save(&aggregation.keyset)?;

        // make sure keyset checks out with recipient and clerks
        if keyset.keys.len() != 1 + committee.clerks.len() { 
            Err("Sizes of keyset and committee do not match")?
        }
        if !keyset.keys.contains_key(&aggregation.recipient) { 
            Err("Keyset is missing key for recipient")?
        }
        if !committee.clerks.iter().all(|clerk_id| { keyset.keys.contains_key(clerk_id) }) {
            Err("Keyset is missing key for some clerks")?
        }

        // make sure the signatures on the encryption keys in keyset check out
        self.verify_keys_in_keyset(&keyset)?;

        // if we made it this far we flag as trusted
        self.flag_as_trusted(&aggregation.id)?;
        self.flag_as_trusted(&committee.id)?;
        self.flag_as_trusted(&keyset.id)?;

        // ready for participation
        Ok(())
    }

    fn new_participation(&self, input: &ParticipantInput, aggregation: &AggregationId) -> SdaClientResult<Participation> {

        // TODO:
        // - clock timings

        // load objects of interest
        let aggregation = self.local_store.load(aggregation)?;
        let committee = self.local_store.load(&aggregation.committee)?;
        let keyset = self.local_store.load(&aggregation.keyset)?;

        let secrets = &input.0;

        // make sure the dimension of the input match the aggregation
        if secrets.len() != aggregation.vector_dimension { 
            Err("The input length does not match the aggregation.")?
        }

        // encryptions for the participation; we'll fill this one up as we go along
        let mut encryptions: HashMap<AgentId, Vec<Encryption>> = HashMap::new();

        // mask the secrets
        let mut secret_masker = aggregation.masking_scheme.new_secret_masker()?;
        let (recipient_mask, committee_masked_secrets) = secret_masker.mask_secrets(secrets);

        // encrypt the recipient's mask
        let recipient_encryption_key = keyset.keys.get(&aggregation.recipient)
            .ok_or("Could not find encryption key for recipient")?;
        let mask_encryptor = aggregation.recipient_encryption_scheme.new_share_encryptor(&recipient_encryption_key.key)?;
        let recipient_encryptions = mask_encryptor.encrypt(&*recipient_mask);
        // .. and add to collection
        encryptions.insert(aggregation.recipient.clone(), recipient_encryptions);

        // share the committee's masked secrets: each inner vector corresponds to the shares of a single clerk
        let mut share_generator = aggregation.committee_sharing_scheme.new_share_generator()?;
        let committee_shares_per_clerk: Vec<Vec<Share>> = share_generator.generate_shares(&committee_masked_secrets);

        // encrypt the committee's shares
        for clerk_index in 0..committee_shares_per_clerk.len() {
            let clerk_shares = &committee_shares_per_clerk[clerk_index];

            // resolve encryption key for clerk
            let clerk_id = &committee.clerks[clerk_index];
            let clerk_encryption_key = keyset.keys.get(&clerk_id)
                .ok_or("Could not find encryption key for clerk")?;

            // encrypt shares
            let share_encryptor = aggregation.committee_encryption_scheme.new_share_encryptor(&clerk_encryption_key.key)?;
            let clerk_encryptions = share_encryptor.encrypt(&*clerk_shares);
            // .. and add to collection
            encryptions.insert(clerk_id.clone(), clerk_encryptions);
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
        Ok(self.sda_service.push_participation(&self.agent, input)?)
    }

}

impl<L, I, S> SdaClient<L, I, S>
    where
        L: Store<AgentId, Profile>
 {

    fn verify_keys_in_keyset(&self, keyset: &Keyset) -> SdaClientResult<()> {

        for (owner, associated_key) in keyset.keys.iter() {

            // get corresponding profile and verification key
            let profile = self.local_store.load(&owner)?;
            let verification_key = profile.verification_key;

            // verify that the signature checks out for the encryption key
            let signature = &associated_key.signature;
            let encryption_key = &associated_key.key;
            // TODO
            if !self.trust_store.verify_signature(signature, &encryption_key.0, &verification_key)? {
                Err("Signature verification failed for encryption key")?
            }
        }

        Ok(())
    }

}