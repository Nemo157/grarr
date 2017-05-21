use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Void {
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Capability {
    SideBand,
    SideBand64K,
    MultiAck,
    MultiAckDetailed,
    Unknown(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Capabilities(Vec<Capability>);

impl Capabilities {
    pub fn new(caps: Vec<Capability>) -> Capabilities {
        Capabilities(caps)
    }

    pub fn empty() -> Capabilities {
        Capabilities(Vec::new())
    }

    pub fn contains(&self, cap: Capability) -> bool {
        self.0.contains(&cap)
    }
}

impl FromStr for Capability {
    type Err = Void;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "side-band" => Capability::SideBand,
            "side-band-64k" => Capability::SideBand64K,
            "multi_ack" => Capability::MultiAck,
            "multi_ack_detailed" => Capability::MultiAckDetailed,
            s => Capability::Unknown(s.to_owned()),
        })
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Capability::SideBand => "side-band",
            Capability::SideBand64K => "side-band-64k",
            Capability::MultiAck => "multi_ack",
            Capability::MultiAckDetailed => "multi_ack_detailed",
            Capability::Unknown(ref s) => &**s,
        })
    }
}

impl FromStr for Capabilities {
    type Err = Void;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Capabilities(s
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect()))
    }
}

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cap in &self.0 {
            fmt::Display::fmt(cap, f)?;
            f.write_str(" ")?;
        }
        Ok(())
    }
}
