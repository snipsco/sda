use jfs;

use std::path;

use sda_protocol::Id;
use sda_protocol::{ AgentId, ClerkingJob, ClerkingJobId, ClerkingResult };

use stores::{BaseStore, ClerkingJobStore};

use SdaServerResult;

pub struct JfsClerkingJobStore(path::PathBuf);

impl JfsClerkingJobStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsClerkingJobStore> {
        Ok(JfsClerkingJobStore(prefix.as_ref().to_path_buf()))
    }

    fn store<I:Id>(&self, prefix:&str, id:&I) -> SdaServerResult<jfs::Store> {
        Ok(jfs::Store::new(self.0
            .join(prefix)
            .join(id.stringify())
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
        self.store("queue", &job.clerk)?.save_with_id(job, &job.id.stringify())?;
        Ok(())
    }

    fn poll_clerking_job(&self, clerk:&AgentId) -> SdaServerResult<Option<ClerkingJob>> {
        Ok(self.store("queue", clerk)?.all::<ClerkingJob>()?.into_iter().next().map(|a| a.1))
    }

    fn get_clerking_job(&self, clerk:&AgentId, job:&ClerkingJobId) -> SdaServerResult<Option<ClerkingJob>> {
        super::get_option(&self.store("queue", clerk)?, &job.stringify())
    }

    fn create_clerking_result(&self, result: &ClerkingResult) -> SdaServerResult<()> {
        let job:ClerkingJob = super::get_option(&self.store("queue", &result.clerk)?, &*result.job.stringify())?.ok_or("Job not found")?;
        self.store("results", &job.snapshot)?.save_with_id(result, &*result.job.stringify())?;
        self.store("done", &result.clerk)?.save_with_id(&job, &*result.job.stringify())?;
        self.store("queue", &result.clerk)?.delete(&*result.job.stringify())?;
        Ok(())
    }
}
