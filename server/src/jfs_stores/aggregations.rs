use jfs;

use std::path;
use std::str::FromStr;

use sda_protocol::{AgentId, Aggregation, AggregationId, Committee, Encryption, Participation,
                   ParticipationId, Snapshot, SnapshotId};

use SdaServerResult;
use ::jfs_stores::JfsStoreExt;

use stores::{BaseStore, AggregationsStore};

#[derive(Debug, Serialize, Deserialize)]
struct SnapshotContent {
    participations: Vec<ParticipationId>,
}

pub struct JfsAggregationsStore {
    participations: path::PathBuf,
    aggregations: jfs::Store,
    committees: jfs::Store,
    snapshots: jfs::Store,
    snapshot_contents: jfs::Store,
    snapshot_masks: jfs::Store,
}

impl JfsAggregationsStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsAggregationsStore> {
        let aggregations = prefix.as_ref().join("aggregations");
        let committees = prefix.as_ref().join("committees");
        let snapshots = prefix.as_ref().join("snapshots");
        let snapshot_contents = prefix.as_ref().join("snapshot_contents");
        let snapshot_masks = prefix.as_ref().join("snapshot_masks");
        Ok(JfsAggregationsStore {
            participations: prefix.as_ref().join("participations"),
            aggregations: jfs::Store::new(aggregations.to_str().ok_or("pathbuf to string")?)?,
            committees: jfs::Store::new(committees.to_str().ok_or("pathbuf to string")?)?,
            snapshots: jfs::Store::new(snapshots.to_str().ok_or("pathbuf to string")?)?,
            snapshot_contents: jfs::Store::new(snapshot_contents.to_str()
                .ok_or("pathbuf to string")?)?,
            snapshot_masks: jfs::Store::new(snapshot_masks.to_str()
                .ok_or("pathbuf to string")?)?,
        })
    }

    fn aggregation_store(&self, aggregation: &AggregationId) -> SdaServerResult<jfs::Store> {
        let path = self.participations.join(aggregation.to_string());
        Ok(jfs::Store::new(path.to_str().ok_or("path to string")?)?)
    }
}

impl BaseStore for JfsAggregationsStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl AggregationsStore for JfsAggregationsStore {
    fn list_aggregations(&self,
                         filter: Option<&str>,
                         recipient: Option<&AgentId>)
                         -> SdaServerResult<Vec<AggregationId>> {
        Ok(self.aggregations
            .all::<Aggregation>()?
            .iter()
            .filter(|&(_, ref agg)| {
                filter.map(|f| agg.title.contains(f)).unwrap_or(true) &&
                recipient.map(|r| &agg.recipient == r).unwrap_or(true)
            })
            .map(|(_, v)| v.id)
            .collect())
    }

    fn create_aggregation(&self, aggregation: &Aggregation) -> SdaServerResult<()> {
        self.aggregations.save_ident(aggregation)
    }

    fn get_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Aggregation>> {
        self.aggregations.get_option(aggregation)
    }

    fn delete_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<()> {
        self.aggregations.delete(&aggregation.to_string())?;
        Ok(())
    }

    fn get_committee(&self, owner: &AggregationId) -> SdaServerResult<Option<Committee>> {
        self.committees.get_option(owner)
    }

    fn create_committee(&self, committee: &Committee) -> SdaServerResult<()> {
        // FIXME: no overwriting
        self.committees.save_at(committee, &committee.aggregation)
    }

    fn create_participation(&self, participation: &Participation) -> SdaServerResult<()> {
        let store = self.aggregation_store(&participation.aggregation)?;
        store.save_ident(participation)
    }

    fn create_snapshot(&self, snapshot: &Snapshot) -> SdaServerResult<()> {
        self.snapshots.save_ident(snapshot)
    }

    fn count_participations(&self, aggregation: &AggregationId) -> SdaServerResult<usize> {
        let store = self.aggregation_store(aggregation)?;
        Ok(store.all::<Participation>()?.len())
    }

    fn snapshot_participations(&self,
                               aggregation: &AggregationId,
                               snapshot: &SnapshotId)
                               -> SdaServerResult<()> {
        let store = self.aggregation_store(aggregation)?;
        let list: SdaServerResult<Vec<ParticipationId>> = store.all::<Participation>()?
            .into_iter()
            .map(|p| Ok(ParticipationId::from_str(&p.0)?))
            .collect();
        let snap = SnapshotContent { participations: list? };
        self.snapshot_contents.save_at(&snap, snapshot)
    }

    fn list_snapshots(&self, aggregation: &AggregationId) -> SdaServerResult<Vec<SnapshotId>> {
        Ok(self.snapshots
            .all::<Snapshot>()?
            .into_iter()
            .map(|p| p.1)
            .filter(|s| &s.aggregation == aggregation)
            .map(|s| s.id)
            .collect())
    }

    fn get_snapshot(&self,
                    _aggregation: &AggregationId,
                    snapshot: &SnapshotId)
                    -> SdaServerResult<Option<Snapshot>> {
        self.snapshots.get_option(snapshot)
    }

    fn iter_snapped_participations<'a, 'b>
        (&'b self,
         aggregation: &AggregationId,
         snapshot: &SnapshotId)
         -> SdaServerResult<Box<Iterator<Item = SdaServerResult<Participation>> + 'a>>
        where 'b: 'a
    {
        let store = self.aggregation_store(aggregation)?;
        let snap = self.snapshot_contents.get::<SnapshotContent>(&snapshot.to_string())?;
        let mut participations = vec![];
        for id in snap.participations {
            let part = store.get_option(&id)?
                .ok_or_else(|| {
                    format!("participation id={:?} for agg={:?} not found",
                            &id,
                            &aggregation)
                })?;
            participations.push(Ok(part));
        }
        Ok(Box::new(participations.into_iter()))
    }

    fn create_snapshot_mask(&self,
                            snapshot: &SnapshotId,
                            mask: Vec<Encryption>)
                            -> SdaServerResult<()> {
        self.snapshot_masks.save_at(&mask, snapshot)
    }

    fn get_snapshot_mask(&self, snapshot: &SnapshotId) -> SdaServerResult<Option<Vec<Encryption>>> {
        self.snapshot_masks.get_option(snapshot)
    }
}
