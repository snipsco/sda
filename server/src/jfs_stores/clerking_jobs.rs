use jfs;

use std::path;

use sda_protocol::Id;
use sda_protocol::ClerkingJob;

use stores::{BaseStore, ClerkingJobStore};

use SdaServerResult;

pub struct JfsClerkingJobStore {
    queues: path::PathBuf,
}

impl JfsClerkingJobStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsClerkingJobStore> {
        Ok(JfsClerkingJobStore { queues: prefix.as_ref().join("jobs") })
    }
}

impl BaseStore for JfsClerkingJobStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl ClerkingJobStore for JfsClerkingJobStore {
    fn enqueue_clerking_job(&self, job: &ClerkingJob) -> SdaServerResult<()> {
        let queue = jfs::Store::new(self.queues
            .join(job.clerk.stringify())
            .to_str()
            .ok_or("pathbuf to string")?)?;
        queue.save_with_id(job, &job.id.stringify())?;
        Ok(())
    }
}
