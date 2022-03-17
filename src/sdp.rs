use crate::error::{Error, Result};
use crate::origin::Origin;
use crate::utils::parse_number;

#[derive(Debug, Default)]
pub struct Sdp<'a> {
    pub(crate) version: u32,
    pub(crate) origin: Origin<'a>,
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
            "v" => self.parse_version(&value)?,
            "o" => self.parse_origin(&value)?,
            "s" => {}
            _ => {}
        };

        println!("{:?} {:?}", key, value);

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
        let parsed = Sdp::parse(SDP);
        println!("{:?}", parsed);
    }
}
