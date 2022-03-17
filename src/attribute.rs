use crate::error::Result;
use crate::utils::{parse_number, parse_str};

/// SDP Attribute
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
#[derive(Debug, Default, Serialize)]
pub struct Attribute<'a> {
    name: &'a str,
}

impl<'a> Attribute<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let split = value.splitn(2, ':').collect::<Vec<&str>>();

        if split.len() > 1 {
            let (key, value) = (split[0], split[1].trim());
            // println!("{:?} {:?}", key, value);
        } else {
            // println!("{:?}", split);
        }

        // let mut split = value.split(':');

        // while let Some(value) = split.next() {
        //     println!("{:?}", value);
        // }

        Ok(Self { name: "David" })
    }
}
