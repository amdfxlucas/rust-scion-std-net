use crate::scion_parse_utils::{as_from_ia, as_to_dotted_hex, isd_from_ia, make_ia};
use crate::{IpAddr, Ipv4Addr, Ipv6Addr, Parser, SocketAddrScion, SocketAddrV4, SocketAddrV6};
use std::error::Error;
use std::str::FromStr;

const MAX_BGP_AS_NR: u32 = 4294967295;

/*
The SCION numbering scheme uses a superset of the existing BGP AS num-
bering scheme .
The default format for AS numbers is similar to IPv6.
It uses a 16-bit colon-separated lower-case hexadecimal encoding with leading
zeros omitted. However, the double colon (::) zero-compression feature of
IPv6 is not supported. As for ISD numbers, 0 represents the wildcard AS and
stands for “any AS”, which may be used during path lookup.
The range from 1 to 2^32 - 1 is dedicated to BGP AS numbers.
If a BGP AS supports SCION, it has the same AS number for both BGP and SCION.
To facilitate the comparison with BGP AS numbers, any number in
the BGP AS range is represented as a decimal.
While it is legal to write a BGP AS number using the SCION syntax,
programs should use the decimal representation for display.
For example, if a program receives 0:1:f, it should display it as 65551.
Currently, the 2:0:0/16 range is allocated to public SCION-only ASes (i.e.,
ASes that are not existing BGP ASes). AS numbers in that range should be
assigned in ascending order, without gaps and without vanity numbers
*/

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]

pub struct ScionAddr {
    pub ia: u64,
    pub host: IpAddr,
}

impl Default for ScionAddr {
    fn default() -> Self {
        Self {
            ia: 0,
            host: IpAddr::default(),
        }
    }
}

impl ScionAddr {
    pub fn new(_ia: u64, _host: IpAddr) -> ScionAddr {
        Self {
            ia: _ia,
            host: _host,
        }
    }

    pub fn new1(_isd: u16, _as: u64, _host: IpAddr) -> ScionAddr {
        Self {
            ia: make_ia(_isd, _as),
            host: _host,
        }
    }
    pub fn set_ia(&mut self, ia_: u64) {
        self.ia = ia_;
    }

    pub const fn get_ia(&self) -> u64 {
        self.ia
    }

    pub fn set_isd(&mut self, isd_: u16) {
        self.set_ia(make_ia(isd_, self.get_as()));
    }

    pub fn get_isd(&self) -> u16 {
        isd_from_ia(self.get_ia())
    }

    pub fn get_as(&self) -> u64 {
        as_from_ia(self.get_ia())
    }

    pub fn set_as(&mut self, as_: u64) {
        self.set_ia(make_ia(self.get_isd(), as_));
    }

    pub fn get_host(&self) -> &IpAddr {
        &self.host
    }

    pub fn set_host(&mut self, h: IpAddr) {
        self.host = h;
    }
}

// #[warn(non_snake_case)]
pub fn format_AS(asn: u64) -> String {
    if asn <= MAX_BGP_AS_NR as u64 {
        // print AS number as decimal
        format!("{}", asn)
    } else {
        // print AS number as Hex
        as_to_dotted_hex(asn)
    }
}

impl std::fmt::Display for ScionAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!(
            "{}-{},{}",
            self.get_isd(),
            format_AS(self.get_as()),
            &self.host.to_string()
        ))
    }
}
