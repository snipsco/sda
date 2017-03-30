error_chain!{
    types {
        SdaServerError, SdaServerErrorKind, SdaServerResultExt, SdaServerResult;
    }
    foreign_links {
        Io(::std::io::Error);
        Sda(::sda_protocol::SdaError);
    }
}
