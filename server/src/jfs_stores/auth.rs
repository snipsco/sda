use jfs;

use std::path;

use sda_protocol::{Id, Identified, AgentId};

use SdaServerResult;
use stores::{BaseStore, AuthStore, AuthToken};

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
    fn upsert_auth_token(&self, token:&AuthToken) -> SdaServerResult<()> {
        self.auth_tokens.save_with_id(token, &token.id().stringify())?;
        Ok(())
    }

    fn get_auth_token(&self, id:&AgentId) -> SdaServerResult<Option<AuthToken>> {
        super::get_option(&self.auth_tokens, &id.stringify())
    }

    fn delete_auth_token(&self, id:&AgentId) -> SdaServerResult<()> {
        self.auth_tokens.delete(&id.stringify())?;
        Ok(())
    }
}

