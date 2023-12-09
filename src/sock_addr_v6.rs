
use std::fmt::*;
use crate::{IpAddr, Ipv4Addr, Ipv6Addr,SocketAddrScion,  SocketAddrV4, ScionAddr,Parser,DisplayBuffer};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub struct SocketAddrV6 {
    ip: Ipv6Addr,
    port: u16,
    flowinfo: u32,
    scope_id: u32,
}


impl std::fmt::Display for SocketAddrV6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If there are no alignment requirements, write the socket address directly to `f`.
        // Otherwise, write it to a local buffer and then use `f.pad`.
        if f.precision().is_none() && f.width().is_none() {
            match self.scope_id() {
                0 => write!(f, "[{}]:{}", self.ip(), self.port()),
                scope_id => write!(f, "[{}%{}]:{}", self.ip(), scope_id, self.port()),
            }
        } else {
            const LONGEST_IPV6_SOCKET_ADDR: &str =
                "[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff%4294967296]:65536";

            let mut buf = DisplayBuffer::<{ LONGEST_IPV6_SOCKET_ADDR.len() }>::new();
            match self.scope_id() {
                0 => write!(buf, "[{}]:{}", self.ip(), self.port()),
                scope_id => write!(buf, "[{}%{}]:{}", self.ip(), scope_id, self.port()),
            }
            // Buffer is long enough for the longest possible IPv6 socket address, so this should never fail.
            .unwrap();

            f.pad(buf.as_str())
        }
    }
}


impl std::fmt::Debug for SocketAddrV6 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

impl SocketAddrV6 {

    
    #[must_use]
    
    #[inline]
    pub const fn new(ip: Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> SocketAddrV6 {
        SocketAddrV6 { ip, port, flowinfo, scope_id }
    }


    #[must_use]
    
    
    #[inline]
    pub const fn ip(&self) -> &Ipv6Addr {
        &self.ip
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// socket.set_ip(Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));
    /// assert_eq!(socket.ip(), &Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));
    /// ```
    
    #[inline]
    pub fn set_ip(&mut self, new_ip: Ipv6Addr) {
        self.ip = new_ip;
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// assert_eq!(socket.port(), 8080);
    /// ```
    #[must_use]
    
    
    #[inline]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Changes the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// socket.set_port(4242);
    /// assert_eq!(socket.port(), 4242);
    /// ```
    
    #[inline]
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }

    /// Returns the flow information associated with this address.
    ///
    /// This information corresponds to the `sin6_flowinfo` field in C's `netinet/in.h`,
    /// as specified in [IETF RFC 2553, Section 3.3].
    /// It combines information about the flow label and the traffic class as specified
    /// in [IETF RFC 2460], respectively [Section 6] and [Section 7].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    /// [IETF RFC 2460]: https://tools.ietf.org/html/rfc2460
    /// [Section 6]: https://tools.ietf.org/html/rfc2460#section-6
    /// [Section 7]: https://tools.ietf.org/html/rfc2460#section-7
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    /// assert_eq!(socket.flowinfo(), 10);
    /// ```
    #[must_use]
    
    
    #[inline]
    pub const fn flowinfo(&self) -> u32 {
        self.flowinfo
    }

    /// Changes the flow information associated with this socket address.
    ///
    /// See [`SocketAddrV6::flowinfo`]'s documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    /// socket.set_flowinfo(56);
    /// assert_eq!(socket.flowinfo(), 56);
    /// ```
    
    #[inline]
    pub fn set_flowinfo(&mut self, new_flowinfo: u32) {
        self.flowinfo = new_flowinfo;
    }

    /// Returns the scope ID associated with this address.
    ///
    /// This information corresponds to the `sin6_scope_id` field in C's `netinet/in.h`,
    /// as specified in [IETF RFC 2553, Section 3.3].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    /// assert_eq!(socket.scope_id(), 78);
    /// ```
    #[must_use]
    
    
    #[inline]
    pub const fn scope_id(&self) -> u32 {
        self.scope_id
    }

    /// Changes the scope ID associated with this socket address.
    ///
    /// See [`SocketAddrV6::scope_id`]'s documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    /// socket.set_scope_id(42);
    /// assert_eq!(socket.scope_id(), 42);
    /// ```
    
    #[inline]
    pub fn set_scope_id(&mut self, new_scope_id: u32) {
        self.scope_id = new_scope_id;
    }
}

impl From<std::net::SocketAddrV6> for SocketAddrV6{
    fn from(sock6: std::net::SocketAddrV6) -> SocketAddrV6
    {
        SocketAddrV6::from_str(&sock6.to_string() ).unwrap()
    }
}

impl Into<std::net::SocketAddrV6> for SocketAddrV6
{
    fn into(self) -> std::net::SocketAddrV6
    {
        std::net::SocketAddrV6::from_str( &self.to_string() ).unwrap()
    }
}