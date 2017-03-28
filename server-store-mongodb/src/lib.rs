#[macro_use]
extern crate bson;
extern crate mongodb;
extern crate sda_protocol;
extern crate sda_server;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde::{Serialize, Deserialize};

use sda_protocol::*;
use sda_server::{SdaServer, SdaServerService};
use sda_server::errors::*;

macro_rules! m {
    ($e:expr) => {
        match $e {
            Ok(ok) => Ok(ok),
            Err(e) =>
                Err(SdaServerError::from(format!("Mongodb Error: {:?}", e)))
        }
    }
}

macro_rules! d {
    () => {{ ::bson::Document::new() }};
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

pub fn from_bson<T: Deserialize>(bson: ::bson::Bson) -> SdaServerResult<T> {
    bson::from_bson(bson).map_err(|e| format!("Error converting from bson: {}", e).into())
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
mod aggregations;
mod auth_tokens;
mod clerking_jobs;

pub fn new_mongodb_server_for_url(url: &str, db: &str) -> SdaResult<SdaServerService> {
    use mongodb::ThreadedClient;
    let client = mongodb::Client::with_uri(url)
        .map_err(|e| format!("could not build mongodb client for: {} ({:?})", url, e))?;
    new_mongodb_server(&client, db)
}

pub fn new_mongodb_server(client: &mongodb::Client, db: &str) -> SdaResult<SdaServerService> {
    use mongodb::ThreadedClient;
    let db = client.db(db);
    let agents = agents::MongoAgentsStore::new(&db).map_err(|e| format!("Error connecting to mongodb: {:?}", e))?;
    let auth = auth_tokens::MongoAuthTokensStore::new(&db).map_err(|e| format!("Error connecting to mongodb: {:?}", e))?;
    let agg = aggregations::MongoAggregationsStore::new(&db).map_err(|e| format!("Error connecting to mongodb: {:?}", e))?;
    let jobs = clerking_jobs::MongoClerkingJobsStore::new(&db).map_err(|e| format!("Error connecting to mongodb: {:?}", e))?;
    Ok(SdaServerService(SdaServer {
        agents_store: Box::new(agents),
        auth_tokens_store: Box::new(auth),
        aggregation_store: Box::new(agg),
        clerking_job_store: Box::new(jobs),
    }))
}

struct Dao<ID: Id, T: Serialize + Deserialize> {
    coll: mongodb::coll::Collection,
    _phantom: ::std::marker::PhantomData<(ID, T)>,
}

impl<ID: Id, T: Serialize + Deserialize> Dao<ID, T> {
    fn new(coll: mongodb::coll::Collection) -> Dao<ID, T> {
        Dao {
            coll: coll,
            _phantom: ::std::marker::PhantomData,
        }
    }

    fn ping(&self) -> SdaServerResult<()> {
        m!(self.coll.count(None, None))?;
        Ok(())
    }

    fn ensure_index(&self, spec: bson::Document, unique: bool) -> SdaServerResult<()> {
        use mongodb::coll::options::IndexOptions;
        m!(self.coll.create_index(spec,
                                  Some(IndexOptions {
                                      unique: Some(unique),
                                      background: Some(true),
                                      ..IndexOptions::default()
                                  })))?;
        Ok(())
    }

    fn get(&self, selector: bson::Document) -> SdaServerResult<Option<T>> {
        let option = m!(self.coll.find_one(Some(selector), None))?;
        if let Some(it) = option {
            Ok(Some(from_doc::<T>(it)?))
        } else {
            Ok(None)
        }
    }

    fn get_by_id(&self, id: &ID) -> SdaServerResult<Option<T>> {
        self.get(d!("id"=>m!(bson::to_bson(&id.to_string()))?))
    }

    fn find(&self, selector: bson::Document) -> SdaServerResult<DaoCursor<T>> {
        Ok(DaoCursor {
            cursor: m!(self.coll.find(Some(selector), None))?,
            _phantom: ::std::marker::PhantomData,
        })
    }

    fn modisert_by_id(&self, id: &ID, update: bson::Document) -> SdaServerResult<()> {
        let selector = d! { "id" => m!(bson::to_bson(id))? };
        m!(self.coll.update_one(selector,
                                update,
                                Some(::mongodb::coll::options::UpdateOptions {
                                    upsert: Some(true),
                                    write_concern: None,
                                })))?;
        Ok(())
    }

    fn modify_by_id(&self, id: &ID, update: bson::Document) -> SdaServerResult<()> {
        let selector = d! { "id" => m!(bson::to_bson(id))? };
        m!(self.coll.update_one(selector, update, None))?;
        Ok(())
    }
}

struct DaoCursor<T: Deserialize> {
    cursor: mongodb::cursor::Cursor,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<T: Deserialize> Iterator for DaoCursor<T> {
    type Item = SdaServerResult<T>;
    fn next(&mut self) -> Option<SdaServerResult<T>> {
        self.cursor.next().map(|res| m!(res).and_then(|doc| from_doc(doc)))
    }
}
