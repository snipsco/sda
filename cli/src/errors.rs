error_chain!{
    types {
        SdaCliError, SdaCliErrorKind, SdaCliResultExt, SdaCliResult;
    }
    foreign_links {
        Protocol(::sda_protocol::SdaError);
        Client(::sda_client::SdaClientError);
        Http(::sda_client_http::SdaHttpClientError);
        Store(::sda_client_store::SdaClientStoreError);
    }
}
