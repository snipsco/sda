use rand::{Rng, OsRng};
use sda_client_store::{Store, SdaClientStoreResult};

pub trait TokenStore {
    fn get(&self) -> SdaClientStoreResult<String>;
}

impl<S: Store> TokenStore for S {
    fn get(&self) -> SdaClientStoreResult<String> {
        match self.get("auth_token")? {
            Some(existing) => Ok(existing),
            None => {
                let mut rng = OsRng::new().unwrap();
                let new_token = rng
                    .gen_ascii_chars()
                    .take(32)
                    .collect::<String>();
                self.put("auth_token", &new_token)?;
                Ok(new_token)
            }
        }
    }
}