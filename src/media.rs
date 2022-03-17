use crate::error::Result;
use crate::utils::{parse_number, parse_str};

/// SDP Media
///
/// m=audio 58779 UDP/TLS/RTP/SAVPF 111 103 104 9 0 8 106 105 13 126
/// m=video 60372 UDP/TLS/RTP/SAVPF 100 101 116 117 96
///
/// m means it is a media line – it condenses a lot of information on the media
/// attributes of the stream. In this order, it tells us:
///
/// * video- the media type is going to be used for the session (media types are registered at the IANA),
/// * 60372
/// * UDP/TLS/RTP/SAVPF- the transport protocol is going to be used for the session, and last but not least
/// * 100 101 116 117 96 - the media format descriptions are supported by the browser to send and receive media.
/// * UDP/TLS/RTP/SAVPF is defined in RFC5764. In short it requires the use of SRTP and SRTCP and RTCP Feedback packets.
///
/// The media format descriptions, with protocol UDP/TLS/RTP/SAVPF, gives the
/// RTP payload numbers which are going to be used for the different formats.  
/// Payload numbers lower than 96 are mapped to encodingformats by the IANA.  
/// In our SDP 100 maps to VP8 and 101 to VP9. Format numbers larger than 95
/// are dynamic and there are a=rtpmap: attribute to map from the RTP payload
/// type numbers to media encoding names.  There are also a=fmtp: attributes
#[derive(Debug, Default, Serialize)]
pub struct Media<'a> {
    r#type: &'a str,
    port: u64,
    protocol: &'a str,
    payloads: &'a str,
    candidates: Vec<Candidate<'a>>,
    direction: &'a str,
    fmtp: Vec<Fmtp<'a>>,
    ptime: u64,
    rtp: Vec<Rtp<'a>>,
}

#[derive(Debug, Default, Serialize)]
pub struct Candidate<'a> {
    component: u64,
    foundation: &'a str,
    ip: &'a str,
    port: u64,
    priority: u64,
    transport: &'a str,
    r#type: &'a str,
}

#[derive(Debug, Default, Serialize)]
pub struct Fmtp<'a> {
    config: &'a str,
    payload: u64,
}

#[derive(Debug, Default, Serialize)]
pub struct Rtp<'a> {
    codec: &'a str,
    payload: u64,
    rate: u64,
}

impl<'a> Media<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let r#type = parse_str(split.next(), 1)?;
        let port = parse_number::<u64>(split.next(), 2)?;
        let protocol = parse_str(split.next(), 3)?;
        let payloads = parse_str(split.next(), 4)?;

        Ok(Self {
            r#type,
            port,
            protocol,
            payloads,
            ..Default::default()
        })
    }
}
