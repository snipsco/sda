use super::*;

use sda_protocol::*;

use sda_client_store::{Store};

pub struct Keystore<U>(U);

impl<U> Keystore<U> {
    pub fn new(underlying_store: U) -> Keystore<U> {
        Keystore(underlying_store)
    }
}

pub trait KeyGeneration<T> {
    fn new_key(&self) -> SdaClientResult<T>;
}

pub trait Export<I, K> {
    fn export(&self, id: &I) -> SdaClientResult<Option<K>>;
}

pub trait SignExport<ID, OBJ>
    where OBJ: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    fn sign_export(&self, signer: &Agent, id: &ID) -> SdaClientResult<Option<Signed<OBJ>>>;
}





// TODO rename
#[derive(Debug, Serialize, Deserialize)]
struct VerificationKeypair {
    vk: VerificationKey,
    sk: SigningKey,
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptionKeypair {
    ek: EncryptionKey,
    dk: DecryptionKey,
}

impl<K: Store> KeyGeneration<VerificationKeyId> for K {
    fn new_key(&self) -> SdaClientResult<VerificationKeyId> {
        // generate
        let (vk, sk) = sodiumoxide::crypto::sign::gen_keypair();
        let wrapped_vk = VerificationKey::Sodium(vk.0.into());
        let wrapped_sk = SigningKey::Sodium(sk.0.into());
        
        // save
        let keypair = VerificationKeypair { vk: wrapped_vk, sk: wrapped_sk };
        let id = VerificationKeyId::new();
        self.put(&id.to_string(), &keypair);

        Ok(id)
    }
}

impl<K: Store> KeyGeneration<Labeled<VerificationKeyId, VerificationKey>> for K {
    fn new_key(&self) -> SdaClientResult<Labeled<VerificationKeyId, VerificationKey>> {
        // generate key
        let key_id: VerificationKeyId = self.new_key()?;
        
        // export public part, assuming that it is there since we just created it and haven't failed
        let key: VerificationKey = self.export(&key_id)?.unwrap();

        Ok(Labeled {
            id: key_id,
            body: key,
        })
    }
}

impl<K: Store> Export<VerificationKeyId, VerificationKey> for K {
    fn export(&self, id: &VerificationKeyId) -> SdaClientResult<Option<VerificationKey>> {
        let keypair: Option<VerificationKeypair> = self.get(&id.to_string())?;
        match keypair {
            None => Ok(None),
            Some(keypair) => Ok(Some(keypair.vk))
        }
    }
}

impl<K: Store> KeyGeneration<EncryptionKeyId> for K {
    fn new_key(&self) -> SdaClientResult<EncryptionKeyId> {
        // generate
        let (pk, sk) = sodiumoxide::crypto::box_::gen_keypair();
        let wrapped_ek = EncryptionKey::Sodium(pk.0.into());
        let wrapped_dk = DecryptionKey::Sodium(sk.0.into());
        
        // save
        let keypair = EncryptionKeypair { ek: wrapped_ek, dk: wrapped_dk };
        let id = EncryptionKeyId::random();
        self.put(&id.stringify(), &keypair);

        Ok(id)
    }
}

impl<K: Store> SignExport<EncryptionKeyId, Labeled<EncryptionKeyId, EncryptionKey>> for K {
    fn sign_export(&self, signer: &Agent, id: &EncryptionKeyId) -> SdaClientResult<Option<Signed<Labeled<EncryptionKeyId, EncryptionKey>>>> {
        // message
        let encryption_keypair: Option<EncryptionKeypair> = self.get(&id.stringify())?;
        let message_to_be_signed = match encryption_keypair {
            None => return Ok(None),
            Some(encryption_keypair) => {
                Labeled {
                    id: id.clone(),
                    body: encryption_keypair.ek,
                }
            }
        };
        // signature
        let signature_keypair: Option<VerificationKeypair> = self.get(&signer.verification_key.id.stringify())?;
        let signature = match signature_keypair {
            None => return Ok(None),
            Some(VerificationKeypair{ sk: SigningKey::Sodium(raw_sk), .. }) => {
                let sk = sodiumoxide::crypto::sign::SecretKey::from_slice(&*raw_sk).unwrap();
                let msg = &message_to_be_signed.canonical()?;
                let signature = sodiumoxide::crypto::sign::sign_detached(msg, &sk);
                Signature::Sodium(signature.0.into())
            }
        };
        // wrapper
        Ok(Some(Signed {
            signature: signature,
            signer: signer.id().clone(),
            body: message_to_be_signed,
        }))
    }
}





impl<K: Store> ExportDecryptionKey<EncryptionKeyId, (sda_protocol::EncryptionKey, crypto::DecryptionKey)> for K {
    fn export_decryption_key(&self, id: &EncryptionKeyId) -> SdaClientResult<Option<(sda_protocol::EncryptionKey, crypto::DecryptionKey)>> {
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

