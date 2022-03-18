use crate::error::Result;
use crate::utils::parse_str;

/// SDP Fingerprint
///
/// a=fingerprint:sha-256 49:66:12:17:0D:1C:91:AE:57:4C:C6:36:DD:D5:97:D2:7D:62:C9:9A:7F:B9:A3:F4:70:03:E7:43:91:73:23:5E
///
/// This fingerprint is the result of a hash function (using sha-256 in this case)
/// of the certificates used in the DTLS-SRTP negotiation. This line creates a
/// binding between the signaling (which is supposed to be trusted) and the
/// certificates used in DTLS, if the fingerprint doesn’t match, then the session
/// should be rejected.
///
#[derive(Debug, Default, Serialize)]
pub struct Fingerprint<'a> {
    r#type: &'a str,
    hash: &'a str,
}

impl<'a> Fingerprint<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let r#type = parse_str(split.next(), 1)?;
        let hash = parse_str(split.next(), 2)?;

        Ok(Self { r#type, hash })
    }
}
