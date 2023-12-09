#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct ScionAddr {
    ia: u64,
    host: IpAddr,
}



impl SocketAddr {

    
    #[stable(feature = "ip_addr", since = "1.7.0")]
    #[must_use]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn new_ip(ip: IpAddr, port: u16) -> SocketAddr {
        match ip {
            IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, port)),
            IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, port, 0, 0)),
        }
    }

    pub const fn new_scion(ia: u64, ip: IpAddr, port: u16) -> SocketAddr {
        SocketAddr::SCION(SocketAddrScion::new(ia, ip,port ) )
    }
 
    #[must_use]
    #[stable(feature = "ip_addr", since = "1.7.0")]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn host(&self) -> IpAddr {
        if matches!(*self, SocketAddr::SCION(_)) 
        {
            < self as SocketAddr::SCION> ::host()
        } else
        {
        match *self {
            SocketAddr::V4(ref a) => IpAddr::V4(*a.ip()),
            SocketAddr::V6(ref a) => IpAddr::V6(*a.ip()),
        }
    }
    }


    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    #[inline]
    pub fn set_ip(&mut self, new_ip: IpAddr) {
        // `match (*self, new_ip)` would have us mutate a copy of self only to throw it away.
        match (self, new_ip) {
            (&mut SocketAddr::V4(ref mut a), IpAddr::V4(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::V6(ref mut a), IpAddr::V6(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::SCION(ref mut a), _) => a.set_host(new_ip),

            (self_, new_ip) => 
            {
                *self_ = Self::new_ip(new_ip, self_.port()) 
            }, 
        }
    }

    pub fn set_host(&mut self, new_host: L3Addr) {
        
        if matches!( new_host, L3Addr::SCION(ScionAddr(ia,host)))
        {
            match(self)
            {
                &mut SocketAddr::SCION( ref mut a) => { a.set_host(host); },
                (&mut SocketAddr::V4(ref mut a)) => a.set_ip(host),
                (&mut SocketAddr::V6(ref mut a)) => a.set_ip(host),
            }
        }else{
            let  L3Addr::IP( new_ip) = new_host;

        match (self, new_ip) {
            (&mut SocketAddr::V4(ref mut a), IpAddr::V4(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::V6(ref mut a), IpAddr::V6(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::SCION(ref mut a), _) => a.set_host(new_ip),

            (self_, new_ip) => 
            {
                *self_ = Self::new_ip(new_ip, self_.port()) 
            },             
        }
        }
    }


    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn port(&self) -> u16 {
        match *self {
            SocketAddr::V4(ref a) => a.port(),
            SocketAddr::V6(ref a) => a.port(),
            SocketAddr::SCION(ref a ) => a.port(),
        }
    }


    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    #[inline]
    pub fn set_port(&mut self, new_port: u16) {
        match *self {
            SocketAddr::V4(ref mut a) => a.set_port(new_port),
            SocketAddr::V6(ref mut a) => a.set_port(new_port),
            SocketAddr::SCION(ref mut a ) => a.set_port(new_port),
        }
    }

    #[must_use]
    #[stable(feature = "sockaddr_checker", since = "1.16.0")]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn is_ipv4(&self) -> bool {
        matches!(*self, SocketAddr::V4(_))
    }


    #[must_use]
    #[stable(feature = "sockaddr_checker", since = "1.16.0")]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn is_ipv6(&self) -> bool {
        matches!(*self, SocketAddr::V6(_))
    }
}



#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct SocketAddrScion {
  l3addr: ScionAddr,
    port: u16,
}


impl SocketAddrScion {
 
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn new(ia: u64, ip: IpvAddr, port: u16) -> SocketAddrScion {
        SocketAddrScion { l3addr: ScionAddr::new(ia, ip), port }
    }

    pub const fn ia(&self) ->u64
    {
        l3addr.ia()
    }

    pub const fn set_ia(ia: u64)
    {
        l3addr.set_ia(ia)
    }

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn host(&self) -> &IpvAddr {
        &self.l3addr.ip()
    }


    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    #[inline]
    pub fn set_host(&mut self, new_ip: IpvAddr) {
        self.ip = new_ip;
    }

 
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_socketaddr", since = "1.69.0")]
    #[inline]
    pub const fn port(&self) -> u16 {
        self.port
    }


    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    #[inline]
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }
}