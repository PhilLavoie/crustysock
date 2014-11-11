use c::consts::*;
use c::funcs::*;
use c::types::*;

use std::mem;
use std::c_str::CString;
use std::num::Int;

///Represents an ip address of version 4 or 6.
pub enum IpAddress {
  Ipv4(u8, u8, u8, u8),
  Ipv6(u16, u16, u16, u16, u16, u16, u16, u16),
}

impl IpAddress {
  ///Function used to convert the 32 bits of an ipv4 address in network byte order
  ///into the equivalent IpAddress.
  fn ipv4_from_net(ip: u32) -> IpAddress {
    let host_endian = ip;
    let byte_ptr = &host_endian as *const u32 as *const u8;
    Ipv4(
      unsafe { *byte_ptr },
      unsafe { *(byte_ptr.offset(1)) },
      unsafe { *(byte_ptr.offset(2)) },
      unsafe { *(byte_ptr.offset(3)) },
    )
  }

  ///Function used to convert the 128 bits of an ipv6 address in network byte order
  ///into the equivalent IpAddress.
  fn ipv6_from_net(byte_array: [u16, ..8]) -> IpAddress {
    Ipv6(
      Int::from_be(byte_array[0]),
      Int::from_be(byte_array[1]),
      Int::from_be(byte_array[2]),
      Int::from_be(byte_array[3]),
      Int::from_be(byte_array[4]),
      Int::from_be(byte_array[5]),
      Int::from_be(byte_array[6]),
      Int::from_be(byte_array[7]),
    )
  }
}

pub struct SocketAddress {
  pub ip_address: IpAddress,
  pub port:             u16,
}

pub struct AddressInfo {
  pub socket_address: SocketAddress
}

impl AddressInfo {
  fn new(ai: &addrinfo) -> AddressInfo {
    if (*ai).ai_family == AF_INET {
      //The ai_addr is expected to be of type sockaddr_in.
      let sap = unsafe { &*((*ai).ai_addr as *const sockaddr_in) };
      //The address is under sin_addr.s_addr field, and in network endianness (big).
      let ip = IpAddress::ipv4_from_net((*sap).sin_addr.s_addr);
      //Get and convert the port.
      let port = Int::from_be((*sap).sin_port);
      return AddressInfo {socket_address: SocketAddress{ip_address: ip, port: port}};
    //Ipv6
    } else {
      let sap6 = unsafe { &*((*ai).ai_addr as *const sockaddr_in6) };
      let ip = IpAddress::ipv6_from_net((*sap6).sin6_addr.s6_addr);
      let port = Int::from_be((*sap6).sin6_port);
      return AddressInfo {socket_address: SocketAddress{ip_address: ip, port: port}};
    }
  }
}

pub struct AddressInfoIterator {
  addr: *mut addrinfo,
}


impl AddressInfoIterator {
  fn new(ai: *mut addrinfo) -> AddressInfoIterator {
    AddressInfoIterator { addr: ai }
  }
}

///The address info iterator moves accross objects allocated by the
///sockets interface. It is therefore responsible for deallocating said
///objects. It is done through a destructor.
impl Drop for AddressInfoIterator {
  fn drop(&mut self) {
    unsafe{ freeaddrinfo(self.addr); }
  }
}

//The user decides if he copies or not :)
impl Iterator<AddressInfo> for AddressInfoIterator {
  fn next(&mut self) -> Option<AddressInfo> {
    if self.addr.is_null() { return None; }

    //Copy and store the data into 
    let sock = Some(AddressInfo::new(unsafe { &*self.addr }));    
    //Move to next addrinfo.
    self.addr = unsafe { (*self.addr).ai_next };
    return sock;
  }
}

///A convenient wrapper around getaddrinfo function.
pub fn get_host_addresses(host: &str, service: &str) -> Result<AddressInfoIterator, String> {
  let node = host.to_c_str();
  let service = service.to_c_str();
  let mut hints: addrinfo = unsafe { mem::zeroed() };
  let mut res: *mut addrinfo = unsafe { mem::zeroed() };

  //Initialize hints.
  hints.ai_family   = AF_UNSPEC;  //Any family: ipv6 or ipv6.
  hints.ai_socktype = 0;                  //0 in this field indicates any socket type.
  hints.ai_flags    = AI_PASSIVE; //Fills out the rest.

  unsafe {
    let errno = getaddrinfo(
      node.as_ptr(),
      service.as_ptr(),
      &hints as *const addrinfo,
      &mut res   as *mut *mut addrinfo
    );

    //Get the error message and return that.
    if errno != 0 { 
      let a = CString::new(gai_strerror(errno), true);
      return Err(a.as_str().unwrap().to_string());
    }

    return Ok(AddressInfoIterator::new(res));
  }
}

pub struct Socket {
  sockfd: c_int
}

impl Socket {
  pub fn new() -> Socket { assert!(false, "unimplemented yet"); Socket{ sockfd: -1 } }
}
