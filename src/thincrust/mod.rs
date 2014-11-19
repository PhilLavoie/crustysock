//TODO: define a uniform error message/number mechanism. Maybe look into what rust std api provides, like IOError or something.
use std::mem;


use c::types::{c_int, socklen_t};
use c::funcs::{socket, bind, connect};

pub use thincrust::consts::*;
pub use thincrust::protocols::*;
pub use thincrust::socket_addresses::*;


mod consts;
mod protocols;
mod socket_addresses;
mod address_info;

pub struct Socket {
  sockfd: c_int
}

impl Socket {
  pub fn new<P: Proto>(
    domain:   ProtocolFamily,
    socktype: SocketType, 
    proto:    P
  ) -> Result<Socket, String> { 
    let maybe_sockfd = unsafe{ socket(domain.get(), socktype.get(), proto.protocol_number()) };
    
    //Error values seem to be os specific from first look around. Therefore, this function
    //does not return any meaningful error.
    if maybe_sockfd < 0 { return Err(format!("unable to create socket, error code: {}", maybe_sockfd)) }
    Ok(Socket{ sockfd: maybe_sockfd })
  }

  //Maybe return a new structure: Bound socket?
  //Only seems to be worth on stream sockets.
  pub fn bind(&mut self, socket_addr: SocketAddress) -> Result<(), String> {
    let (ptr, size_of) = socket_addr.to_native();
    let returned = unsafe{ bind(self.sockfd, ptr, size_of as socklen_t) };
    if returned < 0 { return Err(format!("unable to bind socket")); }
    Ok(())
  }


  //TODO: replace Result<(), String> with option String essé

  pub fn connect(&mut self, socket_addr: SocketAddress) -> Result<(), String> {
    let (ptr, size_of) = socket_addr.to_native();
    let returned = unsafe{ connect(self.sockfd, ptr, size_of as socklen_t) };
    if returned < 0 { return Err(format!("unable to connect socket")); }
    Ok(())
  }
}


