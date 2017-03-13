use jfs;

use std::path;

use sda_protocol::Id;
use sda_protocol::{AgentId, ClerkingJob, ClerkingJobId, ClerkingResult, SnapshotId};

use stores::{BaseStore, ClerkingJobStore};
use jfs_stores::JfsStoreExt;

use SdaServerResult;

pub struct JfsClerkingJobStore(path::PathBuf);

impl JfsClerkingJobStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsClerkingJobStore> {
        Ok(JfsClerkingJobStore(prefix.as_ref().to_path_buf()))
    }

    fn store<I: Id>(&self, prefix: &str, id: &I) -> SdaServerResult<jfs::Store> {
        Ok(jfs::Store::new(self.0
            .join(prefix)
            .join(id.to_string())
            .to_str()
            .ok_or("pathbuf to string")?)?)
    }
}

impl BaseStore for JfsClerkingJobStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl ClerkingJobStore for JfsClerkingJobStore {
    fn enqueue_clerking_job(&self, job: &ClerkingJob) -> SdaServerResult<()> {
        self.store("queue", &job.clerk)?.save_ident(job)
    }

    fn poll_clerking_job(&self, clerk: &AgentId) -> SdaServerResult<Option<ClerkingJob>> {
        Ok(self.store("queue", clerk)?.all::<ClerkingJob>()?.into_iter().next().map(|a| a.1))
    }

    fn get_clerking_job(&self,
                        clerk: &AgentId,
                        job: &ClerkingJobId)
                        -> SdaServerResult<Option<ClerkingJob>> {
        self.store("queue", clerk)?.get_option(job)
    }

    fn create_clerking_result(&self, result: &ClerkingResult) -> SdaServerResult<()> {
        let job: ClerkingJob = self.store("queue", &result.clerk)?
            .get_option(&result.job)?
            .ok_or("Job not found")?;
        self.store("results", &job.snapshot)?.save_at(result, &result.job)?;
        self.store("done", &result.clerk)?.save_at(&job, &result.job)?;
        self.store("queue", &result.clerk)?.delete(&*result.job.to_string())?;
        Ok(())
    }

    fn list_results(&self, snapshot: &SnapshotId) -> SdaServerResult<Vec<ClerkingJobId>> {
        Ok(self.store("results", snapshot)?
            .all::<ClerkingResult>()?
            .iter()
            .map(|r| r.1.job)
            .collect::<Vec<ClerkingJobId>>())
    }

    fn get_result(&self,
                  snapshot: &SnapshotId,
                  job: &ClerkingJobId)
                  -> SdaServerResult<Option<ClerkingResult>> {
        self.store("results", snapshot)?.get_option(job)
    }
}
