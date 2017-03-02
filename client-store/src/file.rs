use std::path;
use jfs;

use errors::*;

pub struct Filebased(jfs::Store);

impl Filebased {

    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaClientStoreResult<Filebased> {
        let path = prefix.as_ref().join("keystore");
        let filename = path.to_str()
            .ok_or("Could not format filename for keystore")?;
        let filestore = jfs::Store::new(filename)?;
        Ok(Filebased(filestore))
    }

    pub fn resolve_alias(&self, alias: &str) -> SdaClientStoreResult<Option<String>> {
        let alias_id = "alias_".to_string() + alias;
        self.get(&alias_id)
    }

    pub fn define_alias(&self, alias: &str, id: &str) -> SdaClientStoreResult<()> {
        let alias_id = "alias_".to_string() + alias;
        match self.0.save_with_id(&id.to_string(), &alias_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)?,
        }
    }

    pub fn get_alias<T>(&self, alias: &str) -> SdaClientStoreResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize
    {
        match self.resolve_alias(alias)? {
            None => Ok(None),
            Some(id) => self.get(&id)
        }
    }

    pub fn get<T>(&self, id: &str) -> SdaClientStoreResult<Option<T>>
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