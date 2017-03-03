use errors::*;
use super::Policy;

pub struct Pessimistic;

impl<ID> Policy<ID> for Pessimistic {
    fn is_flagged_as_trusted(&self, id: &ID) -> SdaClientResult<bool> {
        Ok(false)
    }
    fn flag_as_trusted(&mut self, id: &ID) -> SdaClientResult<()> {
        // ignore request
        Ok(())
    }
    fn unflag_as_trusted(&mut self, id: &ID) -> SdaClientResult<()> {
        Ok(())
    }
}