error_chain!{
    types {
        SdaHttpClientError, SdaHttpClientErrorKind, SdaHttpClientResultExt, SdaHttpClientResult;
    }
    foreign_links {
        Protocol(::sda_protocol::SdaError);
        Store(::sda_client_store::SdaClientStoreError);
        SerdeJson(::serde_json::Error);
        Http(::reqwest::Error);
        Url(::reqwest::UrlError);
    }
}