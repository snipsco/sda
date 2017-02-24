
use super::*;


pub trait Cache<ID, O> {
    fn has(&self, id: &ID) -> SdaClientResult<bool>;
    fn save(&self, id: &ID, object: &O) -> SdaClientResult<()>;
    fn load(&self, id: &ID) -> SdaClientResult<O>;
}

pub trait CachedFetch<ID, O> {
    fn cached_fetch(&mut self, id: &ID) -> SdaClientResult<O>;
}


impl<ID, O, C, K, S> Cache<ID, O> for SdaClient<C, K, S>
    where C: Cache<ID, O>
{

    fn has(&self, id: &ID) -> SdaClientResult<bool> { 
        self.cache.has(id) 
    }

    fn save(&self, id: &ID, object: &O) -> SdaClientResult<()> {
        self.cache.save(id, object)
    }

    fn load(&self, id: &ID) -> SdaClientResult<O> {
        self.cache.load(id)
    }

}

// Generic implementation for caching, combining fetching and storage.
impl<ID, O, T> CachedFetch<ID, O> for T
    where
        T: Cache<ID, O>,
        T: Fetch<ID, O>,
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