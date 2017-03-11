use jfs;

use sda_protocol::{Id, Identified};

use errors::*;


mod agents;
mod aggregations;
mod auth;
mod clerking_jobs;

pub use self::agents::JfsAgentStore;
pub use self::auth::JfsAuthStore;
pub use self::aggregations::JfsAggregationsStore;
pub use self::clerking_jobs::JfsClerkingJobStore;

trait JfsStoreExt {
    fn get_option_for_str<T, S>(&self, id: S) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              S: AsRef<str>;

    fn get_option<T, I>(&self, id: &I) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              I: Id;

    fn save_at<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize,
              I: Id;

    fn save_ident<T>(&self, it: &T) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + Identified;
}

impl JfsStoreExt for jfs::Store {
    fn get_option_for_str<T, S>(&self, id: S) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              S: AsRef<str>
    {
        match self.get(id.as_ref()) {
            Ok(it) => Ok(Some(it)),
            Err(io) => {
                if io.kind() == ::std::io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(io)?
                }
            }
        }
    }

    fn get_option<T, I>(&self, id: &I) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              I: Id
    {
        self.get_option_for_str(id.to_string())
    }

    fn save_at<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize,
              I: Id
    {
        self.save_with_id(it, &*id.to_string())?;
        Ok(())
    }

    fn save_ident<T>(&self, it: &T) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + Identified
    {
        self.save_at(it, it.id())
    }
}
