
use super::*;

pub trait Policy<ID> {
    fn is_flagged_as_trusted(&self, id: &ID) -> SdaClientResult<bool>;
    fn flag_as_trusted(&mut self, id: &ID) -> SdaClientResult<()>;
    fn unflag_as_trusted(&mut self, id: &ID) -> SdaClientResult<()>;
}


pub struct Permissistic;


impl<C, S> Policy<AgentId> for SdaClient<C, S> {
    
    fn is_flagged_as_trusted(&self, id: &AgentId) -> SdaClientResult<bool> {
        unimplemented!()
    }

    fn flag_as_trusted(&mut self, id: &AgentId) -> SdaClientResult<()> {
        unimplemented!()
    }

    fn unflag_as_trusted(&mut self, id: &AgentId) -> SdaClientResult<()> {
        unimplemented!()
    }

}

