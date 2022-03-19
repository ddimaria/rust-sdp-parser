use crate::error::Result;
use crate::utils::{parse_number, parse_str};

/// SDP Origin
///
/// o=- 4611731400430051336 2 IN IP4 127.0.0.1
///
/// The first number is the session id, an unique identifier for the session.
/// The number in second position - 2 - is the session version: if a new
/// offer/answer negotiation is needed during this media session this number
/// will be increased by one. This will happen when any parameter need to be
/// changed in the media session such as on-hold, codec-change, add-remove
/// media track. The three following fields are the network type (Internet),
/// IP address type (version 4) and unicast address of the machine which
/// created the SDP. These three values are not relevant for the negotiation.
///
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Origin<'a> {
    pub username: &'a str,
    pub session_id: u64,
    pub session_version: u64,
    pub network_type: &'a str,
    pub ip_type: &'a str,
    pub ip_address: &'a str,
}

impl<'a> Origin<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let username = parse_str(split.next(), 1)?;
        let session_id = parse_number::<u64>(split.next(), 2)?;
        let session_version = parse_number::<u64>(split.next(), 3)?;
        let network_type = parse_str(split.next(), 4)?;
        let ip_type = parse_str(split.next(), 5)?;
        let ip_address = parse_str(split.next(), 6)?;

        Ok(Self {
            username,
            session_id,
            session_version,
            network_type,
            ip_type,
            ip_address,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_an_origin() {
        let origin = "- 4611731400430051336 2 IN IP4 127.0.0.1";
        let parsed = Origin::new(origin).unwrap();
        let expected = Origin {
            username: "-",
            session_id: 4611731400430051336,
            session_version: 2,
            network_type: "IN",
            ip_type: "IP4",
            ip_address: "127.0.0.1",
        };

        assert_eq!(parsed, expected);
    }
}
