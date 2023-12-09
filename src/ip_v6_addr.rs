use std::cmp::Ordering;
use std::mem::transmute;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use std::fmt::{self, Write};
use std::str::FromStr;
use crate::{IpAddr,DisplayBuffer, Ipv4Addr,bitop_impls};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]

pub struct Ipv6Addr {
  pub(crate)  octets: [u8; 16],
}

impl From<std::net::Ipv6Addr> for Ipv6Addr{
    fn from(ip: std::net::Ipv6Addr) -> Ipv6Addr
    {
        Ipv6Addr::from_str(&ip.to_string() ).unwrap()
    }
}

impl Into<std::net::Ipv6Addr> for Ipv6Addr
{
    fn into(self) -> std::net::Ipv6Addr
    {
        std::net::Ipv6Addr::from_str( &self.to_string() ).unwrap()
    }
}

#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]

#[non_exhaustive]
pub enum Ipv6MulticastScope {
    /// Interface-Local scope.
    InterfaceLocal,
    /// Link-Local scope.
    LinkLocal,
    /// Realm-Local scope.
    RealmLocal,
    /// Admin-Local scope.
    AdminLocal,
    /// Site-Local scope.
    SiteLocal,
    /// Organization-Local scope.
    OrganizationLocal,
    /// Global scope.
    Global,
}


