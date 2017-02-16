
use super::*;

pub trait SdaClerk {

    fn register(&self, force: bool) -> SdaClientResult<bool>;

    fn update_profile(&self) -> SdaClientResult<()>;

    fn clerk_once(&self) -> SdaClientResult<()>;

    fn run_chores(&self) -> SdaClientResult<()>;

}

impl<S: SdaAggregationService> SdaClerk for SdaClient<S> {

    fn register(&self, force: bool) -> SdaClientResult<bool> {
        // TODO
    }

    fn update_profile(&self) -> SdaClientResult<()> {
        // TODO
        self.register();
        self.post_profile()
    }

    fn clerk_once(&self) -> SdaClientResult<()> {
        // TODO
        let job = self.get_job();
        let result = self.process_job(job);
        self.post_result(result)
    }

    fn run_chores(&self) -> SdaClientResult<()> {
        self.register(false);
        while self.clerk_once() {
            // keep clerking until no more tasks
            // TODO put in safety measure to prevent long loops
        }
    }

}

/// Fine-tuned clerk operations, not needed for basic use.
pub trait Operations {

    fn put_identity(&self) -> SdaClientResult<bool>;

    fn post_profile(&self) -> SdaClientResult<()>;

    fn get_job(&self) -> SdaClientResult<Option<ClerkingJob>>;

    fn process_job(&self, job: AsRef<ClerkingJob>) -> SdaClientResult<ClerkingResult>;

    fn post_result(&self, result: AsRef<ClerkingResult>) -> SdaClientResult<()>;

}

impl<S: SdaAggregationService> Operations for SdaClient<S> {

    fn put_identity(&self) -> SdaClientResult<bool> {
        // TODO
    }

    fn post_profile(&self) -> SdaClientResult<()> {
        // TODO
    }

    fn get_job(&self) -> SdaClientResult<Option<ClerkingJob>> {
        // TODO
    }

    fn process_job(&self, job: AsRef<ClerkingJob>) -> SdaClientResult<ClerkingResult> {
        // TODO
    }

    fn post_result(&self, result: AsRef<ClerkingResult>) -> SdaClientResult<()> {
        // TODO
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_first_register() {
        let agent = FileSecurityAgent::new();
        let service = ClientHttpTunnel::new(&agent);
        let client: SdaClerk = SdaClient::new(service, agent);

        let success = client.register(false)?;
        assert!(success);
    }

}