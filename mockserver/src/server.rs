
use sda_protocol::*;

pub struct MockServer;

impl SdaService for MockServer {

    fn ping(&self) -> SdaResult<()> {
        Ok(())
    }

}

impl SdaAggregationService for MockServer {

    fn clerk_register(&self, profile: &ClerkProfile) -> SdaResult<Option<String>> {
        Ok(None)
    }

    fn clerk_pull_job(&self, identity: &ClerkIdentity) -> SdaResult<Option<PartialAggregationJob>> {
        Ok(None)
    }

    fn clerk_push_result(&self, identity: &ClerkIdentity, result: &PartialAggregationResult) -> SdaResult<()> {
        Ok(())
    }

    fn user_post_participation(&self, identity: &UserIdentity, participation: &Participation) -> SdaResult<()> {
        Ok(())
    }

}