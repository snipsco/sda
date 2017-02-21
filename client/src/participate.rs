
//! Specific functionality for participating in aggregations.

use super::*;

pub trait Participate {

    fn quickjoin_aggregation(&mut self, aggregation: &AggregationId) -> SdaClientResult<Aggregation>;

    /// Create a new participation to the given aggregation.
    ///
    /// Having this as a seperate method allows retrying in case of network failure without risk
    /// of recomputation and double participation.
    fn new_participation(&self, input: &ParticipantInput, aggregation: &Aggregation) -> SdaClientResult<Participation>;

    /// Upload participation to the service.
    fn upload_participation(&self, input: &Participation) -> SdaClientResult<()>;

}

impl<T, S> Participate for SdaClient<T, S>
    where 
        S: SdaDiscoveryService,
        S: SdaParticipationService,
        T: TrustStore,
{

    fn quickjoin_aggregation(&mut self, aggregation_id: &AggregationId) -> SdaClientResult<Aggregation> {

        // make sure we have objects of interest
        let aggregation = self.fetch_aggregation(aggregation_id)?;
        let committee = self.fetch_committee(&aggregation.committee)?;
        let keyset = self.fetch_keyset(&aggregation.keyset)?;

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

        // save if we made it this far
        self.trust_store.save_keyset(&keyset)?;
        self.trust_store.save_committee(&committee)?;

        // ready for participation
        Ok(aggregation)
    }

    fn new_participation(&self, input: &ParticipantInput, aggregation: &Aggregation) -> SdaClientResult<Participation> {

        // TODO:
        // - clock timings

        let secrets = &input.0;

        // make sure the dimension of the input match the aggregation
        if secrets.len() != aggregation.vector_dimension { 
            Err("The input length does not match the aggregation.")?
        }

        // load associated keyset
        let keyset = self.trust_store.load_keyset(&aggregation.keyset)?;

        // load associated committee
        let committee = self.trust_store.load_committee(&aggregation.committee)?;

        // encryptions for the participation; we'll fill this one up as we go along
        use std::collections::HashMap;
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

impl<T, S> SdaClient<T, S>
    where
        T: TrustStore,
        S: SdaDiscoveryService,
 {

    fn verify_keys_in_keyset(&self, keyset: &Keyset) -> SdaClientResult<()> {

        for (owner, associated_key) in keyset.keys.iter() {

            // make sure we have the verification key
            let profile = if self.trust_store.has_profile(&owner)? {
                self.trust_store.load_profile(&owner)?
            } else {
                self.sda_service.pull_profile(&self.agent, &owner)?
                    .ok_or("Profile not found on service")?
            };
            let verification_key = profile.verification_key;

            // verify that the signature checks out for the encryption key
            let signature = &associated_key.signature;
            let encryption_key = &associated_key.key;
            if !self.trust_store.verify_signature(signature, &encryption_key.0, &verification_key)? {
                Err("Signature verification failed for encryption key")?
            }
        }

        Ok(())
    }

    fn fetch_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<Aggregation> {
        // TODO better way of wrapping?
        Ok(self.sda_service.pull_aggregation(&self.agent, aggregation)?
            .ok_or("Aggregation not found on service")?)
    }

    fn fetch_committee(&self, committee: &CommitteeId) -> SdaClientResult<Committee> {
        if self.trust_store.has_committee(committee)? {
            self.trust_store.load_committee(committee)
        } else {
            // TODO better way of wrapping?
            Ok(self.sda_service.pull_committee(&self.agent, committee)?
                .ok_or("Committee not found on service")?)
        }
    }

    fn fetch_keyset(&self, keyset: &KeysetId) -> SdaClientResult<Keyset> {
        if self.trust_store.has_keyset(&keyset)? {
            self.trust_store.load_keyset(&keyset)
        } else {
            // TODO better way of wrapping?
            Ok(self.sda_service.pull_keyset(&self.agent, &keyset)?
                .ok_or("Keyset not found on service")?)
        }
    }

}