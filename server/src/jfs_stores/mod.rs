use jfs;

use errors::*;

mod agents;
mod auth;

pub use self::agents::JfsAgentStore;
pub use self::auth::JfsAuthStore;

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
