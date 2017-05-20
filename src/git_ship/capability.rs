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
