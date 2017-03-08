use super::*;

pub trait Store {

    fn put<T>(&self, id: &str, obj: &T) -> SdaClientStoreResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize;

    fn get<T>(&self, id: &str) -> SdaClientStoreResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize;

    fn put_aliased<T>(&self, alias: &str, obj: &T) -> SdaClientStoreResult<()>
        where T: ::serde::Serialize + ::serde::Deserialize + Identified
    {
        let id = obj.id().stringify();
        self.define_alias(alias, &id)?;
        self.put(&id, obj)
    }

    fn get_aliased<T>(&self, alias: &str) -> SdaClientStoreResult<Option<T>>
        where T: ::serde::Serialize + ::serde::Deserialize
    {
        match self.resolve_alias(alias)? {
            None => Ok(None),
            Some(id) => self.get(&id)
        }
    }

    fn define_alias(&self, alias: &str, id: &str) -> SdaClientStoreResult<()> {
        let alias_id = "alias_".to_string() + alias;
        match self.put(&alias_id, &id.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)?,
        }
    }
    
    fn resolve_alias(&self, alias: &str) -> SdaClientStoreResult<Option<String>> {
        let alias_id = "alias_".to_string() + alias;
        self.get(&alias_id)
    }

}