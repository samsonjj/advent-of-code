use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Name([u8; 2]);

impl FromStr for Name {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.bytes().collect::<Vec<_>>().take(2);
    }
}

struct Valve {
    name: Name,
    flow_rate: u16,
    links: Vec<Name>,
}

impl TryFrom<&str> for Valve {
    type Error = regex::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"$Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)")?;

        let caps = regex.captures(regex)?;

        Ok(Valve {
            name: caps.get(1).unwrap().as_str().parse(),
            flow_rate: caps.get(2).unwrap(),
            links: caps.get(3).unwrap().split
        })
    }
}

#[derive(Default)]
struct Network {
    valves: Vec<Valve>,
    connections: HashMap<Name, Vec<Name>>,
}

impl TryFrom<&str> for Network {
    type Error = regex::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"$Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)")?;
        let mut network = Network::default();

        let valves = input.lines().map(|l| {
            regex.captures(l).
        })
    }
}

// Valve ZO has flow rate=0; tunnels lead to valves GY, QM
pub fn parse() {}
