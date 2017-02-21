
use super::*;

pub trait Discover {

    fn list_aggregations(&self, filter: &str) -> SdaClientResult<Vec<AggregationId>>;

    fn pull_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<Option<Aggregation>>;

    fn pull_committee(&self, committee: &CommitteeId) -> SdaClientResult<Option<Committee>>;

    fn pull_keyset(&self, keyset: &KeysetId) -> SdaClientResult<Option<Keyset>>;

}

impl<L, I, S> Discover for SdaClient<L, I, S>
    where 
        S: SdaDiscoveryService
{

    fn list_aggregations(&self, filter: &str) -> SdaClientResult<Vec<AggregationId>> {
        Ok(self.sda_service.list_aggregations_by_title(&self.agent, filter)?)
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
