use crate::{
    DisplayBuffer, IpAddr, Ipv4Addr, Ipv6Addr, Parser, ScionAddr, SocketAddrScion, SocketAddrV4,
    SocketAddrV6,
};
use std::error::Error;
use std::fmt::{self, Write};
use std::str::FromStr;

pub enum L3Addr {
    IP(IpAddr),
    SCION(ScionAddr),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub enum SocketAddr {
    /// An IPv4 socket address.
    V4(SocketAddrV4),
    /// An IPv6 socket address.
    V6(SocketAddrV6),

    SCION(SocketAddrScion),
}

impl SocketAddr {
    pub fn parse_ascii(b: &[u8]) -> Result<Self, AddrParseError> {
        Parser::new(b).parse_with(|p| p.read_socket_addr(), AddrKind::Socket)
    }
}

impl FromStr for SocketAddr {
    type Err = AddrParseError;
    fn from_str(s: &str) -> Result<SocketAddr, AddrParseError> {
        Self::parse_ascii(s.as_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddrKind {
    L3Addr,
    Scion, // -> ScionAddr
    Ip,    // -> IpAddr (either one of the below 2x)
    Ipv4,
    Ipv6,

    Socket,      // L4Addr  -> SocketAddr   (either one of the below 3x)
    SocketScion, // -> SocketAddrScion
    SocketV4,
    SocketV6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddrParseError(pub AddrKind);

impl fmt::Display for AddrParseError {
    #[allow(deprecated, deprecated_in_future)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for AddrParseError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self.0 {
            AddrKind::Ip => "invalid IP address syntax",
            AddrKind::Ipv4 => "invalid IPv4 address syntax",
            AddrKind::Ipv6 => "invalid IPv6 address syntax",
            AddrKind::Scion => "invalid Scion address syntax",
            AddrKind::SocketScion => "invalid ScionSocket address syntax",
            AddrKind::L3Addr => "invalid L3Address syntax",
            AddrKind::Socket => "invalid socket address syntax",
            AddrKind::SocketV4 => "invalid IPv4 socket address syntax",
            AddrKind::SocketV6 => "invalid IPv6 socket address syntax",
        }
    }
}

impl From<SocketAddrV4> for SocketAddr {
    /// Converts a [`SocketAddrV4`] into a [`SocketAddr::V4`].
    #[inline]
    fn from(sock4: SocketAddrV4) -> SocketAddr {
        SocketAddr::V4(sock4)
    }
}

impl From<SocketAddrV6> for SocketAddr {
    /// Converts a [`SocketAddrV6`] into a [`SocketAddr::V6`].
    #[inline]
    fn from(sock6: SocketAddrV6) -> SocketAddr {
        SocketAddr::V6(sock6)
    }
}

impl<I: Into<IpAddr>> From<(I, u16)> for SocketAddr {
    /// Converts a tuple struct (Into<[`IpAddr`]>, `u16`) into a [`SocketAddr`].
    ///
    /// This conversion creates a [`SocketAddr::V4`] for an [`IpAddr::V4`]
    /// and creates a [`SocketAddr::V6`] for an [`IpAddr::V6`].
    ///
    /// `u16` is treated as port of the newly created [`SocketAddr`].
    fn from(pieces: (I, u16)) -> SocketAddr {
        SocketAddr::new_ip(pieces.0.into(), pieces.1)
    }
}
/*
impl<I: Into<ScionAddr>> From<(I, u16)> for SocketAddr {
    fn from(pieces: (I, u16)) -> SocketAddr {
        SocketAddr::new_scion(pieces.0.into(), pieces.1)
    }
}*/

impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl SocketAddr {
    #[must_use]
    #[inline]
    pub fn new_ip(ip: IpAddr, port: u16) -> SocketAddr {
        match ip {
            IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, port)),
            IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, port, 0, 0)),
        }
    }

    pub fn new_scion(ia: u64, ip: IpAddr, port: u16) -> SocketAddr {
        SocketAddr::SCION(SocketAddrScion::new(ia, ip, port))
    }

    #[must_use]
    #[inline]
    pub fn host(&self) -> IpAddr {
        match &self {
            SocketAddr::SCION(addr) => *addr.host(),
            SocketAddr::V4(ref a) => IpAddr::V4(*a.ip()),

            SocketAddr::V6(ref a) => IpAddr::V6(*a.ip()),
        }
    }

    #[inline]
    pub fn set_ip(&mut self, new_ip: IpAddr) {
        // `match (*self, new_ip)` would have us mutate a copy of self only to throw it away.
        match (self, new_ip) {
            (&mut SocketAddr::V4(ref mut a), IpAddr::V4(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::V6(ref mut a), IpAddr::V6(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::SCION(ref mut a), _) => a.set_host(new_ip),

            (self_, new_ip) => *self_ = Self::new_ip(new_ip, self_.port()),
        }
    }

    pub fn set_host(&mut self, new_host: L3Addr) {
        match new_host {
            L3Addr::SCION(ScionAddr { ia, host }) => match (self) {
                &mut SocketAddr::SCION(ref mut a) => {
                    a.set_host(host);
                }
                (&mut SocketAddr::V4(ref mut a)) => match host {
                    IpAddr::V4(h) => a.set_ip(h),
                    _ => {}
                },
                (&mut SocketAddr::V6(ref mut a)) => match host {
                    IpAddr::V6(h) => a.set_ip(h),
                    _ => {}
                },
            },
            L3Addr::IP(new_ip) => match (self, new_ip) {
                (&mut SocketAddr::V4(ref mut a), IpAddr::V4(new_ip)) => a.set_ip(new_ip),
                (&mut SocketAddr::V6(ref mut a), IpAddr::V6(new_ip)) => a.set_ip(new_ip),
                (&mut SocketAddr::SCION(ref mut a), _) => a.set_host(new_ip),

                (self_, new_ip) => *self_ = Self::new_ip(new_ip, self_.port()),
            },
        }
    }

    #[must_use]
    #[inline]
    pub const fn port(&self) -> u16 {
        match *self {
            SocketAddr::V4(ref a) => a.port(),
            SocketAddr::V6(ref a) => a.port(),
            SocketAddr::SCION(ref a) => a.port(),
        }
    }

    #[inline]
    pub fn set_port(&mut self, new_port: u16) {
        match *self {
            SocketAddr::V4(ref mut a) => a.set_port(new_port),
            SocketAddr::V6(ref mut a) => a.set_port(new_port),
            SocketAddr::SCION(ref mut a) => a.set_port(new_port),
        }
    }

    #[must_use]
    #[inline]
    pub fn is_ipv4(&self) -> bool {
        matches!(*self, SocketAddr::V4(_))
    }

    #[must_use]
    #[inline]
    pub fn is_ipv6(&self) -> bool {
        matches!(*self, SocketAddr::V6(_))
    }
}

impl From<SocketAddrScion> for SocketAddr {
    /// Converts a [`SocketAddrScion`] into a [`SocketAddr::SCION`].
    #[inline]
    fn from(sock: SocketAddrScion) -> SocketAddr {
        SocketAddr::SCION(sock)
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SocketAddr::V4(ref a) => a.fmt(f),
            SocketAddr::V6(ref a) => a.fmt(f),
            SocketAddr::SCION(ref a) => a.fmt(f),
        }
    }
}

impl From<std::net::SocketAddr> for SocketAddr{
    fn from(sock6: std::net::SocketAddr) -> SocketAddr
    {
        SocketAddr::from_str(&sock6.to_string() ).unwrap()
    }
}

impl Into<std::net::SocketAddr> for SocketAddr
{
    fn into(self) -> std::net::SocketAddr
    {
        std::net::SocketAddr::from_str( &self.to_string() ).unwrap()
    }
}