
use super::*;

pub use jfs::Store;

impl ExportDecryptionKey<SignedEncryptionKeyId, (EncryptionKey, DecryptionKey)> for Store {
    fn export_decryption_key(&self, id: &SignedEncryptionKeyId) -> SdaClientResult<(EncryptionKey, DecryptionKey)> {
        self.get(id)?
    }
}
