
//! Specific functionality for clerking.

use super::*;

/// Basic tasks for a clerk.
pub trait Clerk {

    /// `force` means contacting the service even if the client believes its already registered.
    /// Return value indicates whether this was the first time the service saw this clerk.
    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool>;

    /// Execute clerking process once: download, process, and upload the next job pending on the service, if any.
    fn clerk_once(&self) -> SdaClientResult<()>;

    /// Execute routine clerking chores, including registering if not done so already.
    fn run_chores(&self) -> SdaClientResult<()>;

}

impl<T, S> Clerk for SdaClient<T, S> 
    where
        S: SdaAggregationService
{

    fn register_as_clerk(&self, force: bool) -> SdaClientResult<bool> {
        // TODO
    }

    // fn update_profile(&self) -> SdaClientResult<()> {
    //     // TODO
    //     self.register();
    //     self.post_profile()
    // }

    fn clerk_once(&self) -> SdaClientResult<()> {
        // TODO
        let job = self.get_job();
        let result = self.process_job(job);
        self.post_result(result)
    }

    fn run_chores(&self) -> SdaClientResult<()> {
        self.register_as_clerk(false);
        while self.clerk_once() {
            // keep clerking until no more tasks
            // TODO put in safety measure to prevent long loops
        }
    }

}

/// Fine-tuned clerk operations, not needed for basic use.
// pub trait Operations {

//     fn get_job(&self) -> SdaClientResult<Option<ClerkingJob>>;

//     fn process_job(&self, job: AsRef<ClerkingJob>) -> SdaClientResult<ClerkingResult>;

//     fn post_result(&self, result: AsRef<ClerkingResult>) -> SdaClientResult<()>;

// }

// impl<S: SdaAggregationService> Operations for SdaClient<S> {

//     fn get_job(&self) -> SdaClientResult<Option<ClerkingJob>> {
//         // TODO
//     }

//     fn process_job(&self, job: AsRef<ClerkingJob>) -> SdaClientResult<ClerkingResult> {
//         // TODO
//     }

//     fn post_result(&self, result: AsRef<ClerkingResult>) -> SdaClientResult<()> {
//         // TODO
//     }

// }

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_first_register() {
//         let agent = FileSecurityAgent::new();
//         let service = ClientHttpTunnel::new(&agent);
//         let client: SdaClerk = SdaClient::new(service, agent);

//         let success = client.register(false)?;
//         assert!(success);
//     }

// }