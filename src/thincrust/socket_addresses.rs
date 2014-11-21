/**
  Socket address module. Wraps around the underlying C equivalent (sockaddr, sockaddr_in, sockaddr_in6, etc...).
*/

use std::mem;
use std::num::Int;

use c::consts::*;
use c::types::{c_int, socklen_t, sockaddr, sa_family_t, sockaddr_in, in_addr, in6_addr, sockaddr_in6, sockaddr_un, sockaddr_storage};

type SocketAddressFamily = sa_family_t; //most likely u16
type PortInt = u16;

pub struct SocketAddressStorage {
  payload: sockaddr_storage,
  size:    socklen_t
}

impl SocketAddressStorage {
  pub fn new() -> SocketAddressStorage {
    return SocketAddressStorage{ payload: unsafe{ mem::zeroed() }, size: mem::size_of::<sockaddr_storage>() as socklen_t }
  }

  pub fn to_native(&mut self) -> (*mut sockaddr, *mut socklen_t) {
    (&mut (self.payload) as *mut sockaddr_storage as *mut sockaddr, &mut (self.size) as *mut socklen_t)
  }

  pub fn to_socket_address(&self) -> ! {
    panic!("unimplemented");
  }
}

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

pub trait FromNative {
  fn from_native(sock: *const sockaddr, len: socklen_t) -> Result<Self, String>;
}

pub trait ToNative {
  fn to_native(&self) -> (*const sockaddr, socklen_t);
}

fn dispatch<F: FromNative>(sock: *const sockaddr, len: socklen_t, ctor: |F| -> SocketAddress) -> Result<SocketAddress, String> {
  let sockaddr_res = FromNative::from_native(sock, len);
  match sockaddr_res {
    Ok(res)   => Ok(ctor(res)),
    Err(msg)  => Err(msg)
  } 
}

impl FromNative for SocketAddress {
  fn from_native(sock: *const sockaddr, len: socklen_t) -> Result<SocketAddress, String> {
    let family = unsafe{ (*sock).sa_family as c_int };
    match family {
      AF_INET   => dispatch(sock, len, |res: SocketAddressIpv4| { SocketAddress::Ipv4(res) } ),
      AF_INET6  => dispatch(sock, len, |res: SocketAddressIpv6| { SocketAddress::Ipv6(res) } ),
      AF_UNIX   => dispatch(sock, len, |res: SocketAddressUnix| { SocketAddress::Unix(res) } ), 
      _ => Err(format!("encountered unsupported socket type: {} while converting from native type", family))
    }
  }
}

impl ToNative for SocketAddress {
  fn to_native(&self) -> (*const sockaddr, socklen_t) {
    match *self {
      SocketAddress::Ipv4(ref x) => x.to_native(),
      SocketAddress::Ipv6(ref x) => x.to_native(),
      SocketAddress::Unix(ref x) => x.to_native(),
    }
  }
}

impl FromNative for SocketAddressIpv4 {
  fn from_native(sock: *const sockaddr, len: socklen_t) -> Result<SocketAddressIpv4, String> {
    assert!(!sock.is_null());
    assert!(unsafe{ (*sock).sa_family } as c_int == AF_INET);
    let expected_len = mem::size_of::<sockaddr_in>() as socklen_t;
    if len != expected_len { return Err(format!("unexpected sockaddr_in length: {}, expected: {}", len, expected_len)); }
    
    Ok(
      SocketAddressIpv4{
        payload: unsafe{ *(sock as *const sockaddr_in) }
      }
    )
  }
}

impl ToNative for SocketAddressIpv4 {
  fn to_native(&self) -> (*const sockaddr, socklen_t) {
    (&(self.payload) as *const sockaddr_in as *const sockaddr, mem::size_of::<sockaddr_in>() as socklen_t)
  }
}

impl FromNative for SocketAddressIpv6 {
  fn from_native(sock: *const sockaddr, len: socklen_t) -> Result<SocketAddressIpv6, String> {
    assert!(!sock.is_null());
    assert!(unsafe{ (*sock).sa_family } as c_int == AF_INET6);
    let expected_len = mem::size_of::<sockaddr_in6>() as socklen_t;
    if len != expected_len { return Err(format!("unexpected sockaddr_in6 length: {}, expected: {}", len, expected_len)); }
    
    Ok(
      SocketAddressIpv6{
        payload: unsafe{ *(sock as *const sockaddr_in6) }
      }
    )
  }
}

impl ToNative for SocketAddressIpv6 {
  fn to_native(&self) -> (*const sockaddr, socklen_t) {
    (&(self.payload) as *const sockaddr_in6 as *const sockaddr, mem::size_of::<sockaddr_in6>() as socklen_t)
  }
}

//TODO: There are three types of UNIX sockets under linux, at least, and their identification
//depends on the socket length returned among other things.
impl FromNative for SocketAddressUnix {
  fn from_native(sock: *const sockaddr, len: socklen_t) -> Result<SocketAddressUnix, String> {
    assert!(!sock.is_null());
    assert!(unsafe{ (*sock).sa_family } as c_int == AF_INET);
    let expected_len = mem::size_of::<sockaddr_un>() as socklen_t;
    if len != expected_len { return Err(format!("unexpected sockaddr_un length: {}, expected: {}", len, expected_len)); }
    
    Ok(
      SocketAddressUnix{
        payload: unsafe{ *(sock as *const sockaddr_un) }
      }
    )
  }
}

impl ToNative for SocketAddressUnix {
  fn to_native(&self) -> (*const sockaddr, socklen_t) {
    (&(self.payload) as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>() as socklen_t)
  }
}

//TODO: Absolutely need to implement from_native tests, even dummy ones.
