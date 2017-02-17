
use super::*;

pub trait SdaUser {

    /// TODO
    ///
    /// Having this as a seperate method allows retrying in case of failure without risk of double participation.
    fn create_participation(&self, input: &UserInput, aggregation: &Aggregation) -> SdaClientResult<Participation>;

    /// TODO
    fn participate(&self, input: &Participation) -> SdaClientResult<()>;

}

impl<T, S> SdaUser for SdaClient<T, S>
    where 
        S: UserSdaAggregationService,
        T: TrustedCommitteeStore,
        T: TrustedKeysetStore,
{

    fn participate(&self, input: &Participation) -> SdaClientResult<()> {
        Ok(self.sda_service.push_user_participation(&self.agent, input)?)
    }

    fn create_participation(&self, input: &UserInput, aggregation: &Aggregation) -> SdaClientResult<Participation> {

        // TODO:
        // - clock timings
        // - OTPs

        // make sure the dimension of the input match the aggregation
        if input.0.len() != aggregation.vector_dimension { 
            Err("The input length does not match the aggregation.")?
        }

        // generate shares: each inner vector corresponds to the shares of a single clerk
        let mut share_generator = aggregation.secret_sharing_scheme.new_share_generator()?;
        let shares_per_clerk: Vec<Vec<Share>> = share_generator.generate_shares(&input.0);

        // load associated committee
        let committee = self.trust_store.load_trusted_committee(&aggregation.committee)?;

        // load associated keyset
        let keyset = self.trust_store.load_trusted_keyset(&aggregation.keyset)?;

        // encrypt all shares
        let encryptions_per_clerk = shares_per_clerk.iter().enumerate()
            .map(|(clerk_index, clerk_shares)| {

                // resolve encryption key for clerk
                let clerk_id = &committee.clerks[clerk_index];
                let clerk_encryption_key = keyset.keys.get(&clerk_id)
                    .ok_or("Could not find encryption key for clerk")?;

                // encrypt shares
                let mut share_encryptor = aggregation.encryption_scheme.new_share_encryptor(&clerk_encryption_key.key)?;
                let clerk_encryptions = share_encryptor.encrypt(&*clerk_shares);

                Ok(clerk_encryptions)
            })
            .collect::<SdaClientResult<Vec<Vec<Encryption>>>>()?;  // to help type inference

        // generate fresh id for this participation
        let participation_id = ParticipationId(Uuid::new_v4());

        Ok(Participation {
            id: participation_id,
            user: self.agent.id.clone(),
            aggregation: aggregation.id.clone(),
            encryptions: encryptions_per_clerk,
        })
    }

}
