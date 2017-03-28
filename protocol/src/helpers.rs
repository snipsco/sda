use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::*;

pub trait Identified {
    type I: Id;
    fn id(&self) -> &Self::I;
}

pub trait Id
    : Sized + ::std::str::FromStr<Err = String> + ToString + Serialize + Deserialize
    {
}

macro_rules! uuid_id {
    ( #[$doc:meta] $name:ident ) => {
        #[$doc]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn random() -> $name {
                $name(Uuid::new(::uuid::UuidVersion::Random).expect("No randomness source"))
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

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer
            {
                serializer.serialize_str(&*self.to_string())
            }
        }

        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                struct Visitor;
                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str("an hex hyphenated uuid")
                    }

                    fn visit_str<E>(self, v: &str) -> ::std::result::Result<Self::Value, E>
                        where E: ::serde::de::Error
                    {
                        use std::str::FromStr;
                        $name::from_str(v).map_err(|s| ::serde::de::Error::custom(s))
                    }

                }
                deserializer.deserialize_str(Visitor)
            }
        }
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
    pub body: M,
}

impl<M> std::ops::Deref for Signed<M>
    where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type Target = M;
    fn deref(&self) -> &M {
        &self.body
    }
}

impl<ID, M> Identified for Signed<M>
    where M: Identified<I = ID> + Clone + Debug + PartialEq + Serialize + Deserialize,
          ID: Id
{
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
pub struct Labelled<ID, M>
    where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
          ID: Id + Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    pub id: ID,
    pub body: M,
}

impl<ID, M> Identified for Labelled<ID, M>
    where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
          ID: Id + Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type I = ID;
    fn id(&self) -> &ID {
        &self.id
    }
}

pub fn label<ID, M>(id: &ID, body: &M) -> Labelled<ID, M>
    where M: Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
          ID: Id + Clone + Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    Labelled {
        id: id.clone(),
        body: body.clone(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binary(pub Vec<u8>);

impl Binary {
    fn to_base64(&self) -> String {
        ::data_encoding::base64::encode(&*self.0)
    }

    fn from_base64(s: &str) -> ::std::result::Result<Binary, String> {
        Ok(Binary(::data_encoding::base64::decode(s.as_bytes()).map_err(|e| format!("Base64 decoding error: {}", e))?))
    }
}

impl ::serde::Serialize for Binary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        serializer.serialize_str(&*self.to_base64())
    }
}

impl ::serde::Deserialize for Binary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer
    {
        struct Visitor;
        impl ::serde::de::Visitor for Visitor {
            type Value = Binary;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E>(self, v: &str) -> ::std::result::Result<Self::Value, E>
                where E: ::serde::de::Error
            {
                Binary::from_base64(v).map_err(|s| ::serde::de::Error::custom(s))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
