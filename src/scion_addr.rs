
use std::error::Error;
use std::str::FromStr;
use crate::{IpAddr, Ipv4Addr, Ipv6Addr,SocketAddrScion, SocketAddrV6, SocketAddrV4, Parser};
use crate::scion_parse_utils::{ make_ia ,as_to_dotted_hex, as_from_ia, isd_from_ia};

#[derive(Copy, Clone, PartialEq, Eq, Hash,Ord,PartialOrd,Debug)]

pub struct ScionAddr {
  pub  ia: u64,
    pub host: IpAddr,
}


impl ScionAddr
{
    pub fn new( _ia: u64, _host: IpAddr)->ScionAddr
    {
        Self{ia:_ia, host: _host }
    }

    pub fn new1( _isd: u16, _as: u64, _host: IpAddr ) -> ScionAddr
    {
        Self{ ia: make_ia(_isd,_as), host: _host }
    }
    pub fn set_ia(&mut self, ia_: u64) 
    {
        self.ia  = ia_;
    }

    pub const fn get_ia(&self) -> u64
    {
        self.ia
    }

    pub fn set_isd(&mut self,isd_: u16)
    {
        self.set_ia( make_ia(isd_, self.get_as()) );
    }

    pub fn get_isd(&self) -> u16
    {
        isd_from_ia(self.get_ia())
    }

    pub fn get_as(&self) ->u64
    {
        as_from_ia( self.get_ia() )
    }

    pub fn set_as( &mut self, as_: u64)
    {
        self.set_ia(make_ia(self.get_isd(), as_));
    }

    pub fn get_host(&self)-> &IpAddr
    { & self.host }

    pub fn set_host(&mut self, h: IpAddr)
    {
        self.host=h;
    }
}


impl std::fmt::Display for ScionAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad( & format!("{}-{},{}", self.get_isd()
        , as_to_dotted_hex(self.get_as() ), & self.host.to_string() ) )
    }
}