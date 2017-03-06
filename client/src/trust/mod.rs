use errors::SdaClientResult;

pub trait Policy<ID> {
    fn is_flagged_as_trusted(&self, id: &ID) -> SdaClientResult<bool>;
    fn flag_as_trusted(&mut self, id: &ID) -> SdaClientResult<()>;
    fn unflag_as_trusted(&mut self, id: &ID) -> SdaClientResult<()>;
}

mod pessimistic;
pub use self::pessimistic::Pessimistic;