impl Ipv6Addr {
    /// Creates a new IPv6 address from eight 16-bit segments.
    ///
    /// The result will represent the IP address `a:b:c:d:e:f:g:h`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
    /// ```
    
    
    #[must_use]
    #[inline]
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        let addr16 = [
            a.to_be(),
            b.to_be(),
            c.to_be(),
            d.to_be(),
            e.to_be(),
            f.to_be(),
            g.to_be(),
            h.to_be(),
        ];
        Ipv6Addr {
            // All elements in `addr16` are big endian.
            // SAFETY: `[u16; 8]` is always safe to transmute to `[u8; 16]`.
            octets: unsafe { transmute::<_, [u8; 16]>(addr16) },
        }
    }

    
    pub const BITS: u32 = 128;

    
    
    #[must_use]
    #[inline]
    pub const fn to_bits(self) -> u128 {
        u128::from_be_bytes(self.octets)
    }

 
    
    #[must_use]
    #[inline]
    pub const fn from_bits(bits: u128) -> Ipv6Addr {
        Ipv6Addr { octets: bits.to_be_bytes() }
    }


    
    #[doc(alias = "IN6ADDR_LOOPBACK_INIT")]
    #[doc(alias = "in6addr_loopback")]
    
    pub const LOCALHOST: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);


    #[doc(alias = "IN6ADDR_ANY_INIT")]
    #[doc(alias = "in6addr_any")]
    
    pub const UNSPECIFIED: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);

  
    
    
    #[must_use]
    #[inline]
    pub const fn segments(&self) -> [u16; 8] {
        // All elements in `self.octets` must be big endian.
        // SAFETY: `[u8; 16]` is always safe to transmute to `[u16; 8]`.
        let [a, b, c, d, e, f, g, h] = unsafe { std::mem::transmute::<_, [u16; 8]>(self.octets) };
        // We want native endian u16
        [
            u16::from_be(a),
            u16::from_be(b),
            u16::from_be(c),
            u16::from_be(d),
            u16::from_be(e),
            u16::from_be(f),
            u16::from_be(g),
            u16::from_be(h),
        ]
    }

    
    
    #[must_use]
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::UNSPECIFIED.octets())
    }

    
    
    
    #[must_use]
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::LOCALHOST.octets())
    }


    
    #[must_use]
    #[inline]
    pub const fn is_global(&self) -> bool {
        !(self.is_unspecified()
            || self.is_loopback()
            // IPv4-mapped Address (`::ffff:0:0/96`)
            || matches!(self.segments(), [0, 0, 0, 0, 0, 0xffff, _, _])
            // IPv4-IPv6 Translat. (`64:ff9b:1::/48`)
            || matches!(self.segments(), [0x64, 0xff9b, 1, _, _, _, _, _])
            // Discard-Only Address Block (`100::/64`)
            || matches!(self.segments(), [0x100, 0, 0, 0, _, _, _, _])
            // IETF Protocol Assignments (`2001::/23`)
            || (matches!(self.segments(), [0x2001, b, _, _, _, _, _, _] if b < 0x200)
                && !(
                    // Port Control Protocol Anycast (`2001:1::1`)
                    u128::from_be_bytes(self.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0001
                    // Traversal Using Relays around NAT Anycast (`2001:1::2`)
                    || u128::from_be_bytes(self.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0002
                    // AMT (`2001:3::/32`)
                    || matches!(self.segments(), [0x2001, 3, _, _, _, _, _, _])
                    // AS112-v6 (`2001:4:112::/48`)
                    || matches!(self.segments(), [0x2001, 4, 0x112, _, _, _, _, _])
                    // ORCHIDv2 (`2001:20::/28`)
                    || matches!(self.segments(), [0x2001, b, _, _, _, _, _, _] if b >= 0x20 && b <= 0x2F)
                ))
            || self.is_documentation()
            || self.is_unique_local()
            || self.is_unicast_link_local())
    }

  
    
    #[must_use]
    #[inline]
    pub const fn is_unique_local(&self) -> bool {
        (self.segments()[0] & 0xfe00) == 0xfc00
    }

  
    
    #[must_use]
    #[inline]
    pub const fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

  
    
    #[must_use]
    #[inline]
    pub const fn is_unicast_link_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }

  
    
    #[must_use]
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
    }

   
    #[must_use]
    #[inline]
    pub const fn is_benchmarking(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0x2) && (self.segments()[2] == 0)
    }

   
    
    #[must_use]
    #[inline]
    pub const fn is_unicast_global(&self) -> bool {
        self.is_unicast()
            && !self.is_loopback()
            && !self.is_unicast_link_local()
            && !self.is_unique_local()
            && !self.is_unspecified()
            && !self.is_documentation()
            && !self.is_benchmarking()
    }

   
    
    #[must_use]
    #[inline]
    pub const fn multicast_scope(&self) -> Option<Ipv6MulticastScope> {
        if self.is_multicast() {
            match self.segments()[0] & 0x000f {
                1 => Some(Ipv6MulticastScope::InterfaceLocal),
                2 => Some(Ipv6MulticastScope::LinkLocal),
                3 => Some(Ipv6MulticastScope::RealmLocal),
                4 => Some(Ipv6MulticastScope::AdminLocal),
                5 => Some(Ipv6MulticastScope::SiteLocal),
                8 => Some(Ipv6MulticastScope::OrganizationLocal),
                14 => Some(Ipv6MulticastScope::Global),
                _ => None,
            }
        } else {
            None
        }
    }

  
    
    
    #[must_use]
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        (self.segments()[0] & 0xff00) == 0xff00
    }


    #[inline]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    
    
    pub const fn to_ipv4_mapped(&self) -> Option<Ipv4Addr> {
        match self.octets() {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, a, b, c, d] => {
                Some(Ipv4Addr::new(a, b, c, d))
            }
            _ => None,
        }
    }

    
    
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_ipv4(&self) -> Option<Ipv4Addr> {
        if let [0, 0, 0, 0, 0, 0 | 0xffff, ab, cd] = self.segments() {
            let [a, b] = ab.to_be_bytes();
            let [c, d] = cd.to_be_bytes();
            Some(Ipv4Addr::new(a, b, c, d))
        } else {
            None
        }
    }

    
    #[inline]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    
    
    pub const fn to_canonical(&self) -> IpAddr {
        if let Some(mapped) = self.to_ipv4_mapped() {
            return IpAddr::V4(mapped);
        }
        IpAddr::V6(*self)
    }

    /// Returns the sixteen eight-bit integers the IPv6 address consists of.
    ///
    /// ```
    /// use std::net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).octets(),
    ///            [255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    
    
    #[must_use]
    #[inline]
    pub const fn octets(&self) -> [u8; 16] {
        self.octets
    }
}

