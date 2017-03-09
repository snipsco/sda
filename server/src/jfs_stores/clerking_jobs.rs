use jfs;

use std::path;

use sda_protocol::Id;
use sda_protocol::{ AgentId, ClerkingJob };

use stores::{BaseStore, ClerkingJobStore};

use SdaServerResult;

pub struct JfsClerkingJobStore {
    queues: path::PathBuf,
}

impl JfsClerkingJobStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsClerkingJobStore> {
        Ok(JfsClerkingJobStore { queues: prefix.as_ref().join("jobs") })
    }

    fn queue(&self, clerk:&AgentId) -> SdaServerResult<jfs::Store> {
        Ok(jfs::Store::new(self.queues
            .join(clerk.stringify())
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
        self.queue(&job.clerk)?.save_with_id(job, &job.id.stringify())?;
        Ok(())
    }

    fn get_clerking_job(&self, clerk:&AgentId) -> SdaServerResult<Option<ClerkingJob>> {
        Ok(self.queue(clerk)?.all::<ClerkingJob>()?.into_iter().next().map(|a| a.1))
    }
}
