use super::*;
use std::collections::HashMap;
use std::str;

pub trait Identified {
    type I : Id;
    fn id(&self) -> &Self::I;
}

pub trait Id: Sized + str::FromStr {
    fn stringify(&self) -> String;
    #[deprecated]
    fn destringify(&str) -> SdaResult<Self>;
}

macro_rules! uuid_id {
    ( #[$doc:meta] $name:ident ) => {
        #[$doc]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn random() -> $name {
                $name(Uuid::new(::uuid::UuidVersion::Random).unwrap())
            }
        }

        impl Default for $name {
            fn default() -> $name {
                $name::random()
            }
        }

        impl str::FromStr for $name {
            type Err=String;
            fn from_str(s: &str) -> std::result::Result<$name, String> {
                let uuid = Uuid::parse_str(s).map_err(|_| format!("unparseable uuid {}", s))?;
                Ok($name(uuid))
            }
        }

        impl Id for $name {
            fn stringify(&self) -> String {
                self.0.to_string()
            }
            fn destringify(s:&str) -> SdaResult<$name> {
                use std::str::FromStr;
                Ok($name::from_str(s)?)
            }
        }
    }
}

uuid_id!{ #[doc="Unique verification key identifier."] VerificationKeyId }
pub type LabeledVerificationKey = Labeled<VerificationKeyId, VerificationKey>;

/// Basic description of an agent, e.g. participants, clerks, and admins.
///
/// Primary use is identification, including allowing services to perform access control and logging.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    /// Key used for verifying signatures from agent.
    pub verification_key: LabeledVerificationKey,
}

uuid_id!{ #[doc="Unique agent identifier."] AgentId }

macro_rules! identify {
    ($object:ident, $id:ident) => {
        impl Identified for $object {
            type I = $id;
            fn id(&self) -> &$id {
                &self.id
            }
        }
    }
}
identify!(Agent,AgentId);

/// Extended profile of an agent, providing information intended for increasing trust such as name and social handles.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub owner: AgentId,
    pub name: Option<String>,
    pub twitter_id: Option<String>,
    pub keybase_id: Option<String>,
    pub website: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signed<M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    pub signature: Signature,
    pub signer: AgentId,
    pub body: M
}

impl<M> std::ops::Deref for Signed<M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type Target = M;
    fn deref(&self) -> &M {
        &self.body
    }
}

pub trait Sign {
    fn canonical(&self) -> SdaResult<Vec<u8>>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Labeled<ID,M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
      ID: Id + Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
{
    pub id: ID,
    pub body: M
}

impl<ID,M> Identified for Labeled<ID,M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
      ID: Id + Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type I = ID;
    fn id(&self) -> &ID {
        &self.id
    }
}

uuid_id!{ #[doc="Unique encryption key identifier."] EncryptionKeyId }

pub type SignedEncryptionKey = Signed<Labeled<EncryptionKeyId, EncryptionKey>>;

/// Description of an aggregation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aggregation {
    pub id: AggregationId,
    pub title: String,
    /// Fixed dimension of input and output vectors.
    pub vector_dimension: usize,
    // pub modulus: i64,  // TODO move here instead of in the primitives?
    /// Recipient of output vector.
    pub recipient: AgentId,
    /// Encryption key to be used for encryptions to the recipient.
    pub recipient_key: EncryptionKeyId,
    /// Masking scheme and parameters to be used between the recipient and the committee.
    pub masking_scheme: LinearMaskingScheme,
    /// Scheme and parameters to be used for secret sharing between the clerks in the committee.
    pub committee_sharing_scheme: LinearSecretSharingScheme,
    /// Scheme and parameters to be used for encrypting masks for the recipient.
    pub recipient_encryption_scheme: AdditiveEncryptionScheme,
    /// Scheme and parameters to be used for encryption masked shares for the committee.
    pub committee_encryption_scheme: AdditiveEncryptionScheme,
}

uuid_id!{ #[doc="Unique aggregation identifier."] AggregationId }
identify!(Aggregation, AggregationId);

/// Description of committee elected for an aggregation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClerkCandidate {
    /// Candidate clerk agent identifier
    pub id: AgentId,
    /// Candidate clerk possible encryption keys
    pub keys: Vec<EncryptionKeyId>,
}

/// Description of committee elected for an aggregation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Committee {
    pub aggregation: AggregationId,
    /// Clerks in the committee, with the EncryptionKeyId to use.
    pub clerks_and_keys: Vec<(AgentId, EncryptionKeyId)>,
}

/// Description of a participant's input to an aggregation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Participation {
    /// Unique identifier of participation.
    ///
    /// This allows a service to keep track, and possible discard, multiple participations from each participant.
    pub id: ParticipationId,
    pub participant: AgentId,
    pub aggregation: AggregationId,
    pub encryptions: HashMap<AgentId, Encryption>,
}

/// Unique participatin identifer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipationId(pub Uuid);

/// Partial aggregation job to be performed by a clerk.
///
/// Includes all inputs needed.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkingJob {
    pub id: ClerkingJobId,
    pub clerk: AgentId,
    pub aggregation: AggregationId,
    pub encryptions: Vec<Encryption>,
}

/// Result of a partial aggregation job performed by a clerk.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkingResult {
    pub job: ClerkingJobId,
    pub aggregation: AggregationId,
    pub encryption: Encryption,
}

uuid_id!{ #[doc="Unique job identifier."] ClerkingJobId }

/// Current status of an aggregation.
#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationStatus {
    pub aggregation: AggregationId,
    /// Current number of participations received from the users.
    pub number_of_participations: usize,
    /// Current number of clerking results received from the clerks.
    pub number_of_clerking_results: usize,
    /// Indication of whether a result of the aggregation can be produced from the current clerking results.
    pub result_ready: bool,
}

/// Result of an aggregation, including output.
#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationResult {
    pub aggregation: AggregationId,
    /// Number of participation used in this result.
    pub number_of_participations: usize,
    /// Number of clerking results used in this result.
    pub number_of_clerking_results: usize,
    /// Result of the aggregation.
    pub encryptions: Vec<Encryption>,
}
