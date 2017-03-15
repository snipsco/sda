use serde::{ Deserialize, Serialize};
use std::fmt::Debug;

use super::*;

pub trait Identified {
    type I : Id;
    fn id(&self) -> &Self::I;
}

pub trait Id: Sized + ::std::str::FromStr<Err=String> + ToString {}

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

        impl ::std::str::FromStr for $name {
            type Err=String;
            fn from_str(s: &str) -> std::result::Result<$name, String> {
                let uuid = Uuid::parse_str(s).map_err(|_| format!("unparseable uuid {}", s))?;
                Ok($name(uuid))
            }
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                self.0.hyphenated().to_string()
            }
        }

        impl Id for $name {}
    }
}

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signed<M>
where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    pub signature: Signature,
    pub signer: AgentId,
    pub body: M
}

impl<M> std::ops::Deref for Signed<M>
where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type Target = M;
    fn deref(&self) -> &M {
        &self.body
    }
}

impl<ID,M> Identified for Signed<M>
where M:Identified<I=ID>+Clone+Debug+PartialEq+Serialize+Deserialize, ID:Id {
    type I = ID;
    fn id(&self) -> &ID {
        use std::ops::Deref;
        self.deref().id()
    }
}

pub trait Sign {
    fn canonical(&self) -> SdaResult<Vec<u8>>;
}

impl<T: ::serde::Serialize> Sign for T {
    fn canonical(&self) -> SdaResult<Vec<u8>> {
        Ok(::serde_json::to_vec(self)?)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Labelled<ID,M>
where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
      ID: Id + Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
{
    pub id: ID,
    pub body: M
}

impl<ID,M> Identified for Labelled<ID,M>
where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
      ID: Id + Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type I = ID;
    fn id(&self) -> &ID {
        &self.id
    }
}

