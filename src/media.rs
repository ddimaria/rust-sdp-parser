use crate::error::{Error, Result};
use crate::utils::{parse_number, parse_str};
use crate::{push_value, set_value};

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
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Media<'a> {
    pub r#type: &'a str,
    pub port: u64,
    pub protocol: &'a str,
    pub payloads: &'a str,
    pub candidates: Vec<Candidate<'a>>,
    pub direction: &'a str,
    pub fmtp: Vec<Fmtp<'a>>,
    pub ptime: u64,
    pub rtpmap: Vec<Rtpmap<'a>>,
    pub rtc_fb: Vec<RtcpFb<'a>>,
    pub ssrc: Vec<Ssrc<'a>>,
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

    pub(crate) fn parse_attribute(&mut self, attribute: &'a str, value: &'a str) -> Result<()> {
        match attribute {
            "ptime" => set_value!(self.ptime, parse_number::<u64>(Some(value), 1)),
            "rtpmap" => push_value!(self.rtpmap, Rtpmap::new(value)),
            "candidate" => push_value!(self.candidates, Candidate::new(value)),
            "fmtp" => push_value!(self.fmtp, Fmtp::new(value)),
            "rtcp-fb" => push_value!(self.rtc_fb, RtcpFb::new(value)),
            "ssrc" => push_value!(self.ssrc, Ssrc::new(value)),
            "direction" => set_value!(self.direction, Result::Ok(value)),
            _ => Err(Error::Parse(format!(
                "Unsupported media attribute: {}",
                attribute
            ))),
        }
    }
}

/// Candidate
///
/// a=candidate:1467250027 1 udp 2122260223 192.168.0.196 46243 typ host generation 0
///
/// ICE is the protocol chosen for NAT traversal in WebRTC. You can find a very
/// didactic and comprehensive explanation of ICE here. ICE is complex enough
/// to deserve its own post, but I will try to explain its SDP lines in an
/// understandable way.
///
/// Host candidate for RTP on UDP - in this ICE line our browser is giving its
/// host candidates- the IP of the interface or interfaces the browser is listening
/// on the computer. The browser can receive/send SRTP and SRTCP on that IP in case
/// there is IP visibility with some candidate of the remote peer. For example, if
/// the other computer is on the same LAN, hosts candidates will be used. The number
/// after the protocol (udp) – 2122260223 - is the priority of the candidate. Notice
/// that priority of host candidates is the higher than other candidates as using host
/// candidates are more efficient in terms of use of resources. The first lines
/// (component= 1) is for RTP and second line (component = 2) is for RTCP.
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Candidate<'a> {
    pub component: u64,
    pub foundation: &'a str,
    pub transport: &'a str,
    pub priority: u64,
    pub ip: &'a str,
    pub port: u64,
    pub r#type: &'a str,
}

impl<'a> Candidate<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let component = parse_number::<u64>(split.next(), 1)?;
        let foundation = parse_str(split.next(), 2)?;
        let transport = parse_str(split.next(), 3)?;
        let priority = parse_number::<u64>(split.next(), 4)?;
        let ip = parse_str(split.next(), 5)?;
        let port = parse_number::<u64>(split.next(), 6)?;

        // skip typ
        split.next();

        let r#type = parse_str(split.next(), 7)?;

        Ok(Self {
            component,
            foundation,
            transport,
            priority,
            ip,
            port,
            r#type,
        })
    }
}

/// FMTP
///
/// a=fmtp:111 minptime=10; useinbandfec=1
///
/// This line includes optional payload-format-specific parameters supported by Chrome
/// for audio Opus codec. minipitime=10 specifies the lowest value of the packetization
/// time (ptime: the number of miliseconds of audio transported by a single packet).
/// useinbandfec=1 specifies that the decoder has the capability to take advantage of
/// the Opus in-band FEC (Forward Error Correction). For more info check RFC7587.
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Fmtp<'a> {
    pub config: &'a str,
    pub payload: u64,
}

impl<'a> Fmtp<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.splitn(2, ' ');
        let payload = parse_number::<u64>(split.next(), 1)?;
        let config = parse_str(split.next(), 2)?;

        Ok(Self { payload, config })
    }
}

