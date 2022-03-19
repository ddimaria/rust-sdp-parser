use crate::error::Result;
use crate::utils::parse_number;

/// SDP Time
///
/// t=0 0
/// Gives the starting and ending time. When they are both set to 0 like our
/// case it means that the session is not bounded to a specific timing- in
/// other words it’s permanent and valid at any time.
#[derive(Debug, Default, Serialize, PartialEq)]
pub(crate) struct Time {
    pub start_time: u64,
    pub stop_time: u64,
    pub bounded: bool,
}

impl<'a> Time {
    pub(crate) fn new(value: &'a str) -> Result<Self> {
        let mut split = value.split(' ');
        let start_time = parse_number::<u64>(split.next(), 1)?;
        let stop_time = parse_number::<u64>(split.next(), 2)?;
        let bounded = !(start_time == 0 && stop_time == 0);

        Ok(Self {
            start_time,
            stop_time,
            bounded,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_time() {
        let time = "0 0";
        let parsed = Time::new(time).unwrap();
        let expected = Time {
            start_time: 0,
            stop_time: 0,
            bounded: false,
        };

        assert_eq!(parsed, expected);
    }
}
