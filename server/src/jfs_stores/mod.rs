use jfs;

use sda_protocol::{Id, Identified};

use errors::*;


mod agents;
mod aggregations;
mod auth_tokens;
mod clerking_jobs;

pub use self::agents::JfsAgentsStore;
pub use self::auth_tokens::JfsAuthTokensStore;
pub use self::aggregations::JfsAggregationsStore;
pub use self::clerking_jobs::JfsClerkingJobsStore;

trait JfsStoreExt {
    fn get_option_for_str<T, S>(&self, id: S) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              S: AsRef<str>;

    fn get_option<T, I>(&self, id: &I) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              I: Id;

    fn create_with_id<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + PartialEq,
              I: Id;

    fn create<T>(&self, it: &T) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + Identified + PartialEq {
        self.create_with_id(it, it.id())
    }

    fn update_with_id<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + PartialEq,
              I: Id;

    fn update<T>(&self, it: &T) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + Identified + PartialEq {
        self.update_with_id(it, it.id())
    }

    fn upsert_with_id<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + PartialEq,
              I: Id;

    fn upsert<T>(&self, it: &T) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + Identified + PartialEq {
        self.upsert_with_id(it, it.id())
    }
}

impl JfsStoreExt for jfs::Store {
    fn get_option_for_str<T, S>(&self, id: S) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              S: AsRef<str>
    {
        match self.get(id.as_ref()) {
            Ok(it) => Ok(Some(it)),
            Err(io) => {
                if io.kind() == ::std::io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(io)?
                }
            }
        }
    }

    fn get_option<T, I>(&self, id: &I) -> SdaServerResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize,
              I: Id
    {
        self.get_option_for_str(id.to_string())
    }

    fn create_with_id<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + PartialEq,
              I: Id {
        if let Some(prev) = self.get_option_for_str::<T, _>(&*id.to_string())? {
            if prev != *it {
                Err("File already exists")?
            }
        }
        self.save_with_id(it, &*id.to_string())?;
        Ok(())
    }

    fn update_with_id<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + PartialEq,
              I: Id {
        if self.get_option_for_str::<T, _>(&*id.to_string())?.is_none() {
            Err("File not present")?
        }
        self.save_with_id(it, &*id.to_string())?;
        Ok(())
    }

    fn upsert_with_id<T, I>(&self, it: &T, id: &I) -> SdaServerResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + PartialEq,
              I: Id {
        self.save_with_id(it, &*id.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;
    use jfs_stores::JfsStoreExt;

    #[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
    struct I(String);
    #[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
    struct A { id: I, int: usize}

    impl ::sda_protocol::Identified for A {
        type I=I;
        fn id(&self) -> &I {
            &self.id
        }
    }

    impl ::sda_protocol::Id for I { }

    impl ::std::str::FromStr for I {
        type Err = String;
        fn from_str(s:&str) -> ::std::result::Result<I,String> {
            Ok(I(s.to_string()))
        }
    }

    impl ::std::fmt::Display for I {
        fn fmt(&self, f:&mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
            self.0.fmt(f)
        }
    }

    #[test]
    fn create() {
        let tmpdir = tempdir::TempDir::new("sda").unwrap();
        let store = ::jfs::Store::new(&tmpdir.path().to_str().unwrap()).unwrap();
        let a = A { id: I("foo".to_string()), int: 12};
        store.create(&a).unwrap();
        store.create(&a).unwrap();
        let b = A { id: I("foo".to_string()), int: 42};
        assert!(store.create(&b).is_err());
    }

    #[test]
    fn update() {
        let tmpdir = tempdir::TempDir::new("sda").unwrap();
        let store = ::jfs::Store::new(&tmpdir.path().to_str().unwrap()).unwrap();
        let a = A { id: I("foo".to_string()), int: 12};
        store.create(&a).unwrap();
        let a = A { id: I("foo".to_string()), int: 42};
        store.update(&a).unwrap();
        let a_again = store.get_option_for_str("foo").unwrap().unwrap();
        assert_eq!(a, a_again);

        let b = A { id: I("bar".to_string()), int: 42};
        assert!(store.update(&b).is_err());
    }


    #[test]
    fn upsert() {
        let tmpdir = tempdir::TempDir::new("sda").unwrap();
        let store = ::jfs::Store::new(&tmpdir.path().to_str().unwrap()).unwrap();
        let a = A { id: I("foo".to_string()), int: 12};
        store.create(&a).unwrap();
        let a = A { id: I("foo".to_string()), int: 42};
        store.upsert(&a).unwrap();
        let a_again = store.get_option_for_str("foo").unwrap().unwrap();
        assert_eq!(a, a_again);

        let b = A { id: I("bar".to_string()), int: 42};
        store.upsert(&b).is_err();
        let b_again = store.get_option_for_str("bar").unwrap().unwrap();
        assert_eq!(b, b_again);
    }
}
