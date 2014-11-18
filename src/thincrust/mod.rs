//TODO: define a uniform error message/number mechanism. Maybe look into what rust std api provides, like IOError or something.
use c::types::c_int;
use c::funcs::{socket, bind};

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
  pub fn bind(&mut self, sockaddr: SocketAddress) -> Result<(), String> {
   assert!(false, "unimplemented yet");
   Ok(())
  }
}