/// RTP Map
///
/// a=rtpmap:111 opus/48000/2
///
/// Opus is one of the MTI audio codecs for WebRTC. It features a variable
/// bit rate (6kbps-510kbps) and is not under any royalty so it can be freely
/// implemented in any browser (unlike other codecs like as G.729). Opus
/// support is starting to become common and it has become critical for most
/// WebRTC applications.
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Rtpmap<'a> {
    pub codec: &'a str,
    pub payload: &'a str,
    pub rate: u64,
}

impl<'a> Rtpmap<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let payload = parse_str(split.next(), 1)?;

        let mut split = parse_str(split.next(), 2)?.split('/');
        let codec = parse_str(split.next(), 2)?;
        let rate = parse_number::<u64>(split.next(), 3)?;

        Ok(Self {
            codec,
            payload,
            rate,
        })
    }
}

/// RTCP FB
///
/// a=rtcp-fb:100 nack
///
/// This line requests the use of Negative ACKs (nack) as indicated in RFC 4585.
/// This allows to make the other end aware about packet losses.
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct RtcpFb<'a> {
    pub payload: &'a str,
    pub r#type: &'a str,
}

impl<'a> RtcpFb<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let payload = parse_str(split.next(), 1)?;
        let r#type = parse_str(split.next(), 2)?;

        Ok(Self { payload, r#type })
    }
}

/// SSSRC
///
/// a=ssrc:3570614608 cname:4TOk42mSjXCkVIa6
///
/// The cname source attribute associates a media source with its Canonical End-Point
/// Identifier which will remain constant for the RTP media stream even when the ssrc
/// identifier changes if a conflict is found. This is the value that the media sender
/// will place in its RTCP SDES packets.
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Ssrc<'a> {
    pub id: u64,
    pub attribute: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<&'a str>,
}

impl<'a> Ssrc<'a> {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let id = parse_number::<u64>(split.next(), 1)?;

        let mut split = parse_str(split.next(), 2)?.split(':');
        let attribute = parse_str(split.next(), 2)?;
        let mut value = None;

        if let Some(split) = split.next() {
            value = Some(parse_str(Some(split), 3)?);
        }

        Ok(Self {
            id,
            attribute,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_media() {
        let media = "audio 58779 UDP/TLS/RTP/SAVPF 111 103 104 9 0 8 106 105 13 126";
        let parsed = Media::new(media).unwrap();
        let expected = Media {
            r#type: "audio",
            port: 58779,
            protocol: "UDP/TLS/RTP/SAVPF",
            payloads: "111",
            candidates: vec![],
            direction: "",
            fmtp: vec![],
            ptime: 0,
            rtpmap: vec![],
            rtc_fb: vec![],
            ssrc: vec![],
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_parses_a_candidate() {
        let candidate = "1467250027 1 udp 2122260223 192.168.0.196 46243 typ host generation 0";
        let parsed = Candidate::new(candidate).unwrap();
        let expected = Candidate {
            component: 1467250027,
            foundation: "1",
            transport: "udp",
            priority: 2122260223,
            ip: "192.168.0.196",
            port: 46243,
            r#type: "host",
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_parses_a_fmtp() {
        let fmtp = "111 minptime=10; useinbandfec=1";
        let parsed = Fmtp::new(fmtp).unwrap();
        let expected = Fmtp {
            config: "minptime=10; useinbandfec=1",
            payload: 111,
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_parses_a_rtpmap() {
        let rtpmap = "111 opus/48000/2";
        let parsed = Rtpmap::new(rtpmap).unwrap();
        let expected = Rtpmap {
            codec: "opus",
            payload: "111",
            rate: 48000,
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_parses_a_rtcpfb() {
        let rtcpfb = "100 nack";
        let parsed = RtcpFb::new(rtcpfb).unwrap();
        let expected = RtcpFb {
            payload: "100",
            r#type: "nack",
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_parses_a_ssrc() {
        let ssrc = "3570614608 cname:4TOk42mSjXCkVIa6";
        let parsed = Ssrc::new(ssrc).unwrap();
        let expected = Ssrc {
            id: 3570614608,
            attribute: "cname",
            value: Some("4TOk42mSjXCkVIa6"),
        };

        assert_eq!(parsed, expected);
    }
}
