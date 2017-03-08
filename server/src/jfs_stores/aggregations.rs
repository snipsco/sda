use jfs;

use std::path;

use sda_protocol::{Id, Identified};
use sda_protocol::{AgentId, Aggregation, AggregationId, Committee, Participation};

use SdaServerResult;

use stores::{BaseStore, AggregationsStore};

pub struct JfsAggregationsStore {
    participations: path::PathBuf,
    aggregations: jfs::Store,
    committees: jfs::Store,
}

impl JfsAggregationsStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsAggregationsStore> {
        let aggregations = prefix.as_ref().join("aggregations");
        let committees = prefix.as_ref().join("committees");
        Ok(JfsAggregationsStore {
            participations: prefix.as_ref().join("participations"),
            aggregations: jfs::Store::new(aggregations.to_str().ok_or("pathbuf to string")?)?,
            committees: jfs::Store::new(committees.to_str().ok_or("pathbuf to string")?)?,
        })
    }

    fn aggregation_store(&self, aggregation: &AggregationId) -> SdaServerResult<jfs::Store> {
        let path = self.participations.join(aggregation.stringify());
        Ok(jfs::Store::new(path.to_str().ok_or("path to string")?)?)
    }

}

impl BaseStore for JfsAggregationsStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl AggregationsStore for JfsAggregationsStore {
    fn list_aggregations(&self, filter: Option<&str>, recipient:Option<&AgentId>) -> SdaServerResult<Vec<AggregationId>> {
        Ok(self.aggregations
            .all::<Aggregation>()?
            .iter()
            .filter(|&(_, ref agg)|
                filter.map(|f| agg.title.contains(f)).unwrap_or(true)
                && recipient.map(|r| &agg.recipient == r).unwrap_or(true))
            .map(|(_,v)| v.id)
            .collect())
    }

    fn create_aggregation(&self, aggregation: &Aggregation) -> SdaServerResult<()> {
        self.aggregations.save_with_id(aggregation, &aggregation.id().stringify())?;
        Ok(())
    }

    fn get_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Aggregation>> {
        super::get_option(&self.aggregations, &*aggregation.stringify())
    }

    fn delete_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<()> {
        self.aggregations.delete(&aggregation.stringify())?;
        Ok(())
    }

    fn get_committee(&self, owner: &AggregationId) -> SdaServerResult<Option<Committee>> {
        super::get_option(&self.committees, &*owner.stringify())
    }

    fn create_committee(&self, committee: &Committee) -> SdaServerResult<()> {
        // FIXME: no overwriting
        self.committees.save_with_id(committee, &committee.aggregation.stringify())?;
        Ok(())
    }

    fn create_participation(&self, participation: &Participation) -> SdaServerResult<()> {
        let store = self.aggregation_store(&participation.aggregation)?;
        store.save_with_id(participation, &participation.id.stringify())?;
        Ok(())
    }

    fn iter_participations<'a, 'b>(&'b self, aggregation:&AggregationId)
         -> SdaServerResult<Box<Iterator<Item = SdaServerResult<Participation>> + 'a>>
        where 'b: 'a {
        let store = self.aggregation_store(aggregation)?;
        Ok(Box::new(store.all::<Participation>()?.into_iter().map(|p| Ok(p.1))))
    }

}
