use std::{fmt::{Debug,Result,Display,Write}, str::FromStr};
use crate::{IpAddr, Ipv4Addr, Ipv6Addr,SocketAddrScion, SocketAddrV6,  ScionAddr,Parser,DisplayBuffer};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub struct SocketAddrV4 {
    ip: Ipv4Addr,
    port: u16,
}



impl std::fmt::Display for SocketAddrV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If there are no alignment requirements, write the socket address directly to `f`.
        // Otherwise, write it to a local buffer and then use `f.pad`.
        if f.precision().is_none() && f.width().is_none() {
            write!(f, "{}:{}", self.ip(), self.port())
        } else {
            const LONGEST_IPV4_SOCKET_ADDR: &str = "255.255.255.255:65536";

            let mut buf = DisplayBuffer::<{ LONGEST_IPV4_SOCKET_ADDR.len() }>::new();
            // Buffer is long enough for the longest possible IPv4 socket address, so this should never fail.
            write!(buf, "{}:{}", self.ip(), self.port()).unwrap();

            f.pad(buf.as_str())
        }
    }
}


impl std::fmt::Debug for SocketAddrV4 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

impl From<std::net::SocketAddrV4> for SocketAddrV4{
    fn from(sock4: std::net::SocketAddrV4) -> SocketAddrV4
    {
        SocketAddrV4::new( Ipv4Addr::from_str( &sock4.ip().to_string() ).unwrap() , sock4.port())
    }
}

impl Into<std::net::SocketAddrV4> for SocketAddrV4
{
    fn into(self) -> std::net::SocketAddrV4
    {
        std::net::SocketAddrV4::from_str( &self.to_string() ).unwrap()
    }
}

impl SocketAddrV4 {

    
    #[must_use]
    
    #[inline]
    pub const fn new(ip: Ipv4Addr, port: u16) -> SocketAddrV4 {
        SocketAddrV4 { ip, port }
    }

    #[must_use]
    
    
    #[inline]
    pub const fn ip(&self) -> &Ipv4Addr {
        &self.ip
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// socket.set_ip(Ipv4Addr::new(192, 168, 0, 1));
    /// assert_eq!(socket.ip(), &Ipv4Addr::new(192, 168, 0, 1));
    /// ```
    
    #[inline]
    pub fn set_ip(&mut self, new_ip: Ipv4Addr) {
        self.ip = new_ip;
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
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
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// socket.set_port(4242);
    /// assert_eq!(socket.port(), 4242);
    /// ```
    
    #[inline]
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }
}