/**
  Socket address module. Wraps around the underlying C equivalent (sockaddr, sockaddr_in, sockaddr_in6, etc...).
*/
use std::mem;
use std::num::Int;

use c::consts::*;
use c::types::{sockaddr, sa_family_t, sockaddr_in, in_addr, in6_addr, sockaddr_in6};

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
  port: PortInt,
  address: [u8, ..4]
}

pub struct SocketAddressIpv6 {
  port: PortInt,
  address: [u16, ..8],
  flowinfo: u32,
  scope_id: u32
}

pub struct SocketAddressUnix {
  path: [u8, ..108]
}

pub trait ToNative<T> {
  fn to_native(&self) -> T;
}

impl ToNative<sockaddr_in> for SocketAddressIpv4 {
  fn to_native(&self) -> sockaddr_in {
    let mut res: sockaddr_in = unsafe{ mem::zeroed() };
    res.sin_family = AF_INET as SocketAddressFamily;
    res.sin_port = self.port.to_be();
    res.sin_addr = in_addr{ s_addr: self.address };
    res
  }
}

impl ToNative<sockaddr_in6> for SocketAddressIpv6 {
  fn to_native(&self) -> sockaddr_in6 {
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


