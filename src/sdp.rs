use crate::attribute::Attribute;
use crate::connection::Connection;
use crate::error::Result;
use crate::fingerprint::Fingerprint;
use crate::media::Media;
use crate::origin::Origin;
use crate::time::Time;
use crate::utils::{parse_number, parse_str};

#[derive(Debug, Default, Serialize)]
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
    current_media: Option<u32>,
}

impl<'a> Sdp<'a> {
    pub fn parse(sdp_message: &'static str) -> Result<Self> {
        let mut sdp = Sdp::default();
        let lines = sdp_message.lines();

        for line in lines {
            sdp.parse_line(&line)?;
        }

        Ok(sdp)
    }

    fn parse_line(&mut self, line: &'static str) -> Result<()> {
        let split = line.splitn(2, '=').collect::<Vec<&str>>();
        let (key, value) = (split[0], split[1].trim());

        match key {
            // globals
            "v" => self.parse_version(&value)?,
            "o" => self.parse_origin(&value)?,
            "s" => self.parse_session_name(&value)?,
            "t" => self.parse_time(&value)?,
            "c" => self.parse_connection(&value)?,
            "a" => self.parse_attribute(&value)?,
            "m" => self.parse_media(&value)?,
            _ => {}
        };

        Ok(())
    }

    fn parse_version(&mut self, value: &str) -> Result<()> {
        self.version = parse_number::<u32>(Some(&value), 1)?;
        Ok(())
    }

    fn parse_origin(&mut self, value: &'static str) -> Result<()> {
        self.origin = Origin::new(&value)?;
        Ok(())
    }

    fn parse_session_name(&mut self, value: &'static str) -> Result<()> {
        self.session_name = parse_str(Some(&value), 1)?;
        Ok(())
    }

    fn parse_time(&mut self, value: &'static str) -> Result<()> {
        self.time = Time::new(&value)?;
        Ok(())
    }

    fn parse_connection(&mut self, value: &'static str) -> Result<()> {
        self.connection = Connection::new(&value)?;
        Ok(())
    }

    fn parse_fingerprint(&mut self, value: &'static str) -> Result<()> {
        self.fingerprint = Fingerprint::new(&value)?;
        Ok(())
    }

    fn parse_media(&mut self, value: &'static str) -> Result<()> {
        let count = self.current_media.unwrap_or(0);
        self.current_media = Some(count + 1);

        println!("MEDIA: {}", value);
        self.media.push(Media::new(&value)?);
        Ok(())
    }

    fn parse_attribute(&mut self, value: &'static str) -> Result<()> {
        let split = value.splitn(2, ':').collect::<Vec<&str>>();

        match split[0] {
            "ice-ufrag" => self.ice_ufrag = split[1],
            "ice-pwd" => self.ice_pwd = split[1],
            "fingerprint" => self.parse_fingerprint(split[1])?,
            _ => {}
        }

        // "rtpmap" "0 PCMU/8000"
        // "rtpmap" "96 opus/48000"
        // "ptime" "20"
        // ["sendrecv"]
        // "candidate" "0 1 UDP 2113667327 203.0.113.1 54400 typ host"
        // "candidate" "1 2 UDP 2113667326 203.0.113.1 54401 typ host"
        // "rtcp-fb" "* nack"
        // "rtpmap" "97 H264/90000"
        // "fmtp" "97 profile-level-id=4d0028;packetization-mode=1"
        // "rtcp-fb" "97 trr-int 100"
        // "rtcp-fb" "97 nack rpsi"
        // "rtpmap" "98 VP8/90000"
        // "rtcp-fb" "98 trr-int 100"
        // "rtcp-fb" "98 nack rpsi"

        Attribute::new(&value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let json = serde_json::to_string_pretty(&parsed).unwrap();
        println!("{}", json);
    }
}
