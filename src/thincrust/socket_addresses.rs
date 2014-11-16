/**
  Socket address module. Wraps around the underlying C equivalent (sockaddr, sockaddr_in, sockaddr_in6, etc...).
*/

use thincrust::consts::*;
use c::types::{sockaddr, sa_family_t, sockaddr_in, sockaddr_in6};

type SocketAddressFamily = sa_family_t; //most likely u16
