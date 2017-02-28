
use std::path;
use jfs::Store;

use super::*;


pub trait KeypairGen<T> {
    fn new_keypair(&self) -> SdaClientResult<T>;
}


pub struct Keystore(jfs::Store);


impl Keystore {

    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaClientResult<Keystore> {
        let path = prefix.as_ref().join("keystore");
        let filename = path.to_str()
            .ok_or("Could not format filename for keystore")?;
        let filestore = Store::new(filename)?;
        Ok(Keystore(filestore))
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


impl KeypairGen<LabelledVerificationKeypairId> for Keystore {
    fn new_keypair(&self) -> SdaClientResult<LabelledVerificationKeypairId> {
        // generate
        let (vk, sk) = sodiumoxide::crypto::sign::gen_keypair();
        let wrapped_vk = VerificationKey::Sodium(vk.0);
        let wrapped_sk = SigningKey::Sodium(sk.0);
        
        // save
        let keypair = (wrapped_vk, wrapped_sk);
        let id = LabelledVerificationKeypairId::new();
        self.0.save_with_id(&keypair, id.to_string());

        Ok(id)
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

