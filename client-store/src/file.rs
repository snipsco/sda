use std::path;
use serde;
use jfs;

use errors::*;
use super::Store;

pub struct Filebased(jfs::Store);

impl Filebased {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaClientStoreResult<Filebased> {
        let path = prefix.as_ref();
        let filename = path.to_str()
            .ok_or("Could not format filename for store")?;
        let filestore = jfs::Store::new(filename)?;
        Ok(Filebased(filestore))
    }
}

impl Store for Filebased {

    fn put<T>(&self, id: &str, obj: &T) -> SdaClientStoreResult<()>
        where T: serde::Serialize + serde::Deserialize
    {
        self.0.save_with_id(obj, id)?;
        Ok(())
    }

    fn get<T>(&self, id: &str) -> SdaClientStoreResult<Option<T>>
        where T: serde::Serialize + serde::Deserialize
    {
        match self.0.get(id) {
            Ok(it) => Ok(Some(it)),
            Err(io) => {
                if io.kind() == ::std::io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(io)?
                }
            }
        }
    }

}