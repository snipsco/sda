use mongodb::coll::Collection;
use sda_protocol::*;
use sda_server::stores;
use sda_server::errors::*;
use {to_bson, to_doc, from_doc, Dao};

use sda_server::stores::AuthToken;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct AuthTokenDocument {
    id: AgentId,
    auth_token: AuthToken,
}

pub struct MongoAuthTokensStore(Dao<AgentId, AuthTokenDocument>);

impl MongoAuthTokensStore {
    pub fn new(db: &::mongodb::db::Database) -> SdaServerResult<MongoAuthTokensStore> {
        use mongodb::db::ThreadedDatabase;
        let dao = Dao::new(db.collection("auth_tokens"));
        dao.ensure_index(d!("id" => 1), true)?;
        Ok(MongoAuthTokensStore(dao))
    }
}

impl stores::BaseStore for MongoAuthTokensStore {
    fn ping(&self) -> SdaServerResult<()> {
        self.0.ping()
    }
}

impl stores::AuthTokensStore for MongoAuthTokensStore {
    fn upsert_auth_token(&self, token: &AuthToken) -> SdaServerResult<()> {
        self.0.modisert_by_id(&token.id, d!("$set" => d! ( "id" => to_bson(&token.id)?, "auth_token" => to_doc(&token)?) ))
    }

    fn get_auth_token(&self, id: &AgentId) -> SdaServerResult<Option<AuthToken>> {
        self.0
            .get_by_id(id)
            .map(|opt| opt.map(|ad| ad.auth_token))
    }

    fn delete_auth_token(&self, id: &AgentId) -> SdaServerResult<()> {
        m!(self.0.coll.delete_one(d!("id" => m!(to_bson(id))?), None))?;
        Ok(())
    }
}
