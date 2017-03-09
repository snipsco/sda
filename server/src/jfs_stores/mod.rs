use jfs;

use errors::*;

mod agents;
mod aggregations;
mod auth;
mod clerking_jobs;

pub use self::agents::JfsAgentStore;
pub use self::auth::JfsAuthStore;
pub use self::aggregations::JfsAggregationsStore;
pub use self::clerking_jobs::JfsClerkingJobStore;

fn get_option<T>(store: &jfs::Store, id: &str) -> SdaServerResult<Option<T>>
    where T: ::serde::Serialize + ::serde::Deserialize
{
    match store.get(id) {
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
