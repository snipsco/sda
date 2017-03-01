
use std::path;
use jfs::Store;

use super::*;


pub trait KeypairGen<T> {
    fn new_keypair(&self) -> SdaClientResult<T>;
}


pub struct Filebased(jfs::Store);


impl Filebased {

    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaClientResult<Filebased> {
        let path = prefix.as_ref().join("keystore");
        let filename = path.to_str()
            .ok_or("Could not format filename for keystore")?;
        let filestore = Store::new(filename)?;
        Ok(Filebased(filestore))
    }

    pub fn resolve_alias(&self, alias: &str) -> SdaClientResult<Option<String>> {
        let alias_id = "alias_".to_string() + alias;
        self.get(&alias_id)
    }

    pub fn define_alias(&self, alias: &str, id: &str) -> SdaClientResult<()> {
        let alias_id = "alias_".to_string() + alias;
        match self.0.save_with_id(&id.to_string(), &alias_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)?,
        }
    }

    pub fn get_alias<T>(&self, alias: &str) -> SdaClientResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize
    {
        match self.resolve_alias(alias)? {
            None => Ok(None),
            Some(id) => self.get(&id)
        }
    }

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

    // pub fn list(&self) -> SdaClientResult<Vec<String>> {
    //     let entries = self.0.all()?;
    //     Ok(entries.keys().cloned().collect())
    // }

}


impl KeypairGen<LabelledVerificationKeypairId> for Filebased {
    fn new_keypair(&self) -> SdaClientResult<LabelledVerificationKeypairId> {
        // generate
        let (vk, sk) = sodiumoxide::crypto::sign::gen_keypair();
        let wrapped_vk = VerificationKey::Sodium(vk.0.into());
        let wrapped_sk = SigningKey::Sodium(sk.0.into());
        
        // save
        let keypair = (wrapped_vk, wrapped_sk);
        let id = LabelledVerificationKeypairId::new();
        self.0.save_with_id(&keypair, &id.to_string());

        Ok(id)
    }
}



impl ExportDecryptionKey<SignedEncryptionKeyId, (sda_protocol::EncryptionKey, crypto::DecryptionKey)> for Filebased {
    fn export_decryption_key(&self, id: &SignedEncryptionKeyId) -> SdaClientResult<Option<(sda_protocol::EncryptionKey, crypto::DecryptionKey)>> {
        unimplemented!()
    }
}














// impl KeypairGen for Foo {
//     fn new_keypair<EncryptionKey>(&self) -> SdaClientResult<EncryptionKey> {
//         unimplemented!()
//     }
// }

// impl KeypairGen for Foo {
//     fn new_keypair<VerificationKey>(&self) -> SdaClientResult<VerificationKey> {
//         unimplemented!()
//     }
// }

pub trait GenerateEncryptionKeypair {
    fn new_keypair<I>(&self, scheme: &AdditiveEncryptionScheme) -> SdaClientResult<I>;
}

// TODO should not be allowed; keep decryption keys in IdentityModule instead and ask it to do the decryption
pub trait ExportDecryptionKey<I, DK> {
    fn export_decryption_key(&self, id: &I) -> SdaClientResult<Option<DK>>;
}

// impl<S, I> GenerateEncryptionKeypair for ... 
//     where 
//         S: GenerateKeypair,
//         I: New,
// {
//     fn new_keypair(&self, scheme: &S) -> SdaClientResult<I> {
//         let (ek, dk) = scheme.new_keypair()?;
//         let id = I::new();
//         // TODO store keypair under I; fail if exists already
//         Ok(id)
//     }
// }




pub trait IdentityModule {
    fn replace_identity_keypair(&mut self) -> SdaClientResult<()>;
    fn export_verification_key(&self) -> SdaClientResult<VerificationKey>;
    fn sign(&self, message: Vec<u8>) -> SdaClientResult<Signature>;
}

