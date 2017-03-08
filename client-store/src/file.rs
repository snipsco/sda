use std::path;
use serde;
use jfs;

use errors::*;
use super::*;

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

macro_rules! wrap {
    ($e:expr) => {
        match $e {
            Ok(o) => Ok(o),
            Err(err) => Err(format!("Storage error: {}", err).into()),
        }
    }
}

impl KeyStorage<EncryptionKeyId, EncryptionKeypair> for Filebased {
    fn put(&self, id: &EncryptionKeyId, obj: &EncryptionKeypair) -> SdaClientResult<()> {
        wrap! { <Self as Store>::put(self, &id.stringify(), obj) }
    }
    fn get(&self, id: &EncryptionKeyId) -> SdaClientResult<Option<EncryptionKeypair>> {
        wrap! { <Self as Store>::get(self, &id.stringify()) }
    }
}

impl KeyStorage<VerificationKeyId, SignatureKeypair> for Filebased {
    fn put(&self, id: &VerificationKeyId, obj: &SignatureKeypair) -> SdaClientResult<()> {
        wrap! { <Self as Store>::put(self, &id.stringify(), obj) }
    }
    fn get(&self, id: &VerificationKeyId) -> SdaClientResult<Option<SignatureKeypair>> {
        wrap! { <Self as Store>::get(self, &id.stringify()) }
    }
}

impl Keystore for Filebased {}