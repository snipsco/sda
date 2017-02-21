
use super::*;


pub trait Store<ID, O> {
    fn has(&self, id: &ID) -> SdaClientResult<bool>;
    fn save(&self, id: &ID, object: &O) -> SdaClientResult<()>;
    fn load(&self, id: &ID) -> SdaClientResult<O>;
    fn drop(&self, id: &ID) -> SdaClientResult<()>;
}


impl<ID, O, L, I, S> Store<ID, O> for SdaClient<L, I, S>
    where L: Store<ID, O>
{

    fn has(&self, id: &ID) -> SdaClientResult<bool> { 
        self.local_store.has(id) 
    }

    fn save(&self, id: &ID, object: &O) -> SdaClientResult<()> {
        self.local_store.save(id, object)
    }

    fn load(&self, id: &ID) -> SdaClientResult<O> {
        self.local_store.load(id)
    }

    fn drop(&self, id: &ID) -> SdaClientResult<()> {
        self.local_store.drop(id)
    }

}