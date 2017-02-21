
use super::*;

pub trait Discover {

    fn list_aggregations(&self, filter: &str) -> SdaClientResult<Vec<AggregationId>>;

    fn pull_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<Option<Aggregation>>;

    fn pull_committee(&self, committee: &CommitteeId) -> SdaClientResult<Option<Committee>>;

    fn pull_keyset(&self, keyset: &KeysetId) -> SdaClientResult<Option<Keyset>>;

}

impl<T, S> Discover for SdaClient<T, S>
    where 
        S: SdaDiscoveryService,
        T: TrustStore,
{

    fn list_aggregations(&self, filter: &str) -> SdaClientResult<Vec<AggregationId>> {
        Ok(self.sda_service.list_aggregations(&self.agent, filter)?)
    }

    fn pull_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<Option<Aggregation>> {
        Ok(self.sda_service.pull_aggregation(&self.agent, aggregation)?)
    }

    fn pull_committee(&self, committee: &CommitteeId) -> SdaClientResult<Option<Committee>> {
        Ok(self.sda_service.pull_committee(&self.agent, committee)?)
    }

    fn pull_keyset(&self, keyset: &KeysetId) -> SdaClientResult<Option<Keyset>> {
        Ok(self.sda_service.pull_keyset(&self.agent, keyset)?)
    }

}

// pub trait Operations {

//     fn get_job(&self) -> SdaClientResult<Option<ClerkingJob>>;

//     fn process_job(&self, job: AsRef<ClerkingJob>) -> SdaClientResult<ClerkingResult>;

//     fn post_result(&self, result: AsRef<ClerkingResult>) -> SdaClientResult<()>;

// }