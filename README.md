# SDP Parser
Parse a WebRTC Session Description Protocol message.
Accepts a SDP string and parses into a Rust struct and outputs JSON.

## The parse() function
The main function is parse(), which accepts a SDP &str and parses it.

```rust
use sdp_parser::parse;

let sdp = parse(sdp_string)?;
```

## Accessing Parsed Attributes

All struct attributes are public, so accessing their values is straightforward:

```rust
use sdp_parser::parse;

let sdp = parse(sdp_string)?;
let first_media = sdp.media.get(0)?;
println!("{:#?}", first_media);
```

### Output:
```rust
Media {
    type: "audio",
    port: 54400,
    protocol: "RTP/SAVPF",
    payloads: "0",
    candidates: [
        Candidate {
            component: 0,
            foundation: "1",
            transport: "UDP",
            priority: 2113667327,
            ip: "203.0.113.1",
            port: 54400,
            type: "host",
        },
        Candidate {
            component: 1,
            foundation: "2",
            transport: "UDP",
            priority: 2113667326,
            ip: "203.0.113.1",
            port: 54401,
            type: "host",
        },
    ],
    direction: "sendrecv",
    fmtp: [],
    ptime: 20,
    rtpmap: [
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
    rtc_fb: [],
    ssrc: [],
}
```

## JSON Output
The `sdp.to_json()` function returns the parsed SDP in JSON format.

```rust
use sdp_parser::parse;

let sdp = parse(sdp_string)?;
println!("{}:?}", sdp.to_json()?);
```

SDP to test: 
```text
v=0
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
a=ssrc:1399694169 baz
```

Expected output: 
```json
{
  "version": 0,
  "session_name": "",
  "ice_ufrag": "F7gI",
  "ice_pwd": "x9cml/YzichV2+XlhiMu8g",
  "fingerprint": {
    "type": "sha-1",
    "hash": "42:89:c5:c6:55:9d:6e:c8:e8:83:55:2a:39:f9:b6:eb:e9:a3:a9:e7"
  },
  "origin": {
    "username": "-",
    "session_id": 20518,
    "session_version": 0,
    "network_type": "IN",
    "ip_type": "IP4",
    "ip_address": "203.0.113.1"
  },
  "time": {
    "start_time": 0,
    "stop_time": 0,
    "bounded": false
  },
  "connection": {
    "network_type": "IN",
    "ip_type": "IP4",
    "ip_address": "203.0.113.1"
  },
  "media": [
    {
      "type": "audio",
      "port": 54400,
      "protocol": "RTP/SAVPF",
      "payloads": "0",
      "candidates": [
        {
          "component": 0,
          "foundation": "1",
          "transport": "UDP",
          "priority": 2113667327,
          "ip": "203.0.113.1",
          "port": 54400,
          "type": "host"
        },
        {
          "component": 1,
          "foundation": "2",
          "transport": "UDP",
          "priority": 2113667326,
          "ip": "203.0.113.1",
          "port": 54401,
          "type": "host"
        }
      ],
      "direction": "sendrecv",
      "fmtp": [],
      "ptime": 20,
      "rtpmap": [
        {
          "codec": "PCMU",
          "payload": "0",
          "rate": 8000
        },
        {
          "codec": "opus",
          "payload": "96",
          "rate": 48000
        }
      ],
      "rtc_fb": [],
      "ssrc": []
    },
    {
      "type": "video",
      "port": 55400,
      "protocol": "RTP/SAVPF",
      "payloads": "97",
      "candidates": [
        {
          "component": 0,
          "foundation": "1",
          "transport": "UDP",
          "priority": 2113667327,
          "ip": "203.0.113.1",
          "port": 55400,
          "type": "host"
        },
        {
          "component": 1,
          "foundation": "2",
          "transport": "UDP",
          "priority": 2113667326,
          "ip": "203.0.113.1",
          "port": 55401,
          "type": "host"
        }
      ],
      "direction": "sendrecv",
      "fmtp": [
        {
          "config": "profile-level-id=4d0028;packetization-mode=1",
          "payload": 97
        }
      ],
      "ptime": 0,
      "rtpmap": [
        {
          "codec": "H264",
          "payload": "97",
          "rate": 90000
        },
        {
          "codec": "VP8",
          "payload": "98",
          "rate": 90000
        }
      ],
      "rtc_fb": [
        {
          "payload": "*",
          "type": "nack"
        },
        {
          "payload": "97",
          "type": "trr-int"
        },
        {
          "payload": "97",
          "type": "nack"
        },
        {
          "payload": "98",
          "type": "trr-int"
        },
        {
          "payload": "98",
          "type": "nack"
        }
      ],
      "ssrc": [
        {
          "id": 1399694169,
          "attribute": "foo",
          "value": "bar"
        },
        {
          "id": 1399694169,
          "attribute": "baz",
          "value": null
        }
      ]
    }
  ]
}
```