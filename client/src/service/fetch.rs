use sda_protocol::*;

use SdaClient;
use errors::SdaClientResult;

/// Basic fetching.
pub trait Fetch<ID, O> {
    fn fetch(&self, id: &ID) -> SdaClientResult<O>;
}

// TODO these should probably all return options instead of errors in case the resource wasn't found!


impl<K, C, S> Fetch<AggregationId, Aggregation> for SdaClient<K, C, S>
    where S: SdaAggregationService
{
    fn fetch(&self, id: &AggregationId) -> SdaClientResult<Aggregation> {
        Ok(self.service.get_aggregation(&self.agent, id)?
            .ok_or("Aggregation not found on service")?)
    }
}

impl<K, C, S> Fetch<AggregationId, Committee> for SdaClient<K, C, S>
    where S: SdaAggregationService
{
    fn fetch(&self, id: &AggregationId) -> SdaClientResult<Committee> {
        Ok(self.service.get_committee(&self.agent, id)?
            .ok_or("Committee not found on service")?)
    }
}

impl<K, C, S> Fetch<AgentId, Agent> for SdaClient<K, C, S>
    where S: SdaAgentService
{
    fn fetch(&self, id: &AgentId) -> SdaClientResult<Agent> {
        Ok(self.service.get_agent(&self.agent, id)?
            .ok_or("Agent not found on service")?)
    }
}

impl<K, C, S> Fetch<EncryptionKeyId, SignedEncryptionKey> for SdaClient<K, C, S>
    where S: SdaAgentService
{
    fn fetch(&self, id: &EncryptionKeyId) -> SdaClientResult<SignedEncryptionKey> {
        Ok(self.service.get_encryption_key(&self.agent, id)?
            .ok_or("Encryption key not found on service")?)
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
//
// fetch!(id, AggregationId, Aggregation, SdaDiscoveryService, self.sda_service.pull_aggregation(&self.agent, id), "Not found");