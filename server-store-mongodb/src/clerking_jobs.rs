use sda_protocol::*;
use sda_server::stores;
use sda_server::errors::*;
use {to_bson, to_doc, Dao};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ClerkingJobDocument {
    id: ClerkingJobId,
    clerking_job: ClerkingJob,
    done: bool,
    result: Option<ClerkingResult>,
}


pub struct MongoClerkingJobsStore(Dao<ClerkingJobId, ClerkingJobDocument>);

impl MongoClerkingJobsStore {
    pub fn new(db: &::mongodb::db::Database) -> SdaServerResult<MongoClerkingJobsStore> {
        use mongodb::db::ThreadedDatabase;
        let dao = Dao::new(db.collection("clerking_jobs"));
        dao.ensure_index(d!("id" => 1), true)?;
        Ok(MongoClerkingJobsStore(dao))
    }
}

impl stores::BaseStore for MongoClerkingJobsStore {
    fn ping(&self) -> SdaServerResult<()> {
        self.0.ping()
    }
}

impl stores::ClerkingJobsStore for MongoClerkingJobsStore {
    fn enqueue_clerking_job(&self, job: &ClerkingJob) -> SdaServerResult<()> {
        self.0.modisert_by_id(&job.id,
                              d!("$set" => d!("clerking_job" => to_doc(job)?,
                                      "id" => to_bson(&job.id)?,
                                      "done" => false) ))
    }

    fn poll_clerking_job(&self, clerk: &AgentId) -> SdaServerResult<Option<ClerkingJob>> {
        self.0
            .get(d!("done" => false, "clerking_job.clerk" => to_bson(clerk)?))
            .map(|opt| opt.map(|doc| doc.clerking_job))
    }

    fn get_clerking_job(&self,
                        clerk: &AgentId,
                        job: &ClerkingJobId)
                        -> SdaServerResult<Option<ClerkingJob>> {
        self.0
            .get(d!("clerking_job.clerk" => to_bson(clerk)?, "id" => to_bson(job)?))
            .map(|opt| opt.map(|doc| doc.clerking_job))
    }

    fn create_clerking_result(&self, result: &ClerkingResult) -> SdaServerResult<()> {
        self.0.modisert_by_id(&result.job,
                              d!("$set" => d!("result" => to_doc(result)?, "done" => true)))
    }

    fn list_results(&self, snapshot: &SnapshotId) -> SdaServerResult<Vec<ClerkingJobId>> {
        self.0
            .find(d!("clerking_job.snapshot" => to_bson(snapshot)?, "done" => true))?
            .map(|res| res.map(|cj| cj.id))
            .collect()
    }

    fn get_result(&self,
                  snapshot: &SnapshotId,
                  job: &ClerkingJobId)
                  -> SdaServerResult<Option<ClerkingResult>> {
        self.0
            .get(d!("clerking_job.snapshot" => to_bson(snapshot)?, "id" => to_bson(job)?))
            .map(|opt| opt.and_then(|doc| doc.result))
    }
}
