/**
  Socket address module. Wraps around the underlying C equivalent (sockaddr, sockaddr_in, sockaddr_in6, etc...).
*/

use thincrust::consts::*;
use c::types::{sockaddr, sa_family_t, sockaddr_in, sockaddr_in6};

type SocketAddressFamily = sa_family_t; //most likely u16
type PortInt = u16;

//Eventually support all socket address type but also provide a custom one, for a fallback mechanism.
//It could hold a *void and a size_t.
enum SocketAddress {
  Ipv4{ port: PortInt, address: [u8, ..4] },
  Ipv6{ port: PortInt, address: [u16, ..8], flowinfo: u32, scope_id: u32 },
  Unix{ path: [u8, ..108] },
}


