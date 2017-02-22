
use super::*;


pub trait Fetch<ID, O> {
    fn fetch(&self, id: &ID) -> SdaClientResult<O>;
}

pub trait CachedFetch<ID, O> {
    fn cached_fetch(&mut self, id: &ID) -> SdaClientResult<O>;
}

/// Generic implementation combining fetching and storage.
impl<ID, O, T> CachedFetch<ID, O> for T
    where
        T: Store<ID, O>,
        T: Fetch<ID, O>,
{
    fn cached_fetch(&mut self, id: &ID) -> SdaClientResult<O> {
        if self.has(id)? {
            self.load(id)
        } else {
            let obj = self.fetch(id)?;
            self.save(id, &obj)?;
            Ok(obj)
        }
    }
}


// macro_rules! fetch {
//     ( $i:ident, $it:ty, $ot:ty, $s:ty, $ep:expr, $err:expr ) => {

//         impl<L, I, S> Fetch<$it, $ot> for SdaClient<L, I, S>
//             where S: $s
//         {
//             fn fetch(&self, $i: &$it) -> SdaClientResult<$ot> {
//                 let service_result = $ep;
//                 Ok(service_result?.ok_or($err)?)
//             }
//         }

//     };
// }

// fetch!(id, AggregationId, Aggregation, SdaDiscoveryService, self.sda_service.pull_aggregation(&self.agent, id), "Not found");


impl<L, I, S> Fetch<AggregationId, Aggregation> for SdaClient<L, I, S>
    where S: SdaDiscoveryService
{
    fn fetch(&self, id: &AggregationId) -> SdaClientResult<Aggregation> {
        Ok(self.sda_service.pull_aggregation(&self.agent, id)?
            .ok_or("Aggregation not found on service")?)
    }
}

impl<L, I, S> Fetch<CommitteeId, Committee> for SdaClient<L, I, S>
    where S: SdaDiscoveryService
{
    fn fetch(&self, id: &CommitteeId) -> SdaClientResult<Committee> {
        Ok(self.sda_service.pull_committee(&self.agent, id)?
            .ok_or("Committee not found on service")?)
    }
}

impl<L, I, S> Fetch<KeysetId, Keyset> for SdaClient<L, I, S>
    where S: SdaDiscoveryService
{
    fn fetch(&self, id: &KeysetId) -> SdaClientResult<Keyset> {
        Ok(self.sda_service.pull_keyset(&self.agent, id)?
            .ok_or("Keyset not found on service")?)
    }
}

impl<L, I, S> Fetch<AgentId, Profile> for SdaClient<L, I, S>
    where S: SdaDiscoveryService
{
    fn fetch(&self, id: &AgentId) -> SdaClientResult<Profile> {
        Ok(self.sda_service.pull_profile(&self.agent, id)?
            .ok_or("Profile not found on service")?)
    }
}
