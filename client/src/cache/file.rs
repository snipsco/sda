
use std::path;
use jfs::Store;

use super::*;


pub struct FileStore(jfs::Store);

impl FileStore {

    pub fn get<T>(&self, id: &str) -> SdaClientResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize
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
