/**
  Socket address module. Wraps around the underlying C equivalent (sockaddr, sockaddr_in, sockaddr_in6, etc...).
*/
use std::mem;
use std::num::Int;

use c::consts::*;
use c::types::{sockaddr, sa_family_t, sockaddr_in, in_addr, in6_addr, sockaddr_in6, sockaddr_un};

type SocketAddressFamily = sa_family_t; //most likely u16
type PortInt = u16;

//Eventually support all socket address type but also provide a custom one, for a fallback mechanism.
//It could hold a *void and a size_t.
pub enum SocketAddress {
  Ipv4( SocketAddressIpv4 ),
  Ipv6( SocketAddressIpv6 ),
  Unix( SocketAddressUnix ),
}

pub struct SocketAddressIpv4 {
  payload: sockaddr_in
}

pub struct SocketAddressIpv6 {
  payload: sockaddr_in6
}

pub struct SocketAddressUnix {
  payload: sockaddr_un
}

pub trait ToNative {
  fn to_native(&self) -> (*const sockaddr, uint);
}

impl ToNative for SocketAddress {
  fn to_native(&self) -> (*const sockaddr, uint) {
    match *self {
      Ipv4(ref x) => x.to_native(),
      Ipv6(ref x) => x.to_native(),
      Unix(ref x) => x.to_native(),
    }
  }
}

impl ToNative for SocketAddressIpv4 {
  fn to_native(&self) -> (*const sockaddr, uint) {
    (&(self.payload) as *const sockaddr_in as *const sockaddr, mem::size_of::<sockaddr_in>())
  }
}

impl ToNative for SocketAddressIpv6 {
  fn to_native(&self) -> (*const sockaddr, uint) {
    (&(self.payload) as *const sockaddr_in6 as *const sockaddr, mem::size_of::<sockaddr_in6>())
  }
}

impl ToNative for SocketAddressUnix {
  fn to_native(&self) -> (*const sockaddr, uint) {
    (&(self.payload) as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>())
  }
}

/*
impl FromNative<sockaddr_in> for SocketAddressIpv4 {
  fn from_native(&self) -> sockaddr_in {
    let mut res: sockaddr_in = unsafe{ mem::zeroed() };
    res.sin_family = AF_INET as SocketAddressFamily;
    res.sin_port = self.port.to_be();
    res.sin_addr = in_addr{ s_addr: self.address };
    res
  }
}

impl FromNative<sockaddr_in6> for SocketAddressIpv6 {
  fn from_native(&self) -> sockaddr_in6 {
    let mut res: sockaddr_in6 = unsafe{ mem::zeroed() };
    res.sin6_family   = AF_INET6 as SocketAddressFamily; 
    res.sin6_port     = self.port.to_be(); 
    res.sin6_flowinfo = self.flowinfo.to_be();
    res.sin6_addr     = in6_addr{
      s6_addr: [
        self.address[0].to_be(),
        self.address[1].to_be(),
        self.address[2].to_be(),
        self.address[3].to_be(),
        self.address[4].to_be(),
        self.address[5].to_be(),
        self.address[6].to_be(),
        self.address[7].to_be(),
      ]
    };
    res.sin6_scope_id = self.scope_id.to_be();
    res
  }
}

impl FromNative<sockaddr_un> for SocketAddressUnix {
  fn from_native(&self) -> sockaddr_un {
    let mut res: sockaddr_un = unsafe{ mem::zeroed() };
    assert!(false, "unfinished business");
    res.sun_path = self.path;
    res
  }
}
*/
