
use super::*;

impl Agent {
    pub fn new() -> Agent {
        Agent {
            id: AgentId::new(),
            auth_token: None,
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
        VerificationKey::Sodium { 
            key: vec![0; 32]
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
