use super::*;

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

uuid_id!{ #[doc="Unique encryption key identifier."] EncryptionKeyId }

pub type SignedEncryptionKey = Signed<Labeled<EncryptionKeyId, EncryptionKey>>;

/// Description of an aggregation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aggregation {
    pub id: AggregationId,
    pub title: String,
    /// Fixed dimension of input and output vectors.
    pub vector_dimension: usize,
    /// The group in which all operations are performed.
    ///
    /// Cryptographic primitives must match with this value.
    pub modulus: i64,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub recipient_encryption: Option<Encryption>,
    pub clerk_encryptions: Vec<(AgentId, Encryption)>,
}

uuid_id!{ #[doc="Unique participation identifier."] ParticipationId }
identify!(Participation, ParticipationId);

/// Capture existing participations in an agggregation in order to create a
/// consistent set of clerkable shares.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub aggregation: AggregationId,
}

uuid_id!{ #[doc="Unique snapshot identifier."] SnapshotId }
identify!(Snapshot, SnapshotId);

/// Partial aggregation job to be performed by a clerk.
///
/// Includes all inputs needed.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkingJob {
    pub id: ClerkingJobId,
    pub clerk: AgentId,
    pub aggregation: AggregationId,
    pub snapshot: SnapshotId,
    pub encryptions: Vec<Encryption>,
}

uuid_id!{ #[doc="Unique job identifier."] ClerkingJobId }
identify!(ClerkingJob, ClerkingJobId);

/// Result of a partial aggregation job performed by a clerk.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkingResult {
    pub job: ClerkingJobId,
    pub clerk: AgentId,
    pub encryption: Encryption,
}

/// Current status of an aggregation.
#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationStatus {
    pub aggregation: AggregationId,
    /// Current number of participations received from the users.
    pub number_of_participations: usize,
    /// Snapshot and their status for this aggregation
    pub snapshots: Vec<SnapshotStatus>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SnapshotStatus {
    /// Snapshot id
    pub id: SnapshotId,
    /// Current number of clerking results received from the clerks.
    pub number_of_clerking_results: usize,
    /// Indication of whether a result of the aggregation can be produced from the current clerking results.
    pub result_ready: bool,
}

/// Result of an aggregation snapshot, including output, ready for
/// reconstruction.
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotResult {
    pub snapshot: SnapshotId,
    /// Number of participation used in this result.
    pub number_of_participations: usize,
    /// Result of the aggregation.
    pub clerk_encryptions: Vec<ClerkingResult>,
    /// Optional mask
    pub recipient_encryptions: Option<Vec<Encryption>>,
}
