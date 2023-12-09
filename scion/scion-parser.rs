
trait ReadNumberHelper: Sized {
    const ZERO: Self;
    fn checked_mul(&self, other: u32) -> Option<Self>;
    fn checked_add(&self, other: u32) -> Option<Self>;
}

macro_rules! impl_helper {
    ($($t:ty)*) => ($(impl ReadNumberHelper for $t {
        const ZERO: Self = 0;
        #[inline]
        fn checked_mul(&self, other: u32) -> Option<Self> {
            Self::checked_mul(*self, other.try_into().ok()?)
        }
        #[inline]
        fn checked_add(&self, other: u32) -> Option<Self> {
            Self::checked_add(*self, other.try_into().ok()?)
        }
    })*)
}

impl_helper! { u8 u16 u32 }

struct Parser<'a> {
    // Parsing as ASCII, so can use byte array.
    state: &'a [u8],
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Parser<'a> {
        Parser { state: input }
    }

    /// Run a parser, and restore the pre-parse state if it fails.
    fn read_atomically<T, F>(&mut self, inner: F) -> Option<T>
    where
        F: FnOnce(&mut Parser<'_>) -> Option<T>,
    {
        let state = self.state;
        let result = inner(self);
        if result.is_none() {
            self.state = state;
        }
        result
    }

    /// Run a parser, but fail if the entire input wasn't consumed.
    /// Doesn't run atomically.
    fn parse_with<T, F>(&mut self, inner: F, kind: AddrKind) -> Result<T, AddrParseError>
    where
        F: FnOnce(&mut Parser<'_>) -> Option<T>,
    {
        let result = inner(self);
        if self.state.is_empty() { result } else { None }.ok_or(AddrParseError(kind))
    }

    /// Peek the next character from the input
    fn peek_char(&self) -> Option<char> {
        self.state.first().map(|&b| char::from(b))
    }

    /// Read the next character from the input
    fn read_char(&mut self) -> Option<char> {
        self.state.split_first().map(|(&b, tail)| {
            self.state = tail;
            char::from(b)
        })
    }

    #[must_use]
    /// Read the next character from the input if it matches the target.
    fn read_given_char(&mut self, target: char) -> Option<()> {
        self.read_atomically(|p| {
            p.read_char().and_then(|c| if c == target { Some(()) } else { None })
        })
    }

    /// Helper for reading separators in an indexed loop. Reads the separator
    /// character iff index > 0, then runs the parser. When used in a loop,
    /// the separator character will only be read on index > 0 (see
    /// read_ipv4_addr for an example)
    fn read_separator<T, F>(&mut self, sep: char, index: usize, inner: F) -> Option<T>
    where
        F: FnOnce(&mut Parser<'_>) -> Option<T>,
    {
        self.read_atomically(move |p| {
            if index > 0 {
                p.read_given_char(sep)?;
            }
            inner(p)
        })
    }

    // Read a number off the front of the input in the given radix, stopping
    // at the first non-digit character or eof. Fails if the number has more
    // digits than max_digits or if there is no number.
    fn read_number<T: ReadNumberHelper>(
        &mut self,
        radix: u32,
        max_digits: Option<usize>,
        allow_zero_prefix: bool,
    ) -> Option<T> {
        self.read_atomically(move |p| {
            let mut result = T::ZERO;
            let mut digit_count = 0;
            let has_leading_zero = p.peek_char() == Some('0');

            while let Some(digit) = p.read_atomically(|p| p.read_char()?.to_digit(radix)) {
                result = result.checked_mul(radix)?;
                result = result.checked_add(digit)?;
                digit_count += 1;
                if let Some(max_digits) = max_digits {
                    if digit_count > max_digits {
                        return None;
                    }
                }
            }

            if digit_count == 0 {
                None
            } else if !allow_zero_prefix && has_leading_zero && digit_count > 1 {
                None
            } else {
                Some(result)
            }
        })
    }

    /// Read an IPv4 address.
    fn read_ipv4_addr(&mut self) -> Option<Ipv4Addr> {
        self.read_atomically(|p| {
            let mut groups = [0; 4];

            for (i, slot) in groups.iter_mut().enumerate() {
                *slot = p.read_separator('.', i, |p| {
                    // Disallow octal number in IP string.
                    // https://tools.ietf.org/html/rfc6943#section-3.1.1
                    p.read_number(10, Some(3), false)
                })?;
            }

            Some(groups.into())
        })
    }

    

    /// Read an IPv6 Address.
    fn read_ipv6_addr(&mut self) -> Option<Ipv6Addr> {
        /// Read a chunk of an IPv6 address into `groups`. Returns the number
        /// of groups read, along with a bool indicating if an embedded
        /// trailing IPv4 address was read. Specifically, read a series of
        /// colon-separated IPv6 groups (0x0000 - 0xFFFF), with an optional
        /// trailing embedded IPv4 address.
        fn read_groups(p: &mut Parser<'_>, groups: &mut [u16]) -> (usize, bool) {
            let limit = groups.len();

            for (i, slot) in groups.iter_mut().enumerate() {
                // Try to read a trailing embedded IPv4 address. There must be
                // at least two groups left.
                if i < limit - 1 {
                    let ipv4 = p.read_separator(':', i, |p| p.read_ipv4_addr());

                    if let Some(v4_addr) = ipv4 {
                        let [one, two, three, four] = v4_addr.octets();
                        groups[i + 0] = u16::from_be_bytes([one, two]);
                        groups[i + 1] = u16::from_be_bytes([three, four]);
                        return (i + 2, true);
                    }
                }

                let group = p.read_separator(':', i, |p| p.read_number(16, Some(4), true));

                match group {
                    Some(g) => *slot = g,
                    None => return (i, false),
                }
            }
            (groups.len(), false)
        }

        self.read_atomically(|p| {
            // Read the front part of the address; either the whole thing, or up
            // to the first ::
            let mut head = [0; 8];
            let (head_size, head_ipv4) = read_groups(p, &mut head);

            if head_size == 8 {
                return Some(head.into());
            }

            // IPv4 part is not allowed before `::`
            if head_ipv4 {
                return None;
            }

            // Read `::` if previous code parsed less than 8 groups.
            // `::` indicates one or more groups of 16 bits of zeros.
            p.read_given_char(':')?;
            p.read_given_char(':')?;

            // Read the back part of the address. The :: must contain at least one
            // set of zeroes, so our max length is 7.
            let mut tail = [0; 7];
            let limit = 8 - (head_size + 1);
            let (tail_size, _) = read_groups(p, &mut tail[..limit]);

            // Concat the head and tail of the IP address
            head[(8 - tail_size)..8].copy_from_slice(&tail[..tail_size]);

            Some(head.into())
        })
    }

    fn read_scion_addr(& mut self) -> Option<ScionAddress>
    {

        fn read_AS( p:  &mut Parser<'_>) -> Option<u64>
        {    self.read_atomically(|p| {
            let mut groups = [0; 3];

            for (i, slot) in groups.iter_mut().enumerate() {
                *slot = p.read_separator(':', i, |p| {
                    
                    p.read_number(16, Some(4), true)
                })?;
            }

            Some(groups)
        }) }

        self.read_atomically(|p|{ 
            let isd = p.read_number(10, Some(6),true )?;

            p.read_given_char('-')?;

            let as = read_AS(p,)

        });
    }

    /// Read an IP Address, either IPv4 or IPv6.
    fn read_ip_addr(&mut self) -> Option<IpAddr> {
        self.read_ipv4_addr().map(IpAddr::V4).or_else(move || self.read_ipv6_addr().map(IpAddr::V6))
    }

    /// Read a `:` followed by a port in base 10.
    fn read_port(&mut self) -> Option<u16> {
        self.read_atomically(|p| {
            p.read_given_char(':')?;
            p.read_number(10, None, true)
        })
    }

    /// Read a `%` followed by a scope ID in base 10.
    fn read_scope_id(&mut self) -> Option<u32> {
        self.read_atomically(|p| {
            p.read_given_char('%')?;
            p.read_number(10, None, true)
        })
    }

    /// Read an IPv4 address with a port.
    fn read_socket_addr_v4(&mut self) -> Option<SocketAddrV4> {
        self.read_atomically(|p| {
            let ip = p.read_ipv4_addr()?;
            let port = p.read_port()?;
            Some(SocketAddrV4::new(ip, port))
        })
    }

    /// Read an IPv6 address with a port.
    fn read_socket_addr_v6(&mut self) -> Option<SocketAddrV6> {
        self.read_atomically(|p| {
            p.read_given_char('[')?;
            let ip = p.read_ipv6_addr()?;
            let scope_id = p.read_scope_id().unwrap_or(0);
            p.read_given_char(']')?;

            let port = p.read_port()?;
            Some(SocketAddrV6::new(ip, port, 0, scope_id))
        })
    }

    /// Read an IP address with a port
    fn read_socket_addr(&mut self) -> Option<SocketAddr> {
        self.read_socket_addr_v4()
            .map(SocketAddr::V4)
            .or_else(|| self.read_socket_addr_v6().map(SocketAddr::V6))
    }

    fn read_socket_addr_scion(&mut self)-> Option<SocketAddr>
    {
        self.read_atomically ( |p|{
            let scion_addr = p.read_scion()?;
            let port = p.read_port()?;

            Some( SocketAddrScion::new() )
        });
    }
}