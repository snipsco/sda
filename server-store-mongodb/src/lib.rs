#[macro_use]
extern crate bson;
extern crate mongodb;
extern crate sda_protocol;
extern crate sda_server;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde::{ Serialize, Deserialize };

use sda_protocol::*;
use sda_server::{SdaServer, SdaServerService};
use sda_server::errors::*;

macro_rules! m {
    ($e:expr) => {
        match $e {
            Ok(ok) => Ok(ok),
            Err(e) =>
                Err(SdaServerError::from(format!("Mongodb Error: {}", e)))
        }
    }
}

macro_rules! d {
    () => {{ $crate::Document::new() }};
    ( $($key:expr => $val:expr),* ) => {{
        let mut document = ::bson::Document::new();

        $(
            document.insert_bson($key.to_owned(), bson!($val));
        )*

        document
    }};
}

pub fn to_bson<T: Serialize>(t: &T) -> SdaServerResult<::bson::Bson> {
    bson::to_bson(t).map_err(|e| format!("Error converting to bson: {}", e).into())
}

pub fn to_doc<T: Serialize>(t: &T) -> SdaServerResult<::bson::Document> {
    match m!(::bson::to_bson(t))? {
        ::bson::Bson::Document(d) => Ok(d),
        e => Err(format!("expected a doc, found {:?}", e).into()),
    }

}

pub fn from_doc<T: Deserialize>(doc: ::bson::Document) -> SdaServerResult<T> {
    Ok(m!(::bson::from_bson(::bson::Bson::Document(doc)))?)
}

mod agents;

pub fn new_mongodb_server<P: AsRef<::std::path::Path>>(client: &mongodb::Client,
                                                       db: &str,
                                                       dir: P)
                                                       -> SdaResult<SdaServerService> {
    use mongodb::ThreadedClient;
    let dir = dir.as_ref();
    let db = client.db(db);
    let agents = agents::MongoAgentsStore::new(&db).unwrap();
    let auth = sda_server::jfs_stores::JfsAuthTokensStore::new(dir.join("auths")).unwrap();
    let agg = sda_server::jfs_stores::JfsAggregationsStore::new(dir.join("agg")).unwrap();
    let jobs = sda_server::jfs_stores::JfsClerkingJobStore::new(dir.join("jobs")).unwrap();
    Ok(SdaServerService(SdaServer {
        agents_store: Box::new(agents),
        auth_tokens_store: Box::new(auth),
        aggregation_store: Box::new(agg),
        clerking_job_store: Box::new(jobs),
    }))
}

trait CollectionExt {
    fn get_option<T:Deserialize>(&self, selector: bson::Document) -> SdaServerResult<Option<T>>;
    fn get_option_by_id<T:Deserialize, ID: Id>(&self, id: &ID) -> SdaServerResult<Option<T>>;
    fn modisert_by_id<ID: Id>(&self, id: &ID, update: bson::Document) -> SdaServerResult<()>;
    fn modify_by_id<ID: Id>(&self, id: &ID, update: bson::Document) -> SdaServerResult<()>;
}

impl CollectionExt for mongodb::coll::Collection {

    fn get_option<T:Deserialize>(&self, selector: bson::Document) -> SdaServerResult<Option<T>> {
        let option = m!(self.find_one(Some(selector), None))?;
        if let Some(it) = option {
            Ok(Some(from_doc::<T>(it)?))
        } else {
            Ok(None)
        }
    }

    fn get_option_by_id<T:Deserialize, ID: Id>(&self, id: &ID) -> SdaServerResult<Option<T>> {
        self.get_option(d!("_id"=>m!(bson::to_bson(&id.to_string()))?))
    }

    fn modisert_by_id<ID: Id>(&self, id: &ID, update: bson::Document) -> SdaServerResult<()> {
        let selector = d! { "_id" => m!(bson::to_bson(id))? };
        m!(self.update_one(selector,
                             update,
                             Some(::mongodb::coll::options::UpdateOptions {
                                 upsert: Some(true),
                                 write_concern: None,
                             })))?;
        Ok(())
    }

    fn modify_by_id<ID: Id>(&self, id: &ID, update: bson::Document) -> SdaServerResult<()> {
        let selector = d! { "_id" => m!(bson::to_bson(id))? };
        m!(self.update_one(selector, update, None))?;
        Ok(())
    }
}
