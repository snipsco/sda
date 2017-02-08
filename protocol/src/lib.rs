
//! This crate describes the common interface of SDA, including the operations
//! exposed by an SDA service and the message format used.
//!
//! As such it is lightweight crate referenced by most of the other (Rust) crates.
//!
//! It takes a REST approach whenever possible.

#[macro_use]
extern crate error_chain;

mod crypto;
pub use crypto::*;

mod errors {
    error_chain! {
        types {
            SdaError, SdaErrorKind, SdaResultExt, SdaResult;
        }
    }
}

pub use errors::*;


/// Identifies an aggregation.
pub struct Aggregation {
    pub id: String,
    pub title: String,
    pub vector_dimension: usize,
}

pub struct AggregationConfiguration {
    pub secret_sharing_scheme: LinearSecretSharingScheme,
    pub encryption_scheme: AdditiveEncryptionScheme,
}

/// Clerk public identity.
pub struct ClerkIdentity {
    pub id: String,
    pub name: String,
}

/// User public identity.
pub struct UserIdentity {
    pub id: String,
}

pub struct EncryptionKey;

/// Clerk public profile, including identity and cryptographic keys.
pub struct ClerkProfile {
    pub identity: ClerkIdentity,
    pub encryption_key: EncryptionKey,
}

/// Partial aggregation job to be performed by a clerk, including all inputs needed.
pub struct ClerkingJob {
    pub aggregation: Aggregation,
}

/// Result of a partial aggregation job performed by a clerk.
pub struct ClerkingResult {
    pub aggregation: Aggregation,
}

/// Description of an user's input to an aggregation.
pub struct Participation {
    pub aggregation: Aggregation,
}





/// Common trait for all SDA services.
pub trait SdaService {
    fn ping(&self) -> SdaResult<()>;
}

// TODO avoid naming that directly refers to REST to be abstract about implementation?
// TODO eg use `push` instead of `post`, `pull` instead of `get`, ...

/// TODO
pub trait SdaAggregationService : SdaService {

    /// Register clerk with the given profile and identity.
    fn post_clerk_profile(&self, profile: &ClerkProfile) -> SdaResult<Option<String>>;

    /// Pull any job waiting to be performed by the clerk.
    fn get_clerking_job(&self, clerk: &ClerkIdentity) -> SdaResult<Option<ClerkingJob>>;

    /// Push the result of a finished job.
    fn post_clerking_result(&self, clerk: &ClerkIdentity, result: &ClerkingResult) -> SdaResult<()>;

    /// Provide user input to an aggregation.
    fn post_user_participation(&self, user: &UserIdentity, participation: &Participation) -> SdaResult<()>;

}

/// Trait for opening, closing, reading, and removing aggregations.
pub trait SdaAdministrationService : SdaService {

    fn get_aggregations(&self, filter: Option<&str>) -> SdaResult<Vec<Aggregation>>;
    fn put_aggregation(&self, aggregation: &Aggregation) -> SdaResult<Aggregation>;
    fn delete_aggregation(&self, aggregation: &Aggregation) -> SdaResult<()>;

}