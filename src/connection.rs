use crate::error::Result;
use crate::utils::parse_str;

/// SDP Connection
///
/// c=IN IP4 217.130.243.155
///
/// c is a connection line. This line gives the IP from where you expect to
/// send and receive the real time traffic. As ICE is mandatory in WebRTC the
/// IP in the c-line is not going to be used.
///
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Connection<'a> {
    pub network_type: &'a str,
    pub ip_type: &'a str,
    pub ip_address: &'a str,
}

impl<'a> Connection<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let network_type = parse_str(split.next(), 1)?;
        let ip_type = parse_str(split.next(), 2)?;
        let ip_address = parse_str(split.next(), 3)?;

        Ok(Self {
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
    fn it_parses_a_connection() {
        let connection = "IN IP4 203.0.113.1";
        let parsed = Connection::new(connection).unwrap();
        let expected = Connection {
            network_type: "IN",
            ip_type: "IP4",
            ip_address: "203.0.113.1",
        };

        assert_eq!(parsed, expected);
    }
}
