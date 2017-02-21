
//! Specific functionality for clerking.

use super::*;

/// Basic tasks for a clerk.
pub trait Clerk {

    /// `force` means contacting the service even if the client believes its already registered.
    /// Return value indicates whether this was the first time the service saw this clerk.
    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool>;

    /// Execute clerking process once: download, process, and upload the next job pending on the service, if any.
    fn clerk_once(&self) -> SdaClientResult<bool>;

    /// Execute routine clerking chores, including registering if not done so already.
    fn run_chores(&self) -> SdaClientResult<()>;

}

impl<T, S> Clerk for SdaClient<T, S> 
    where
        S: SdaClerkingService
{

    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool> {
        unimplemented!()
    }

    fn clerk_once(&self) -> SdaClientResult<bool> {
        // pull any pending job
        // TODO better way of doing this?
        match self.sda_service.pull_clerking_job(&self.agent, &self.agent.id)? {
            None => {
                Ok(false)
            },
            Some(job) => {
                // process
                let result = self.process_job(&job)?;
                // post result
                let _ = self.sda_service.push_clerking_result(&self.agent, &result)?;
                Ok(true)
            }
        }
    }

    fn run_chores(&self) -> SdaClientResult<()> {
        // register if we haven't done so already
        self.register_as_clerk(false)?;
        // repeatedly process jobs
        let max_iterations = 10;
        for _ in 0..max_iterations {
            if self.clerk_once()? { 
                continue
            } else {
                break
            }
        }
        Ok(())
    }

}

impl<T, S> SdaClient<T, S> {

    fn process_job(&self, job: &ClerkingJob) -> SdaClientResult<ClerkingResult> {
        unimplemented!()
    }

}
