use std::cmp::Ordering;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use std::fmt::{self, Write};
use crate::{Ipv6Addr, IpAddr, DisplayBuffer, bitop_impls};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]

pub struct Ipv4Addr {
    octets: [u8; 4],
}

impl From<std::net::Ipv4Addr> for Ipv4Addr{
    fn from(ip: std::net::Ipv4Addr) -> Ipv4Addr
    {
        Ipv4Addr::from_str(&ip.to_string() ).unwrap()
    }
}

impl Into<std::net::Ipv4Addr> for Ipv4Addr
{
    fn into(self) -> std::net::Ipv4Addr
    {
        std::net::Ipv4Addr::from_str( &self.to_string() ).unwrap()
    }
}
impl PartialOrd<IpAddr> for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(v4) => self.partial_cmp(v4),
            IpAddr::V6(_) => Some(Ordering::Less),
        }
    }
}



impl fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.octets();

        // If there are no alignment requirements, write the IP address directly to `f`.
        // Otherwise, write it to a local buffer and then use `f.pad`.
        if fmt.precision().is_none() && fmt.width().is_none() {
            write!(fmt, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
        } else {
            const LONGEST_IPV4_ADDR: &str = "255.255.255.255";

            let mut buf = DisplayBuffer::<{ LONGEST_IPV4_ADDR.len() }>::new();
            // Buffer is long enough for the longest possible IPv4 address, so this should never fail.
            write!(buf, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]).unwrap();

            fmt.pad(buf.as_str())
        }
    }
}


impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}


impl Ord for Ipv4Addr {
    #[inline]
    fn cmp(&self, other: &Ipv4Addr) -> Ordering {
        self.octets.cmp(&other.octets)
    }
}


impl PartialEq<IpAddr> for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(v4) => self == v4,
            IpAddr::V6(_) => false,
        }
    }
}


impl PartialOrd for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl From<Ipv4Addr> for u32 {
    /// Uses [`Ipv4Addr::to_bits`] to convert an IPv4 address to a host byte order `u32`.
    #[inline]
    fn from(ip: Ipv4Addr) -> u32 {
        ip.to_bits()
    }
}


impl From<u32> for Ipv4Addr {
    /// Uses [`Ipv4Addr::from_bits`] to convert a host byte order `u32` into an IPv4 address.
    #[inline]
    fn from(ip: u32) -> Ipv4Addr {
        Ipv4Addr::from_bits(ip)
    }
}


impl From<[u8; 4]> for Ipv4Addr {

    #[inline]
    fn from(octets: [u8; 4]) -> Ipv4Addr {
        Ipv4Addr { octets }
    }
}


impl Ipv4Addr {
    /// Creates a new IPv4 address from four eight-bit octets.
    ///
    /// The result will represent the IP address `a`.`b`.`c`.`d`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// ```
    
    
    #[must_use]
    #[inline]
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr { octets: [a, b, c, d] }
    }

    pub const BITS: u32 = 32;

    
    
    #[must_use]
    #[inline]
    pub const fn to_bits(self) -> u32 {
        u32::from_be_bytes(self.octets)
    }


    
    #[must_use]
    #[inline]
    pub const fn from_bits(bits: u32) -> Ipv4Addr {
        Ipv4Addr { octets: bits.to_be_bytes() }
    }

    
    pub const LOCALHOST: Self = Ipv4Addr::new(127, 0, 0, 1);


    #[doc(alias = "INADDR_ANY")]
    
    pub const UNSPECIFIED: Self = Ipv4Addr::new(0, 0, 0, 0);


    
    pub const BROADCAST: Self = Ipv4Addr::new(255, 255, 255, 255);


    
    
    #[must_use]
    #[inline]
    pub const fn octets(&self) -> [u8; 4] {
        self.octets
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        u32::from_be_bytes(self.octets) == 0
    }

  
    
    
    #[must_use]
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        self.octets()[0] == 127
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_private(&self) -> bool {
        match self.octets() {
            [10, ..] => true,
            [172, b, ..] if b >= 16 && b <= 31 => true,
            [192, 168, ..] => true,
            _ => false,
        }
    }


    
    
    #[must_use]
    #[inline]
    pub const fn is_link_local(&self) -> bool {
        matches!(self.octets(), [169, 254, ..])
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_global(&self) -> bool {
        !(self.octets()[0] == 0 // "This network"
            || self.is_private()
            || self.is_shared()
            || self.is_loopback()
            || self.is_link_local()
            // addresses reserved for future protocols (`192.0.0.0/24`)
            ||(self.octets()[0] == 192 && self.octets()[1] == 0 && self.octets()[2] == 0)
            || self.is_documentation()
            || self.is_benchmarking()
            || self.is_reserved()
            || self.is_broadcast())
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_shared(&self) -> bool {
        self.octets()[0] == 100 && (self.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_benchmarking(&self) -> bool {
        self.octets()[0] == 198 && (self.octets()[1] & 0xfe) == 18
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_reserved(&self) -> bool {
        self.octets()[0] & 240 == 240 && !self.is_broadcast()
    }


    
    
    #[must_use]
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        self.octets()[0] >= 224 && self.octets()[0] <= 239
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_broadcast(&self) -> bool {
        u32::from_be_bytes(self.octets()) == u32::from_be_bytes(Self::BROADCAST.octets())
    }


    
    
    #[must_use]
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        matches!(self.octets(), [192, 0, 2, _] | [198, 51, 100, _] | [203, 0, 113, _])
    }

    
    
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_ipv6_compatible(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr { octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, a, b, c, d] }
    }

   
    
    
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_ipv6_mapped(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr { octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, a, b, c, d] }
    }
}



impl Not for Ipv4Addr {
    type Output = Ipv4Addr;

    #[inline]
    fn not(mut self) -> Ipv4Addr {
        for octet in &mut self.octets {
            *octet = !*octet;
        }
        self
    }
}


impl Not for &'_ Ipv4Addr {
    type Output = Ipv4Addr;

    #[inline]
    fn not(self) -> Ipv4Addr {
        !*self
    }
}

bitop_impls! {
    
    impl (BitAnd, BitAndAssign) for Ipv4Addr = (bitand, bitand_assign);
    
    impl (BitOr, BitOrAssign) for Ipv4Addr = (bitor, bitor_assign);
}