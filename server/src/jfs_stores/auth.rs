use jfs;

use std::path;

use sda_protocol::AgentId;

use SdaServerResult;
use stores::{BaseStore, AuthStore, AuthToken};
use jfs_stores::JfsStoreExt;

pub struct JfsAuthStore {
    auth_tokens: jfs::Store,
}

impl JfsAuthStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsAuthStore> {
        let auth_tokens = prefix.as_ref().join("auth_tokens");
        Ok(JfsAuthStore {
            auth_tokens: jfs::Store::new(auth_tokens.to_str().ok_or("pathbuf to string")?)?,
        })
    }
}

impl BaseStore for JfsAuthStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl AuthStore for JfsAuthStore {
    fn upsert_auth_token(&self, token: &AuthToken) -> SdaServerResult<()> {
        self.auth_tokens.save_ident(token)
    }

    fn get_auth_token(&self, id: &AgentId) -> SdaServerResult<Option<AuthToken>> {
        self.auth_tokens.get_option(id)
    }

    fn delete_auth_token(&self, id: &AgentId) -> SdaServerResult<()> {
        println!("delete: {}", id.to_string());
        self.auth_tokens.delete(&*id.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;
    use sda_protocol::AgentId;
    use sda_protocol::Identified;
    use stores::{BaseStore, AuthStore, AuthToken};
    use super::JfsAuthStore;
    use jfs_stores::JfsStoreExt;

    #[test]
    fn delete() {
        let tmpdir = tempdir::TempDir::new("sda-server").unwrap();
        let token = AuthToken {
            id: AgentId::random(),
            body: "token".to_string(),
        };
        let store = JfsAuthStore::new(tmpdir.path()).unwrap();
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
