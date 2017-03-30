use super::*;

uuid_id!{ #[doc="Unique verification key identifier."] VerificationKeyId }

/// Verification key and its associated id.
pub type LabelledVerificationKey = Labelled<VerificationKeyId, VerificationKey>;

/// Fundamental description of agents in the system, i.e. participants, clerks, recipients, and admins.
///
/// Primary use is identification, including allowing services to perform access control and logging.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier of agent.
    pub id: AgentId,
    /// Key used for verifying signed resources from agent.
    pub verification_key: LabelledVerificationKey,
}

uuid_id!{ #[doc="Unique agent identifier."] AgentId }
identify!(Agent,AgentId);

/// Extended profile of an agent, providing information intended for increasing trust such as name and social handles.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    /// Agent to which profile belongs.
    pub owner: AgentId,
    /// Name or alias.
    pub name: Option<String>,
    /// Twitter handle.
    pub twitter_id: Option<String>,
    /// Keybase handle.
    pub keybase_id: Option<String>,
    /// Public website.
    pub website: Option<String>,
}

uuid_id!{ #[doc="Unique encryption key identifier."] EncryptionKeyId }

/// Encryption key with its associated, signed by the owner of the corresponding keypair.
pub type SignedEncryptionKey = Signed<Labelled<EncryptionKeyId, EncryptionKey>>;

/// Description of an aggregation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aggregation {
    /// Unique identifier of aggregation.
    pub id: AggregationId,
    /// Title, e.g. explaining purpose of the aggregation.
    pub title: String,
    /// Fixed dimension of input and output vectors.
    pub vector_dimension: usize,
    /// The group in which all operations are performed.
    ///
    /// Cryptographic primitives must be compatible with this value.
    pub modulus: i64,
    /// Recipient of output vector.
    pub recipient: AgentId,
    /// Encryption key to be used for encryptions intended for the recipient.
    pub recipient_key: EncryptionKeyId,
    /// Scheme and parameters used for masking secrets between the recipient and the committee.
    pub masking_scheme: LinearMaskingScheme,
    /// Scheme and parameters used for sharing masked secrets between the clerks in the committee.
    pub committee_sharing_scheme: LinearSecretSharingScheme,
    /// Scheme and parameters used for encrypting masks for the recipient.
    pub recipient_encryption_scheme: AdditiveEncryptionScheme,
    /// Scheme and parameters used for encrypting shares of masked secrets for the committee.
    pub committee_encryption_scheme: AdditiveEncryptionScheme,
}

uuid_id!{ #[doc="Unique aggregation identifier."] AggregationId }
identify!(Aggregation, AggregationId);

/// Suggested clerk for a given aggregation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClerkCandidate {
    /// Clerk identifier.
    pub id: AgentId,
    /// Available and matching encryption keys for candidate.
    pub keys: Vec<EncryptionKeyId>,
}

/// Description of committee elected for an aggregation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Committee {
    /// Aggregation identifier.
    pub aggregation: AggregationId,
    /// Clerks in the committee, with corresponding encryption key to use for encrypting messages for each.
    pub clerks_and_keys: Vec<(AgentId, EncryptionKeyId)>,
}

/// Description of a participant's input to an aggregation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Participation {
    /// Unique identifier of this participation.
    ///
    /// This allows a service to keep track, and possible discard, participations that where sent several times,
    /// for instance as a result of retries due to communication errors.
    pub id: ParticipationId,
    /// Participant identifier.
    ///
    /// This allows a service to keep track, and possible discard, multiple participations from each participant.
    pub participant: AgentId,
    /// Aggregation identifier.
    pub aggregation: AggregationId,
    /// Encryption intended for recipient.
    pub recipient_encryption: Option<Encryption>,
    /// Encryptions intended for the clerks in the committee.
    pub clerk_encryptions: Vec<(AgentId, Encryption)>,
}

uuid_id!{ #[doc="Unique participation identifier."] ParticipationId }
identify!(Participation, ParticipationId);

/// Captures a subset of the current participations to an agggregation in order to 
/// create a consistent set of clerkable shares.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    /// Unique identifiers.
    pub id: SnapshotId,
    /// Associated aggregation.
    pub aggregation: AggregationId,
}

uuid_id!{ #[doc="Unique snapshot identifier."] SnapshotId }
identify!(Snapshot, SnapshotId);

/// Partial aggregation job to be performed by a clerk.
#[derive(Clone, Debug, PartialEq,  Serialize, Deserialize)]
pub struct ClerkingJob {
    /// Unique identifier.
    pub id: ClerkingJobId,
    /// Intended clerk.
    pub clerk: AgentId,
    /// Associated aggregation.
    pub aggregation: AggregationId,
    /// Associated snapshot of aggregation.
    pub snapshot: SnapshotId,
    /// Encryptions containing shares for clerking.
    pub encryptions: Vec<Encryption>,
}

uuid_id!{ #[doc="Unique job identifier."] ClerkingJobId }
identify!(ClerkingJob, ClerkingJobId);

/// Result of a partial aggregation job performed by a clerk.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClerkingResult {
    /// Associated clerking job.
    pub job: ClerkingJobId,
    /// Executing clerk.
    pub clerk: AgentId,
    /// Encryption of combined shares. 
    pub encryption: Encryption,
}

/// Current status of an aggregation.
#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationStatus {
    /// Associated aggregation.
    pub aggregation: AggregationId,
    /// Current number of participations received.
    pub number_of_participations: usize,
    /// Associated anapshots and their status for this aggregation.
    pub snapshots: Vec<SnapshotStatus>
}

/// Current status of a snapshot.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SnapshotStatus {
    /// Unique identifier.
    pub id: SnapshotId,
    /// Current number of clerking results received.
    pub number_of_clerking_results: usize,
    /// Indication of whether a result of the aggregation can be produced from the current clerking results.
    pub result_ready: bool,
}

/// Result of an aggregation snapshot, including output, ready for reconstruction.
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotResult {
    /// Associated snapshot.
    pub snapshot: SnapshotId,
    /// Number of participations used in this result.
    pub number_of_participations: usize,
    /// Encrypted shares of the masked result.
    pub clerk_encryptions: Vec<ClerkingResult>,
    /// Encrypted mask for the result.
    pub recipient_encryptions: Option<Vec<Encryption>>,
}
