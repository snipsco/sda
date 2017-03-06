use errors::SdaClientResult;

/// Basic caching.
pub trait Cache<ID, O> {
    fn has(&self, id: &ID) -> SdaClientResult<bool>;
    fn save(&self, id: &ID, object: &O) -> SdaClientResult<()>;
    fn load(&self, id: &ID) -> SdaClientResult<O>;
}

mod nocache;
pub use self::nocache::NoCache;