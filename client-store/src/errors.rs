error_chain!{
    types {
        SdaClientStoreError, SdaClientStoreErrorKind, SdaClientStoreResultExt, SdaClientStoreResult;
    }
    foreign_links {
        Io(::std::io::Error);
        SerdeJson(::serde_json::Error);
    }
}