use crate::{IpAddr, ScionAddr};
use std::fmt::*;

impl std::fmt::Display for SocketAddrScion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{}:{}", self.addr, self.port))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub struct SocketAddrScion {
    pub addr: ScionAddr,
    pub port: u16,
}

impl Default for SocketAddrScion {
    fn default() -> Self {
        Self {
            addr: ScionAddr::default(),
            port: 0,
        }
    }
}

impl SocketAddrScion {
    #[must_use]
    #[inline]
    pub fn new(ia: u64, ip: IpAddr, port: u16) -> SocketAddrScion {
        SocketAddrScion {
            addr: ScionAddr::new(ia, ip),
            port,
        }
    }

    pub fn new1(add: ScionAddr, p: u16) -> SocketAddrScion {
        SocketAddrScion { addr: add, port: p }
    }

    pub fn ia(&self) -> u64 {
        self.addr.get_ia()
    }

    pub fn set_ia(&mut self, ia: u64) {
        self.addr.set_ia(ia)
    }

    #[must_use]
    #[inline]
    pub fn host(&self) -> &IpAddr {
        &self.addr.get_host()
    }

    #[inline]
    pub fn set_host(&mut self, new_ip: IpAddr) {
        self.addr.set_host(new_ip);
    }

    #[must_use]
    #[inline]
    pub const fn port(&self) -> u16 {
        self.port
    }

    #[inline]
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }
}

impl Into<ScionAddr> for SocketAddrScion {
    fn into(self) -> ScionAddr {
        self.addr.clone()
    }
}
