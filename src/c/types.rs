///C types translated into rust.

//Basic primitive c types are imported directly from the already provided interface
//of the standard rust library, since the compiler is in better position to provide
//the size of a c_int for example. Most of the other types are defined here
//to decouple them from the putative changes of the experimental standard library.
extern crate libc;

pub use self::libc::{c_char, c_int, socklen_t, sa_family_t};
pub use self::libc::types::os::common::bsd44::{in_port_t, in_addr_t};

///Protocol entry extracted from the protocol database (/etc/protocols on Linux for example)
///by related functions, like getprotobyname().
pub struct protoent {
  pub p_name:     *const c_char,        //Official protocol name
  pub p_aliases:  *const *const c_char, //Alias list
  pub p_proto:    c_int,                //Protocol number
}


pub struct in_addr {
  pub s_addr: in_addr_t,
}

pub struct sockaddr_in {
  pub sin_family:   sa_family_t,
  pub sin_port:     in_port_t,
  pub sin_addr:     in_addr,
  pub sin_zero:     [u8, ..8],
}

pub struct in6_addr {
  pub s6_addr: [u16, ..8],
}

pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: u32,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: u32,
}

pub struct sockaddr {
  pub sa_family:  sa_family_t,
  pub sa_data:    [u8, ..14],
}

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
