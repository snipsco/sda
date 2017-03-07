error_chain!{
    types {
        SdaHttpClientError, SdaHttpClientErrorKind, SdaHttpClientResultExt, SdaHttpClientResult;
    }
    links {
        Sda(::sda_protocol::SdaError, ::sda_protocol::SdaErrorKind);
    }
    foreign_links {
        Store(::sda_client_store::SdaClientStoreError);
        SerdeJson(::serde_json::Error);
        Http(::reqwest::Error);
        Url(::reqwest::UrlError);
    }
}
