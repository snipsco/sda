
use super::*;

pub trait CommitteeStore {
    // fn store_trusted_committee(&self, committee: Committee) -> SdaClientResult<()>;
    fn load_trusted_committee(&self, committee: &CommitteeId) -> SdaClientResult<Committee>;
}

pub trait KeysetStore {
    // fn store_trusted_keyset(&self, keyset: Keyset) -> SdaClientResult<()>;
    fn load_trusted_keyset(&self, keyset: &KeysetId) -> SdaClientResult<Keyset>;
    // fn verify_keyset(&self, keyset: Keyset) -> SdaClientResult<bool>;
}

pub trait SdaUser {

    /// TODO
    ///
    /// Having this as a seperate method allows retrying in case of failure without risk of double participation.
    fn create_participation(&self, input: &UserInput, aggregation: &Aggregation) -> SdaClientResult<Participation>;

    /// TODO
    fn participate(&self, input: &Participation) -> SdaClientResult<()>;

}

impl<'l, S: 'l, K: 'l> CommitteeStore for SdaClient<'l, S, K> {
    fn load_trusted_committee(&self, committee: &CommitteeId) -> SdaClientResult<Committee> {
        unimplemented!(); // TODO
    }
}

impl<'l, S: 'l, K: 'l> KeysetStore for SdaClient<'l, S, K> {
    fn load_trusted_keyset(&self, keyset: &KeysetId) -> SdaClientResult<Keyset> {
        unimplemented!(); // TODO
    }
}

impl<'l, S: 'l, K: 'l> SdaUser for SdaClient<'l, S, K>
    where S: UserSdaAggregationService 
{

    fn participate(&self, input: &Participation) -> SdaClientResult<()> {
        Ok(self.sda_service.push_user_participation(self.agent, input)?)

//        let agent = try!(self.agent.lock().or(Err("Poisoned agent mutex")));
//        let mut timings = HashMap::new();
//        let foo: Option<Foo> = timer!(timings, "participate/get_package",
//            try!(self.service.)
//        )
    }

    fn create_participation(&self, input: &UserInput, aggregation: &Aggregation) -> SdaClientResult<Participation> {

        // TODO:
        // - clock timings

        // make sure the dimension of the input match the aggregation
        if input.0.len() != aggregation.vector_dimension { 
            Err("The input length does not match the aggregation.")?
        }

        // generate shares: each inner vector corresponds to the shares of a single clerk
        let share_generator = aggregation.secret_sharing_scheme.new_share_generator();
        let shares_per_clerk: Vec<Vec<Share>> = share_generator.generate_shares(&input.0);

        // load associated committee
        let committee = self.load_trusted_committee(&aggregation.committee)?;

        // load associated keyset
        let keyset = self.load_trusted_keyset(&aggregation.keyset)?;

        // encrypt all shares
        let encryptions_per_clerk = shares_per_clerk.iter().enumerate()
            .map(|(clerk_index, clerk_shares)| {

                // resolve encryption key for clerk
                let clerk_id = &committee.clerks[clerk_index];
                let clerk_encryption_key = keyset.keys.get(&clerk_id)
                    .ok_or("Could not find encryption key for clerk")?;

                // encrypt shares
                let share_encryptor = aggregation.encryption_scheme.new_encryptor(&clerk_encryption_key.key);
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
