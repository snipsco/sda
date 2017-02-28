
use std::path;
use jfs::Store;

use super::*;


pub struct FileKeyStore(jfs::Store);


impl FileKeyStore {

    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaClientResult<FileKeyStore> {
        
        let path = prefix.as_ref().join("keystore.json");
        let filename = path.to_str()
            .ok_or("Could not format filename for keystore")?;
        
        let mut cfg = jfs::Config::default();
        cfg.single = true;

        let filestore = Store::new_with_cfg(filename, cfg)?;
        Ok(FileKeyStore(filestore))
    }

    // pub fn init(&self) ->SdaClientResult<bool> {

    // }

}













// impl GenerateEncryptionKeypair {
//     fn new_keypair(&self, scheme: &AdditiveEncrytionScheme) -> SdaClientResult<SignedEncryptionKeyId> {
//         match scheme {

//             Sodium => {
//                 let (pk, sk) = sodiumoxide::crypto::box_::gen_keypair();
                
//             }

//         }
//     }
// }


// impl ExportDecryptionKey<EncryptionKey, DecryptionKey> for FileKeyStore {
//     fn export_decryption_key(&self, ek: &EncryptionKey) -> SdaClientResult<Option<DecryptionKey>> {

//         let id = match ek {

//         }

//         let dk = self.get(&id.to_string())
//     }
// }
