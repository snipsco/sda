error_chain!{
    types {
        SdaClientError, SdaClientErrorKind, SdaClientResultExt, SdaClientResult;
    }
    foreign_links {
        Protocol(::sda_protocol::SdaError);
        Io(::std::io::Error);
        SerdeJson(::serde_json::Error);
        NumParseInt(::std::num::ParseIntError);
        TimeSystemTime(::std::time::SystemTimeError);
    }
}