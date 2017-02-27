
use super::*;

impl Agent {
    pub fn new() -> Agent {
        Agent {
            id: AgentId::new(),
            verification_key: None,
            profile: None,
        }
    }
}

impl AgentId {
    pub fn new() -> AgentId {
        AgentId(Uuid::new_v4())
    }
}

impl ParticipationId {
    pub fn new() -> ParticipationId {
        ParticipationId(Uuid::new_v4())
    }
}

impl Default for VerificationKey {
    fn default() -> Self {
        VerificationKey::Sodium([0; 32])
    }
}

impl Clone for SigningKey {
    fn clone(&self) -> Self {
        match self {
            &SigningKey::Sodium(raw_sk) => SigningKey::Sodium(raw_sk)
        }
    }
}

impl Clone for VerificationKey {
    fn clone(&self) -> Self {
        match self {
            &VerificationKey::Sodium(raw_vk) => VerificationKey::Sodium(raw_vk)
        }
    }
}

// impl From<&'static str> for AgentId {
//     fn from(s: &'static str) -> AgentId {
//         AgentId(Uuid::from(s))
//     }
// }

// impl From<&'static str> for AggregationId {
//     fn from(s: &'static str) -> AggregationId {
//         AggregationId(s)
//     }
// }

// impl<'a> From<&'a AgentId> for UserId {
//     fn from(id: &'a AgentId) -> UserId {
//         UserId(id.0.clone())
//     }
// }
