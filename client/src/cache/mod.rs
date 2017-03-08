mod nocache;
pub use self::nocache::NoCache;

use SdaClient;
use errors::SdaClientResult;

/// Basic caching.
pub trait Cache<ID, O> {
    fn has(&self, id: &ID) -> SdaClientResult<bool>;
    fn save(&self, id: &ID, object: &O) -> SdaClientResult<()>;
    fn load(&self, id: &ID) -> SdaClientResult<O>;
}

pub struct CachedService<S> {
    cache: NoCache,
    service: S,
}

impl SdaService for CachedService {

}



/// Combined fetching and caching.
pub trait CachedFetch<ID, O> {
    fn cached_fetch(&mut self, id: &ID) -> SdaClientResult<O>;
}

/// Generic implementation for caching, combining fetching and storage.
impl<ID, O, T> CachedFetch<ID, O> for T
    where
        T: Fetch<ID, O>,
        T: Cache<ID, O>,
{
    fn cached_fetch(&mut self, id: &ID) -> SdaClientResult<O> {
        if self.has(id)? {
            self.load(id)
        } else {
            let obj = self.fetch(id)?;
            self.save(id, &obj)?;
            Ok(obj)
        }
    }
}