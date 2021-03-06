///C types translated into rust.

//Basic primitive c types are imported directly from the already provided interface
//of the standard rust library, since the compiler is in better position to provide
//the actual size of a c_int for example. Most of the other types are defined here
//to decouple them from the putative changes of the experimental standard library.
extern crate libc;

pub use self::libc::{c_void, c_char, c_uchar, c_int, c_short, c_ushort, socklen_t, sa_family_t, size_t, ssize_t};

///Protocol entry extracted from the protocol database (/etc/protocols on Linux for example)
///by related functions, like getprotobyname().
#[repr(C)]
pub struct protoent {
  pub p_name:     *const c_char,        //Official protocol name
  pub p_aliases:  *const *const c_char, //Alias list
  pub p_proto:    c_int,                //Protocol number
}


/**
  Dev note: 
  The address family of the socket address is host byte order.
  The rest is in network byte order.
*/

#[repr(C)]
pub struct in_addr {
  pub s_addr: [u8, ..4],  //Address in network byte order. However, since the network byte order doesn't really matter, it has been decided that its type should be a 4 byte array instead of a 32 bit word.
}

//Ipv4
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family:   sa_family_t,
  pub sin_port:     u16,
  pub sin_addr:     in_addr,
  pub sin_zero:     [u8, ..8],
}

#[repr(C)]
pub struct in6_addr {
  pub s6_addr: [u16, ..8],
}

//Ipv6
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: u16,
  pub sin6_flowinfo: u32,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: u32,
}

//Unix
#[repr(C)]
pub struct sockaddr_un {
  pub sun_family: sa_family_t,
  pub sun_path: [u8, ..108]
}

//Packet level
#[repr(C)]
pub struct sockaddr_ll {
  pub sll_family:   c_ushort,
  pub sll_protocol: c_ushort,
  pub sll_ifindex:  c_int,
  pub sll_hatype:   c_ushort,
  pub sll_pkttype:  c_uchar,
  pub sll_halen:    c_uchar,
  pub sll_addr:     [c_uchar, ..8]
}

//Socket storage for when receiving a socket address (as when calling accept()).
#[repr(C)]
pub struct sockaddr_storage {
  pub ss_family: sa_family_t,
  pub __ss_align: i64,
  pub __ss_pad2: [u8, ..112],
}

#[repr(C)]
pub struct sockaddr {
  pub sa_family:  sa_family_t,
  pub sa_data:    [u8, ..14],
}

#[repr(C)]
pub struct addrinfo {
  pub ai_flags:     c_int,
  pub ai_family:    c_int,
  pub ai_socktype:  c_int,
  pub ai_protocol:  c_int,
  pub ai_addrlen:   socklen_t,
  pub ai_addr:      *mut sockaddr,
  pub ai_canonname: *mut c_char,
  pub ai_next:      *mut addrinfo,
}
