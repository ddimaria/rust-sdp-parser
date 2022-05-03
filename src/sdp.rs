use crate::connection::Connection;
use crate::error::{Error, Result};
use crate::fingerprint::Fingerprint;
use crate::media::Media;
use crate::origin::Origin;
use crate::set_value;
use crate::time::Time;
use crate::utils::{parse_number, parse_str};

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Sdp<'a> {
    version: u32,
    session_name: &'a str,
    ice_ufrag: &'a str,
    ice_pwd: &'a str,
    fingerprint: Fingerprint<'a>,
    origin: Origin<'a>,
    time: Time,
    connection: Connection<'a>,
    media: Vec<Media<'a>>,

    #[serde(skip)]
    current_media: Option<usize>,
}

impl<'a> Sdp<'a> {
    // parse each line of the SDP
    pub fn parse(sdp_message: &'a str) -> Result<Self> {
        let mut sdp = Sdp::default();
        let lines = sdp_message.lines();

        for line in lines {
            sdp.parse_line(line)?;
        }

        Ok(sdp)
    }

    // parse an individual SDP line
    // return errors for invalid entries
    fn parse_line(&mut self, line: &'a str) -> Result<()> {
        let split = line.splitn(2, '=').collect::<Vec<&str>>();
        let (key, value) = (split[0], split[1].trim());

        match key {
            "v" => set_value!(self.version, parse_number::<u32>(Some(value), 1)),
            "o" => set_value!(self.origin, Origin::new(value)),
            "s" => set_value!(self.session_name, parse_str(Some(value), 1)),
            "t" => set_value!(self.time, Time::new(value)),
            "c" => set_value!(self.connection, Connection::new(value)),
            "a" => self.parse_attribute(value),
            "m" => self.parse_media(value),
            _ => Err(Error::Parse(format!("Unsupported attribute: {}", key))),
        }
    }

    // media parsing is slightly more complex
    // maintain state as subsequent lines relate to the current_media
    fn parse_media(&mut self, value: &'a str) -> Result<()> {
        let count = self.current_media.unwrap_or(0);
        self.current_media = Some(count + 1);
        self.media.push(Media::new(value)?);

        Ok(())
    }

    fn parse_media_attribute(&mut self, attribute: &'a str, value: &'a str) -> Result<()> {
        let count = self.current_media.unwrap_or(0);
        let media = self.media.get_mut(count - 1).ok_or_else(|| {
            Error::Parse("Cannot parse a media attribute before a 'm' line".into())
        })?;

        media.parse_attribute(attribute, value)
    }

    fn parse_attribute(&mut self, value: &'a str) -> Result<()> {
        let split = value.splitn(2, ':').collect::<Vec<&str>>();

        if split.len() == 1 {
            self.parse_media_attribute("direction", split[0])?
        } else {
            match split[0] {
                "ice-ufrag" => self.ice_ufrag = split[1],
                "ice-pwd" => self.ice_pwd = split[1],
                "fingerprint" => self.fingerprint = Fingerprint::new(split[1])?,
                _ => self.parse_media_attribute(split[0], split[1])?,
            }
        }

        Ok(())
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(|e| Error::ConvertToJson(e.to_string()))
    }
}

#[macro_export]
macro_rules! set_value {
    ($attribute:expr, $value:expr) => {{
        $attribute = $value?;
        Ok(())
    }};
}

