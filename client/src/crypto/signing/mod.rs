use super::*;
use crypto::encryption::EncryptionKeypair;

use sda_client_store::Store;

use sodiumoxide;

trait Export<I, O> {
    fn export(&self, id: &I) -> SdaClientResult<Option<O>>;
}

pub trait SignExport<I, O>
    where O: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    fn sign_export(&self, signer: &Agent, id: &I) -> SdaClientResult<Option<Signed<O>>>;
}

pub trait SignatureVerification<O> {
    fn signature_is_valid(&self, object: &O) -> SdaClientResult<bool>;
}


#[derive(Debug, Serialize, Deserialize)]
struct SignatureKeypair {
    pub vk: VerificationKey,
    pub sk: SigningKey,
}


impl<K: Store> KeyGeneration<VerificationKeyId> for CryptoModule<K> {
    fn new_key(&self) -> SdaClientResult<VerificationKeyId> {
        // generate
        let (vk, sk) = sodiumoxide::crypto::sign::gen_keypair();
        let wrapped_vk = VerificationKey::Sodium(vk.0.into());
        let wrapped_sk = SigningKey::Sodium(sk.0.into());
        
        // save
        let keypair = SignatureKeypair { vk: wrapped_vk, sk: wrapped_sk };
        let id = VerificationKeyId::new();
        self.keystore.put(&id.stringify(), &keypair)?;

        Ok(id)
    }
}


impl<K: Store> KeyGeneration<Labeled<VerificationKeyId, VerificationKey>> for CryptoModule<K> {
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


impl<K: Store> Export<VerificationKeyId, VerificationKey> for CryptoModule<K> {
    fn export(&self, id: &VerificationKeyId) -> SdaClientResult<Option<VerificationKey>> {
        let keypair: Option<SignatureKeypair> = self.keystore.get(&id.stringify())?;
        match keypair {
            None => Ok(None),
            Some(keypair) => Ok(Some(keypair.vk))
        }
    }
}


impl<K: Store> SignExport<EncryptionKeyId, Labeled<EncryptionKeyId, EncryptionKey>> for CryptoModule<K> {
    fn sign_export(&self, signer: &Agent, id: &EncryptionKeyId) -> SdaClientResult<Option<Signed<Labeled<EncryptionKeyId, EncryptionKey>>>> {
        // message
        let encryption_keypair: Option<EncryptionKeypair> = self.keystore.get(&id.stringify())?;
        let message_to_be_signed = match encryption_keypair {
            None => { return Ok(None) },
            Some(encryption_keypair) => {
                Labeled {
                    id: id.clone(),
                    body: encryption_keypair.ek,
                }
            }
        };
        // signature
        let signature_keypair: Option<SignatureKeypair> = self.keystore.get(&signer.verification_key.id.stringify())?;
        let signature = match signature_keypair {
            None => { return Ok(None) },
            Some(SignatureKeypair{ sk: SigningKey::Sodium(raw_sk), .. }) => {
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


impl SignatureVerification<SignedEncryptionKey> for Agent {
    fn signature_is_valid(&self, signed_encryption_key: &SignedEncryptionKey) -> SdaClientResult<bool> {

        // TODO remember result to avoid running verification more than once

        let raw_msg = match &signed_encryption_key.body.body {
            &EncryptionKey::Sodium(raw_ek) => raw_ek
        };

        let wrapped_vk = &self.verification_key.body;
        let wrapped_sig = &signed_encryption_key.signature;

        match (wrapped_vk, wrapped_sig) {

            (&VerificationKey::Sodium(raw_vk), &Signature::Sodium(raw_sig)) => {
                let sig = sodiumoxide::crypto::sign::Signature(*raw_sig);
                let vk = sodiumoxide::crypto::sign::PublicKey(*raw_vk);
                let is_valid = sodiumoxide::crypto::sign::verify_detached(&sig, &*raw_msg, &vk);
                Ok(is_valid)
            },

        }
    }
}