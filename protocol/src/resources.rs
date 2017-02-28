


use super::*;
use std::collections::HashMap;


/// Basic description of an agent, e.g. participants, clerks, and admins.
///
/// Primary use is identification, including allowing services to perform access control and logging.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    /// Key used for verifying signatures from agent, if any.
    pub verification_key: Option<VerificationKey>,
}

/// Unique agent identifier.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)] // TODO could we use Copy instead?
pub struct AgentId(pub Uuid);

impl Default for AgentId {
    fn default() -> AgentId {
        AgentId(Uuid::new(::uuid::UuidVersion::Random).unwrap())
    }
}


/// Extended profile of an agent, providing information intended for increasing trust such as name and social handles.
#[derive(Clone, Default)]
pub struct Profile {
    pub owner: AgentId,
    pub name: Option<String>,
    pub twitter_id: Option<String>,
    pub keybase_id: Option<String>,
    pub website: Option<String>,
}

/// Encryption key signed by owner.
pub struct SignedEncryptionKey {
    pub id: SignedEncryptionKeyId,
    pub owner: AgentId,
    pub key: EncryptionKey,
    pub signature: Signature,
}

/// Unique signed encryption key identifier.
#[derive(Debug)]
pub struct SignedEncryptionKeyId(pub Uuid);


/// Description of an aggregation.
pub struct Aggregation {
    pub id: AggregationId,
    pub title: String,
    /// Fixed dimension of input and output vectors.
    pub vector_dimension: usize,
    // pub modulus: i64,  // TODO move here instead of in the primitives?
    /// Recipient of output vector.
    pub recipient: AgentId,
    /// Associated committee.
    pub committee: CommitteeId,
    /// Encryption keys of to be used for the recipient and committee.
    ///
    /// Note that while this could simply be a vector, it's easier to work with a map.
    pub keyset: HashMap<AgentId, SignedEncryptionKeyId>,
    /// Masking scheme and parameters to be used between the recipient and the committee.
    pub masking_scheme: LinearMaskingScheme,
    /// Scheme and parameters to be used for secret sharing between the clerks in the committee.
    pub committee_sharing_scheme: LinearSecretSharingScheme,
    /// Scheme and parameters to be used for encrypting masks for the recipient.
    pub recipient_encryption_scheme: AdditiveEncryptionScheme,
    /// Scheme and parameters to be used for encryption masked shares for the committee.
    pub committee_encryption_scheme: AdditiveEncryptionScheme,
}

/// Unique aggregation identifier.
#[derive(Clone, Debug)] // TODO could we use Copy instead?
pub struct AggregationId(pub Uuid);

/// Description of committee elected for one or more aggregations.
///
/// Having this as a separate object allows for reuse of trusted committees.
pub struct Committee {
    pub id: CommitteeId,
    pub name: Option<String>,
    pub clerks: Vec<AgentId>,
}

/// Unique committee identifier.
#[derive(PartialEq, Eq)]
pub struct CommitteeId(pub Uuid);

/// Description of a participant's input to an aggregation.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ParticipationId(pub Uuid);

/// Partial aggregation job to be performed by a clerk.
///
/// Includes all inputs needed.
pub struct ClerkingJob {
    pub id: ClerkingJobId,
    pub clerk: AgentId,
    pub aggregation: AggregationId,
    pub encryptions: Vec<Encryption>,
}

/// Result of a partial aggregation job performed by a clerk.
pub struct ClerkingResult {
    pub job: ClerkingJobId,
    pub aggregation: AggregationId,
    pub encryption: Encryption,
}

#[derive(Clone)]
pub struct ClerkingJobId(pub Uuid);

/// Current status of an aggregation.
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
pub struct AggregationResult {
    pub aggregation: AggregationId,
    /// Number of participation used in this result.
    pub number_of_participations: usize,
    /// Number of clerking results used in this result.
    pub number_of_clerking_results: usize,
    /// Result of the aggregation.
    pub encryptions: Vec<Encryption>,
}
