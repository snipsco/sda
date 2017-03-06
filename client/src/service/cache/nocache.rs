use errors::SdaClientResult;
use super::Cache;

pub struct NoCache;

#[allow(unused_variables)]
impl<ID, O> Cache<ID, O> for NoCache {
    fn has(&self, id: &ID) -> SdaClientResult<bool> {
        Ok(false)
    }
    fn save(&self, id: &ID, object: &O) -> SdaClientResult<()> {
        Ok(())
    }
    fn load(&self, id: &ID) -> SdaClientResult<O> {
        unimplemented!()
    }
}