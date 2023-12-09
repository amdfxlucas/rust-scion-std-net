
pub enum L3Addr
{
    IP( IpAddr),
    SCION(ScionAddr),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum SocketAddr {
    /// An IPv4 socket address.
    #[stable(feature = "rust1", since = "1.0.0")]
    V4(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV4),
    /// An IPv6 socket address.
    #[stable(feature = "rust1", since = "1.0.0")]
    V6(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV6),

    SCION(SocketAddrScion),
}


impl SocketAddr {
#[unstable(feature = "addr_parse_ascii", issue = "101035")]
pub fn parse_ascii(b: &[u8]) -> Result<Self, AddrParseError> {
    Parser::new(b).parse_with(|p| p.read_socket_addr(), AddrKind::Socket)
}
}

#[stable(feature = "rust1", since = "1.0.0")]
impl FromStr for SocketAddr {
type Err = AddrParseError;
fn from_str(s: &str) -> Result<SocketAddr, AddrParseError> {
    Self::parse_ascii(s.as_bytes())
}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AddrKind {
    L3Addr,
    Scion, // -> ScionAddr
    Ip, // -> IpAddr (either one of the below 2x)
    Ipv4,
    Ipv6,

    Socket, // L4Addr  -> SocketAddr   (either one of the below 3x)
    SocketScion, // -> SocketAddrScion
    SocketV4,
    SocketV6,
}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddrParseError(AddrKind);

#[stable(feature = "addr_parse_error_error", since = "1.4.0")]
impl fmt::Display for AddrParseError {
    #[allow(deprecated, deprecated_in_future)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

#[stable(feature = "addr_parse_error_error", since = "1.4.0")]
impl Error for AddrParseError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self.0 {
            AddrKind::Ip => "invalid IP address syntax",
            AddrKind::Ipv4 => "invalid IPv4 address syntax",
            AddrKind::Ipv6 => "invalid IPv6 address syntax",
            AddrKind::Socket => "invalid socket address syntax",
            AddrKind::SocketV4 => "invalid IPv4 socket address syntax",
            AddrKind::SocketV6 => "invalid IPv6 socket address syntax",
        }
    }
}