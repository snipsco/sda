
use super::*;
use std::collections::HashMap;

pub struct ParticipantInput(pub Vec<i64>);
pub struct AuthToken(pub String);

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)] // TODO could we use Copy instead?
pub struct AgentId(pub Uuid);

#[derive(Debug)]
pub struct ParticipationId(pub Uuid);

#[derive(Debug)]
pub struct SignedEncryptionKeyId(pub Uuid);

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct KeysetId(pub Uuid);

#[derive(PartialEq, Eq)]
pub struct CommitteeId(pub Uuid);

#[derive(Clone, Debug)] // TODO could we use Copy instead?
pub struct AggregationId(pub Uuid);

/// Identifies any agent accessing the SDA service (including users, clerks, and admins).
pub struct Agent {
    pub id: AgentId,
    pub auth_token: Option<AuthToken>,
}

pub struct Committee {
    pub id: CommitteeId,
    pub name: Option<String>,
    pub clerks: Vec<AgentId>,
}

pub struct SignedEncryptionKey {
    pub id: SignedEncryptionKeyId,
    pub owner: AgentId,
    pub key: EncryptionKey,
    pub signature: Signature,
}

pub struct Aggregation {
    pub id: AggregationId,
    pub title: String,
    pub vector_dimension: usize,
    // pub modulus: i64,  // TODO move here instead of in the primitives?
    pub recipient: AgentId,
    pub committee: CommitteeId,

    /// Note that this could simply be a vector, but it's easier to work with a map for both participants and clerks
    pub keyset: HashMap<AgentId, SignedEncryptionKeyId>,
    pub masking_scheme: LinearMaskingScheme,
    pub committee_sharing_scheme: LinearSecretSharingScheme,
    pub recipient_encryption_scheme: AdditiveEncryptionScheme,
    pub committee_encryption_scheme: AdditiveEncryptionScheme,
}

/// Description of an user's input to an aggregation.
#[derive(Debug)]
pub struct Participation {
    pub id: ParticipationId,
    pub participant: AgentId,
    pub aggregation: AggregationId,
    pub encryptions: HashMap<AgentId, Encryption>,
}

pub struct AggregationStatus {
    pub aggregation: AggregationId,
    /// Current number of participations received from the users.
    pub number_of_participations: usize,
    /// Current number of clerking results received from the clerks.
    pub number_of_clerking_results: usize,
    /// Indication of whether a result of the aggregation can be produced from the current data.
    pub result_ready: bool,
}

pub struct AggregationResult {
    pub aggregation: AggregationId,
    /// Number of participation used in this result.
    pub number_of_participations: usize,
    /// Number of clerking results used in this result.
    pub number_of_clerking_results: usize,
    /// Result of the aggregation.
    pub output: Vec<i64>,
}

/// Public profile of a clerk, including identity, cryptographic keys, name, social handles, etc.
#[derive(Clone, Default)]
pub struct Profile {
    pub owner: AgentId,
    pub name: Option<String>,
    pub verification_key: VerificationKey,
    pub twitter_id: Option<String>,
    pub keybase_id: Option<String>,
    pub website: Option<String>,
}

/// Partial aggregation job to be performed by a clerk, including all inputs needed.
pub struct ClerkingJob {
    pub aggregation: AggregationId,
    pub encryptions: Vec<Encryption>,
}

/// Result of a partial aggregation job performed by a clerk.
pub struct ClerkingResult {
    pub clerk: AgentId,
    pub aggregation: AggregationId,
    pub encryption: Encryption,
}
