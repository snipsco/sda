
use super::*;
use std::sync::{Once, ONCE_INIT};
use integer_encoding::VarInt;


static SODIUM_INITIALIZED: Once = ONCE_INIT;


pub struct Encryptor {
    pk: sodiumoxide::crypto::box_::PublicKey,
}

impl Encryptor {
    pub fn new(ek: &EncryptionKey) -> SdaClientResult<Encryptor> {
        // initialise Sodium per recommendations; 
        //  - documentation hints it's okay to do so more than once but we'll play it safe
        SODIUM_INITIALIZED.call_once(|| { sodiumoxide::init(); });

        match ek {

            &EncryptionKey::Sodium(raw_ek) => {
                let pk = sodiumoxide::crypto::box_::PublicKey::from_slice(&raw_ek)
                    .ok_or("Failed to parse Sodium public key")?;
                Ok(Encryptor {
                    pk: pk,
                })
            },

            _ => Err("Wrong key type for this encryptor")?,

        }
    }
}

impl ShareEncryptor for Encryptor {
    fn encrypt(&self, shares: &[Share]) -> SdaClientResult<Encryption> {
        // encode
        let mut encoded_shares = vec![];
        let mut buf = [0; 128];
        for &share in shares {
            let size = share.encode_var(&mut buf);
            encoded_shares.extend(&buf[0..size]);
        }
        // encrypt
        let raw_data = sodiumoxide::crypto::sealedbox::seal(&*encoded_shares, &self.pk);
        Ok(Encryption::Sodium(raw_data))
    }
}


pub struct Decryptor {
    pk: sodiumoxide::crypto::box_::PublicKey,
    sk: sodiumoxide::crypto::box_::SecretKey,
}

impl Decryptor {
    pub fn new<I>(ek: &EncryptionKey, identity: &I) -> SdaClientResult<Decryptor>
        where I: ExportDecryptionKey
    {
        let dk = &identity.export_decryption_key(ek)?;
        match (ek, dk) {

            (&EncryptionKey::Sodium(raw_ek), &DecryptionKey::Sodium(raw_dk)) => {

                let pk = sodiumoxide::crypto::box_::PublicKey::from_slice(&raw_ek)
                    .ok_or("Failed to parse Sodium public key")?;

                let sk = sodiumoxide::crypto::box_::SecretKey::from_slice(&raw_dk)
                    .ok_or("Failed to parse Sodium secret key")?;
                
                Ok(Decryptor {
                    pk: pk,
                    sk: sk,
                })
            },

            _ => Err("Wrong key type(s) for this decryptor")?,

        }
    }
}

impl ShareDecryptor for Decryptor {
    fn decrypt(&self, encryption: &Encryption) -> SdaClientResult<Vec<Share>> {
        let encryption = match encryption {
            &Encryption::Sodium(ref raw) => raw,
            _ => Err("Cannot decrypt this type of encryption")?,
        };
        // decrypt
        let result = sodiumoxide::crypto::sealedbox::open(&encryption[..], &self.pk, &self.sk);
        // TODO better way of doing this?
        let raw_data = if result.is_ok() {
            Ok(result.unwrap())
        } else {
            Err("Sodium decryption failure")
        }?;
        // decode
        let mut reader = &raw_data[..];
        let mut decoded_shares = vec![];
        while reader.len() > 0 {
            let (i, size) = Share::decode_var(reader);
            decoded_shares.push(i);
            reader = &reader[size..];
        }
        Ok(decoded_shares)
    }
}
