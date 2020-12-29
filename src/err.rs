//! error definitions
use thiserror::Error;
/// crate error enum
#[derive(Error, Debug)]
pub enum OcspError {
    /// cannot convert raw data into asn1_der::typed::*;
    #[error(transparent)]
    Asn1DecodingError(#[from] asn1_der::Asn1DerError),

    /// extractor cannot find matching sequence  
    /// eg. OID sequence is not 0x06, 0x05
    #[error("Unable to extract desired sequence of {0} at {1}")]
    Asn1MismatchError(&'static str, &'static str),

    /// unable to parse vec\<u8\> to &str   
    /// eg. requestorName
    #[error("Unable to deserialize string from ocsp req/resp")]
    Asn1Utf8Error(#[from] std::str::Utf8Error),

    /// OID length is not 2, 0x06, 0x05
    #[error("Unable to deserialize {0} due to incorrect sequence length at {1}")]
    Asn1LengthError(&'static str, &'static str),

    /// Cannot find OID in predefined list
    #[error("Unable to locate OID info {0}")]
    Asn1OidUnknown(&'static str),
}

/// display error location
#[macro_export]
macro_rules! err_at{
    () => {
        concat!("at ", file!(), " line ", line!(), " column ", column!())
    };
}
