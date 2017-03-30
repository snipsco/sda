//! Typed binary arrays of specific sizes.

macro_rules! B {
    ($name:ident, $size:expr) => {
        #[derive(Copy)]
        pub struct $name(pub [u8;$size]);

        impl $name {

            fn to_base64(&self) -> String {
                ::data_encoding::base64::encode(&self.0)
            }

            fn from_base64(s:&str) -> ::std::result::Result<$name, String> {
                let mut data = [0; $size];
                let decode = &s.as_bytes()[0..::data_encoding::base64::encode_nopad_len($size)];
                match ::data_encoding::base64::decode_nopad_mut(decode, &mut data) {
                    Ok(_) => Ok($name(data)),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }

        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer
            {
                serializer.serialize_str(&*self.to_base64())
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
                        formatter.write_str("a base64 string")
                    }

                    fn visit_str<E>(self, v: &str) -> ::std::result::Result<Self::Value, E>
                        where E: ::serde::de::Error
                    {
                        $name::from_base64(v).map_err(|s| ::serde::de::Error::custom(s))
                    }

                }
                deserializer.deserialize_str(Visitor)
            }
        }

        impl Clone for $name {
            fn clone(&self) -> $name {
                *self
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                write!(fmt, "{:?}", &self.0 as &[u8])
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> $name {
                $name([0; $size])
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                (0..$size).all(|i| self.0[i] == other.0[i])
            }
        }

        impl ::std::cmp::Eq for $name { }

        impl ::std::ops::Deref for $name {
            type Target = [u8; $size];
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::From<[u8; $size]> for $name {
            fn from(data: [u8; $size]) -> $name {
                $name(data)
            }
        }
    }
}

B!(B8, 8);
B!(B32, 32);
B!(B64, 64);

#[cfg(test)]
mod tests {
    use serde_test::{Token, assert_tokens};

    #[test]
    fn test_b64_raw() {
        let a = [0u8; 8];
        let a64 = ::data_encoding::base64::encode(&a);
        assert_eq!("AAAAAAAAAAA=", a64);

        let mut b = [0u8; 8];
        let decode = &a64.as_bytes()[0..::data_encoding::base64::encode_nopad_len(8)];
        ::data_encoding::base64::decode_nopad_mut(decode, &mut b).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_b64() {
        let a = super::B8::default();
        assert_eq!("AAAAAAAAAAA=", a.to_base64());
        let clone = super::B8::from_base64(&*a.to_base64()).unwrap();
        assert_eq!(a, clone);
    }

    #[test]
    fn test_serde() {
        #[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        struct T {
            a: super::B8,
            b: super::B32,
            c: super::B64,
        };
        let mut t = T::default();
        assert_tokens(&t,
                      &[Token::StructStart("T", 3),

                        Token::StructSep,
                        Token::Str("a"),
                        Token::Str("AAAAAAAAAAA="),

                        Token::StructSep,
                        Token::Str("b"),
                        Token::Str("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),

                        Token::StructSep,
                        Token::Str("c"),
                        Token::Str("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=="),

                        Token::StructEnd]);
    }
}
