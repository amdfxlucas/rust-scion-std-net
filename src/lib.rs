
// mod net;


// #![feature(maybe_uninit_uninit_array)]
extern crate libc;
pub use self::ip_addr::IpAddr as IpAddr;
pub use self::ip_v4_addr::Ipv4Addr as Ipv4Addr;
pub use self::ip_v6_addr::Ipv6Addr;
pub use self::ip_v6_addr::Ipv6MulticastScope as Ipv6MulticastScope;
pub use self::scion_addr::ScionAddr as ScionAddr;

pub use self::socket_addr::AddrParseError as AddrParseError;

pub use self::socket_addr::SocketAddr as SocketAddr;
pub use self::socket_addr::AddrKind as AddrKind;
pub use self::sock_addr_scion::SocketAddrScion as SocketAddrScion;

pub use self::sock_addr_v6::SocketAddrV6 as SocketAddrV6;
pub use self::sock_addr_v4::SocketAddrV4 as SocketAddrV4;
pub use self::scion_parse_utils::*;
pub use self::parser::*;
pub use self::display_buffer::*;

pub use self::bitop_impl::*;
pub use self::sock_addr_traits::*;

mod display_buffer;
mod ip_addr;
mod scion_parse_utils;
mod ip_v4_addr;
mod ip_v6_addr;
mod scion_addr;
mod sock_addr_v4;
mod sock_addr_v6;
mod socket_addr;
mod bitop_impl;
mod parser;
mod sock_addr_scion;
mod sock_addr_traits;
// rust/library/core/src/net/mod.rs


#[cfg(test)]
mod tests {
    
    use crate::{as_from_dotted_hex, as_to_dotted_hex};

    use super::{SocketAddr, Ipv4Addr,SocketAddrScion,IpAddr,ScionAddr,make_ia};
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn parse_scion_addr()
    {
        let b = as_from_dotted_hex("ffaa:1:1067");
        assert_eq!(b, 281105609592935);

        assert_eq!(as_to_dotted_hex(b),"ffaa:1:1067");


        let a = SocketAddr::from_str("19-ffaa:1:1067,127.0.0.1:53").unwrap();
        let ia = if let SocketAddr::SCION(SocketAddrScion{ addr, port:_}) =a {addr.get_ia()}else{0};
        let port = if let SocketAddr::SCION(SocketAddrScion{ addr:_, port:p}) =a {p}else{0};
        assert_eq!(port,53);
/*
ia: 5629130167095399 isd: 19 as: 281105609592935

 */
        assert_eq!(make_ia(19,b),5629130167095399);
        assert_eq!(ia,make_ia(19,b));

        assert_eq!( if let SocketAddr::SCION(SocketAddrScion{ addr, port:_}) =a {addr.get_isd()}else{0}, 19);
        assert_eq!( if let SocketAddr::SCION(SocketAddrScion{ addr, port:_}) =a {addr.get_as()}else{0}, 281105609592935);

        assert_eq!(a.to_string(), "19-ffaa:1:1067,127.0.0.1:53");

        let expected = SocketAddr::SCION( SocketAddrScion::new1(ScionAddr::new1(19, b
                                                                , IpAddr::V4( Ipv4Addr::new(127,0,0,1) ) 
                                                            ) ,53) );
        assert_eq!(a, expected);
    }
}