#[macro_export]
macro_rules! push_value {
    ($attribute:expr, $value:expr) => {{
        $attribute.push($value?);
        Ok(())
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::{Candidate, Fmtp, Media, RtcpFb, Rtpmap, Ssrc};

    const SDP: &'static str = "v=0
o=- 20518 0 IN IP4 203.0.113.1
s=
t=0 0
c=IN IP4 203.0.113.1
a=ice-ufrag:F7gI
a=ice-pwd:x9cml/YzichV2+XlhiMu8g
a=fingerprint:sha-1 42:89:c5:c6:55:9d:6e:c8:e8:83:55:2a:39:f9:b6:eb:e9:a3:a9:e7
m=audio 54400 RTP/SAVPF 0 96
a=rtpmap:0 PCMU/8000
a=rtpmap:96 opus/48000
a=ptime:20
a=sendrecv
a=candidate:0 1 UDP 2113667327 203.0.113.1 54400 typ host
a=candidate:1 2 UDP 2113667326 203.0.113.1 54401 typ host
m=video 55400 RTP/SAVPF 97 98
a=rtcp-fb:* nack
a=rtpmap:97 H264/90000
a=fmtp:97 profile-level-id=4d0028;packetization-mode=1
a=rtcp-fb:97 trr-int 100
a=rtcp-fb:97 nack rpsi
a=rtpmap:98 VP8/90000
a=rtcp-fb:98 trr-int 100
a=rtcp-fb:98 nack rpsi
a=sendrecv
a=candidate:0 1 UDP 2113667327 203.0.113.1 55400 typ host
a=candidate:1 2 UDP 2113667326 203.0.113.1 55401 typ host
a=ssrc:1399694169 foo:bar
a=ssrc:1399694169 baz";

    #[test]
    fn it_parses_a_sdp_message() {
        let parsed = Sdp::parse(SDP).unwrap();
        let expected = Sdp {
            version: 0,
            session_name: "",
            ice_ufrag: "F7gI",
            ice_pwd: "x9cml/YzichV2+XlhiMu8g",
            fingerprint: Fingerprint {
                r#type: "sha-1",
                hash: "42:89:c5:c6:55:9d:6e:c8:e8:83:55:2a:39:f9:b6:eb:e9:a3:a9:e7",
            },
            origin: Origin {
                username: "-",
                session_id: 20518,
                session_version: 0,
                network_type: "IN",
                ip_type: "IP4",
                ip_address: "203.0.113.1",
            },
            time: Time {
                start_time: 0,
                stop_time: 0,
                bounded: false,
            },
            connection: Connection {
                network_type: "IN",
                ip_type: "IP4",
                ip_address: "203.0.113.1",
            },
            media: vec![
                Media {
                    r#type: "audio",
                    port: 54400,
                    protocol: "RTP/SAVPF",
                    payloads: "0",
                    candidates: vec![
                        Candidate {
                            component: 0,
                            foundation: "1",
                            transport: "UDP",
                            priority: 2113667327,
                            ip: "203.0.113.1",
                            port: 54400,
                            r#type: "host",
                        },
                        Candidate {
                            component: 1,
                            foundation: "2",
                            transport: "UDP",
                            priority: 2113667326,
                            ip: "203.0.113.1",
                            port: 54401,
                            r#type: "host",
                        },
                    ],
                    direction: "sendrecv",
                    fmtp: vec![],
                    ptime: 20,
                    rtpmap: vec![
                        Rtpmap {
                            codec: "PCMU",
                            payload: "0",
                            rate: 8000,
                        },
                        Rtpmap {
                            codec: "opus",
                            payload: "96",
                            rate: 48000,
                        },
                    ],
                    rtc_fb: vec![],
                    ssrc: vec![],
                },
                Media {
                    r#type: "video",
                    port: 55400,
                    protocol: "RTP/SAVPF",
                    payloads: "97",
                    candidates: vec![
                        Candidate {
                            component: 0,
                            foundation: "1",
                            transport: "UDP",
                            priority: 2113667327,
                            ip: "203.0.113.1",
                            port: 55400,
                            r#type: "host",
                        },
                        Candidate {
                            component: 1,
                            foundation: "2",
                            transport: "UDP",
                            priority: 2113667326,
                            ip: "203.0.113.1",
                            port: 55401,
                            r#type: "host",
                        },
                    ],
                    direction: "sendrecv",
                    fmtp: vec![Fmtp {
                        config: "profile-level-id=4d0028;packetization-mode=1",
                        payload: 97,
                    }],
                    ptime: 0,
                    rtpmap: vec![
                        Rtpmap {
                            codec: "H264",
                            payload: "97",
                            rate: 90000,
                        },
                        Rtpmap {
                            codec: "VP8",
                            payload: "98",
                            rate: 90000,
                        },
                    ],
                    rtc_fb: vec![
                        RtcpFb {
                            payload: "*",
                            r#type: "nack",
                        },
                        RtcpFb {
                            payload: "97",
                            r#type: "trr-int",
                        },
                        RtcpFb {
                            payload: "97",
                            r#type: "nack",
                        },
                        RtcpFb {
                            payload: "98",
                            r#type: "trr-int",
                        },
                        RtcpFb {
                            payload: "98",
                            r#type: "nack",
                        },
                    ],
                    ssrc: vec![
                        Ssrc {
                            id: 1399694169,
                            attribute: "foo",
                            value: Some("bar"),
                        },
                        Ssrc {
                            id: 1399694169,
                            attribute: "baz",
                            value: None,
                        },
                    ],
                },
            ],
            current_media: Some(2),
        };

        assert_eq!(parsed, expected);
    }
}
