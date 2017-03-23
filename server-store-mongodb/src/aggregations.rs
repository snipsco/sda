use sda_protocol::*;
use sda_server::stores;
use sda_server::errors::*;
use {to_bson, to_doc, Dao, from_bson};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct AggregationDocument {
    id: AggregationId,
    aggregation: Aggregation,
    committee: Option<Committee>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SnapshotDocument {
    id: SnapshotId,
    snapshot: Snapshot,
    mask: Option<Vec<Encryption>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ParticipationDocument {
    id: SnapshotId,
    participation: Participation,
    #[serde(default)]
    snapshots: Vec<SnapshotId>,
}

pub struct MongoAggregationsStore {
    aggregations: Dao<AggregationId, AggregationDocument>,
    participations: Dao<ParticipationId, ParticipationDocument>,
    snapshots: Dao<SnapshotId, SnapshotDocument>,
}

impl MongoAggregationsStore {
    pub fn new(db: &::mongodb::db::Database) -> SdaServerResult<MongoAggregationsStore> {
        use mongodb::db::ThreadedDatabase;
        let store = MongoAggregationsStore {
            aggregations: Dao::new(db.collection("aggregations")),
            participations: Dao::new(db.collection("participations")),
            snapshots: Dao::new(db.collection("snapshots")),
        };
        store.aggregations.ensure_index(d!("id" => 1), true)?;
        store.participations.ensure_index(d!("id" => 1), true)?;
        store.snapshots.ensure_index(d!("id" => 1), true)?;
        Ok(store)
    }
}

impl stores::BaseStore for MongoAggregationsStore {
    fn ping(&self) -> SdaServerResult<()> {
        self.aggregations.ping()
    }
}

impl stores::AggregationsStore for MongoAggregationsStore {
    fn list_aggregations(&self,
                         filter: Option<&str>,
                         recipient: Option<&AgentId>)
                         -> SdaServerResult<Vec<AggregationId>> {
        let mut selector = ::bson::Document::new();
        if let Some(filter) = filter {
            selector.insert_bson("aggregation.title".into(),
                                 ::bson::Bson::RegExp(filter.into(), "".into()));
        }
        if let Some(recipient) = recipient {
            selector.insert("aggregation.recipient", m!(to_bson(recipient))?);
        }
        self.aggregations
            .find(selector)?
            .map(|res| res.map(|it| it.id))
            .collect()
    }

    fn create_aggregation(&self, aggregation: &Aggregation) -> SdaServerResult<()> {
        self.aggregations.modisert_by_id(&aggregation.id,
                                         d!("$set" => d!("id" => to_bson(&aggregation.id)?, 
                            "aggregation" => to_doc(aggregation)?)))
    }

    fn get_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Aggregation>> {
        self.aggregations.get_by_id(aggregation).map(|opt| opt.map(|a| a.aggregation))
    }

    fn delete_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<()> {
        m!(self.aggregations.coll.delete_one(d!("id" => m!(to_bson(aggregation))?), None))?;
        Ok(())
    }

    fn get_committee(&self, owner: &AggregationId) -> SdaServerResult<Option<Committee>> {
        self.aggregations.get_by_id(owner).map(|opt| opt.and_then(|a| a.committee))
    }

    fn create_committee(&self, committee: &Committee) -> SdaServerResult<()> {
        self.aggregations.modify_by_id(&committee.aggregation,
                                       d!("$set" => d!("committee" => to_doc(committee)?)))
    }

    fn create_participation(&self, participation: &Participation) -> SdaServerResult<()> {
        self.participations.modisert_by_id(&participation.id,
                                           d!("$set" => d!("id" => to_bson(&participation.id)?, 
                            "participation" => to_doc(participation)?)))
    }

    fn create_snapshot(&self, snapshot: &Snapshot) -> SdaServerResult<()> {
        self.snapshots.modisert_by_id(&snapshot.id,
                                      d!("$set" => d!("id" => to_bson(&snapshot.id)?, 
                            "snapshot" => to_doc(snapshot)?)))
    }

    fn list_snapshots(&self, aggregation: &AggregationId) -> SdaServerResult<Vec<SnapshotId>> {
        self.snapshots
            .find(d!("snapshot.aggregation" => to_bson(aggregation)?))?
            .map(|res| res.map(|s| s.id))
            .collect()
    }

    fn get_snapshot(&self,
                    _aggregation: &AggregationId,
                    snapshot: &SnapshotId)
                    -> SdaServerResult<Option<Snapshot>> {
        self.snapshots.get_by_id(snapshot).map(|opt| opt.map(|s| s.snapshot))
    }

    fn count_participations(&self, aggregation: &AggregationId) -> SdaServerResult<usize> {
        m!(self.participations
                .coll
                .count(Some(d!("participation.aggregation" => to_bson(aggregation)?)),
                       None))
            .map(|i| i as _)
    }

    fn snapshot_participations(&self,
                               aggregation: &AggregationId,
                               snapshot: &SnapshotId)
                               -> SdaServerResult<()> {
        m!(self.participations
            .coll
            .update_many(d!("participation.aggregation" => to_bson(aggregation)?),
                         d!("$addToSet" => d!("snapshots" => to_bson(snapshot)?)),
                         None))?;
        Ok(())
    }

    fn iter_snapped_participations<'a, 'b>
        (&'b self,
         _aggregation: &AggregationId,
         snapshot: &SnapshotId)
         -> SdaServerResult<Box<Iterator<Item = SdaServerResult<Participation>> + 'a>>
        where 'b: 'a
    {
        Ok(Box::new(self.participations
            .find(d!("snapshots" => to_bson(snapshot)?))?
            .map(|res| res.map(|pd| pd.participation))))
    }

    fn count_participations_snapshot(&self,
                                     _aggregation: &AggregationId,
                                     snapshot: &SnapshotId)
                                     -> SdaServerResult<usize> {
        m!(self.participations.coll.count(Some(d!("snapshots" => to_bson(snapshot)?)), None))
            .map(|i| i as _)
    }

    fn iter_snapshot_clerk_jobs_data<'a, 'b>
        (&'b self,
         _aggregation: &AggregationId,
         snapshot: &SnapshotId,
         _clerks_number: usize)
         -> SdaServerResult<Box<Iterator<Item = SdaServerResult<Vec<Encryption>>> + 'a>>
        where 'b: 'a
    {
        use mongodb::coll::options::AggregateOptions;
        use mongodb::cursor::Cursor;
        let cursor:Cursor = m!(self.participations.coll
                .aggregate(vec!(
                        d!("$match" => d!("snapshots" => to_bson(snapshot)?)),
                        d!("$unwind" =>
                           d!("path" => "$participation.clerk_encryptions", "includeArrayIndex" => "clerk_id")),
                        d!("$group" => d!("_id" => "$clerk_id", "shares" => d!("$push" => "$participation.clerk_encryptions"))),
                        d!("$sort" => d!("_id" => 1))
                        ),
                    Some(AggregateOptions {
                        allow_disk_use: Some(true),
                        use_cursor: Some(true),
                        .. AggregateOptions::default()
                    })))?;
        let shares = cursor.map(|doc| -> SdaServerResult<Vec<Encryption>> {
            let doc = m!(doc)?;
            let shares = doc.get("shares").ok_or("invalid aggregation result")?;
            let shares: Vec<(AgentId, Encryption)> = from_bson(shares.to_owned())?;
            Ok(shares.into_iter().map(|(_id, enc)| enc).collect())
        });
        Ok(Box::new(shares))

    }

    fn create_snapshot_mask(&self,
                            snapshot: &SnapshotId,
                            mask: Vec<Encryption>)
                            -> SdaServerResult<()> {
        self.snapshots.modify_by_id(&snapshot, d!("$set" => d!("mask" => to_bson(&mask)?)))
    }

    fn get_snapshot_mask(&self, snapshot: &SnapshotId) -> SdaServerResult<Option<Vec<Encryption>>> {
        self.snapshots.get_by_id(snapshot).map(|opt| opt.and_then(|s| s.mask))
    }
}
