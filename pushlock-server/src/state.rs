use std::{collections::HashSet, net::IpAddr};

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Locked {
    pub(crate) locked_by: Vec<IpAddr>,
}

#[derive(Default, Clone)]
pub(crate) struct Context {
    pub(crate) locks: HashSet<IpAddr>,
}

impl Context {
    pub(crate) fn get_lock_status(&self, requested_ip: &IpAddr) -> Option<Locked> {
        let locked_by: Vec<IpAddr> = self
            .locks
            .iter()
            .filter(|&ip| ip != requested_ip)
            .copied()
            .collect();

        if !locked_by.is_empty() {
            Some(Locked { locked_by })
        } else {
            None
        }
    }
}