/// Write an Ipv6Addr, conforming to the canonical style described by
/// [RFC 5952](https://tools.ietf.org/html/rfc5952).

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If there are no alignment requirements, write the IP address directly to `f`.
        // Otherwise, write it to a local buffer and then use `f.pad`.
        if f.precision().is_none() && f.width().is_none() {
            let segments = self.segments();

            if let Some(ipv4) = self.to_ipv4_mapped() {
                write!(f, "::ffff:{}", ipv4)
            } else {
                #[derive(Copy, Clone, Default)]
                struct Span {
                    start: usize,
                    len: usize,
                }

                // Find the inner 0 span
                let zeroes = {
                    let mut longest = Span::default();
                    let mut current = Span::default();

                    for (i, &segment) in segments.iter().enumerate() {
                        if segment == 0 {
                            if current.len == 0 {
                                current.start = i;
                            }

                            current.len += 1;

                            if current.len > longest.len {
                                longest = current;
                            }
                        } else {
                            current = Span::default();
                        }
                    }

                    longest
                };

                /// Write a colon-separated part of the address
                #[inline]
                fn fmt_subslice(f: &mut fmt::Formatter<'_>, chunk: &[u16]) -> fmt::Result {
                    if let Some((first, tail)) = chunk.split_first() {
                        write!(f, "{:x}", first)?;
                        for segment in tail {
                            f.write_char(':')?;
                            write!(f, "{:x}", segment)?;
                        }
                    }
                    Ok(())
                }

                if zeroes.len > 1 {
                    fmt_subslice(f, &segments[..zeroes.start])?;
                    f.write_str("::")?;
                    fmt_subslice(f, &segments[zeroes.start + zeroes.len..])
                } else {
                    fmt_subslice(f, &segments)
                }
            }
        } else {
            const LONGEST_IPV6_ADDR: &str = "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff";

            let mut buf = DisplayBuffer::<{ LONGEST_IPV6_ADDR.len() }>::new();
            // Buffer is long enough for the longest possible IPv6 address, so this should never fail.
            write!(buf, "{}", self).unwrap();

            f.pad(buf.as_str())
        }
    }
}


impl PartialOrd for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl fmt::Debug for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}


impl PartialEq<IpAddr> for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => self == v6,
        }
    }
}




impl PartialOrd<IpAddr> for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(_) => Some(Ordering::Greater),
            IpAddr::V6(v6) => self.partial_cmp(v6),
        }
    }
}


impl Ord for Ipv6Addr {
    #[inline]
    fn cmp(&self, other: &Ipv6Addr) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}


impl From<Ipv6Addr> for u128 {
    /// Uses [`Ipv6Addr::to_bits`] to convert an IPv6 address to a host byte order `u128`.
    #[inline]
    fn from(ip: Ipv6Addr) -> u128 {
        ip.to_bits()
    }
}

impl From<u128> for Ipv6Addr {
    /// Uses [`Ipv6Addr::from_bits`] to convert a host byte order `u128` to an IPv6 address.
    #[inline]
    fn from(ip: u128) -> Ipv6Addr {
        Ipv6Addr::from_bits(ip)
    }
}


impl From<[u8; 16]> for Ipv6Addr {
 
    #[inline]
    fn from(octets: [u8; 16]) -> Ipv6Addr {
        Ipv6Addr { octets }
    }
}


impl From<[u16; 8]> for Ipv6Addr {
 
    #[inline]
    fn from(segments: [u16; 8]) -> Ipv6Addr {
        let [a, b, c, d, e, f, g, h] = segments;
        Ipv6Addr::new(a, b, c, d, e, f, g, h)
    }
}


impl Not for Ipv6Addr {
    type Output = Ipv6Addr;

    #[inline]
    fn not(mut self) -> Ipv6Addr {
        for octet in &mut self.octets {
            *octet = !*octet;
        }
        self
    }
}


impl Not for &'_ Ipv6Addr {
    type Output = Ipv6Addr;

    #[inline]
    fn not(self) -> Ipv6Addr {
        !*self
    }
}


bitop_impls! {
  
    
    impl (BitAnd, BitAndAssign) for Ipv6Addr = (bitand, bitand_assign);
    
    impl (BitOr, BitOrAssign) for Ipv6Addr = (bitor, bitor_assign);
}