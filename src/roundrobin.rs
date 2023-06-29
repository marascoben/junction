use std::net::{SocketAddr};

use crate::balancer;
use balancer::LoadBalancer;

pub struct RoundRobin {
    destinations: Vec<SocketAddr>,
    last: usize,
}

impl RoundRobin {
    pub fn new(destinations: Vec<String>) -> RoundRobin {
        let parsed = destinations
            .iter()
            .map(|d| d.parse::<SocketAddr>().unwrap())
            .collect::<Vec<SocketAddr>>();

        RoundRobin {
            destinations: parsed,
            last: 0,
        }
    }
}

impl LoadBalancer for RoundRobin {
    fn next(&mut self) -> Option<SocketAddr> {
        let next = self.last % self.destinations.len();
        // print the next destination
        println!("Next destination: {}", self.destinations[next]);
        self.last += 1;
        Some(self.destinations[next])
    }
}
