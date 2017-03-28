use jfs;

use std::path;

use sda_protocol::AgentId;

use SdaServerResult;
use stores::{BaseStore, AuthTokensStore, AuthToken};
use jfs_stores::JfsStoreExt;

pub struct JfsAuthTokensStore {
    auth_tokens: jfs::Store,
}

impl JfsAuthTokensStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsAuthTokensStore> {
        let auth_tokens = prefix.as_ref().join("auth_tokens");
        Ok(JfsAuthTokensStore {
            auth_tokens: jfs::Store::new(auth_tokens.to_str().ok_or("pathbuf to string")?)?,
        })
    }
}

impl BaseStore for JfsAuthTokensStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl AuthTokensStore for JfsAuthTokensStore {
    fn upsert_auth_token(&self, token: &AuthToken) -> SdaServerResult<()> {
        self.auth_tokens.upsert(token)
    }

    fn get_auth_token(&self, id: &AgentId) -> SdaServerResult<Option<AuthToken>> {
        self.auth_tokens.get_option(id)
    }

    fn delete_auth_token(&self, id: &AgentId) -> SdaServerResult<()> {
        self.auth_tokens.delete(&*id.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;
    use sda_protocol::AgentId;
    use sda_protocol::Identified;
    use stores::{AuthTokensStore, AuthToken};
    use super::JfsAuthTokensStore;

    #[test]
    fn delete() {
        let tmpdir = tempdir::TempDir::new("sda-server").unwrap();
        let token = AuthToken {
            id: AgentId::random(),
            body: "token".to_string(),
        };
        let store = JfsAuthTokensStore::new(tmpdir.path()).unwrap();
        store.upsert_auth_token(&token).unwrap();
        store.get_auth_token(&token.id()).unwrap().unwrap();
        store.delete_auth_token(&token.id()).unwrap();
    }

    #[test]
    fn delete_raw() {
        let tmpdir = tempdir::TempDir::new("sda-server").unwrap();
        let token = AuthToken {
            id: AgentId::random(),
            body: "token".to_string(),
        };
        let store = ::jfs::Store::new(tmpdir.path().to_str().unwrap()).unwrap();
        store.save_with_id(&token, "foo").unwrap();
        store.get::<AuthToken>("foo").unwrap();
        store.delete("foo").unwrap();
    }
}
