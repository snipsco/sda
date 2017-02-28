
use std::path;
use jfs::Store;

use super::*;


pub struct JsonFileStore(jfs::Store);


impl JsonFileStore {

    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaClientResult<JsonFileStore> {
        let mut dbcfg = jfs::Config::default();
        dbcfg.single = true;
        let filename = prefix.as_ref().join("keystore.json")
            .to_str().ok_or("Could not format filename for keystore")?;
        let db = Store::new_with_cfg(filename, dbcfg)?;
        Ok(JsonFileStore(db))
    }

    fn get<T>(&self, id: &str) -> SdaClientResult<Option<T>>
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


impl ExportDecryptionKey<SignedEncryptionKeyId, (EncryptionKey, DecryptionKey)> for JsonFileStore {
    fn export_decryption_key(&self, id: &SignedEncryptionKeyId) -> SdaClientResult<Option<(EncryptionKey, DecryptionKey)>> {
        self.get(&id.to_string())
    }
}
