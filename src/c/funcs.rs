use c::types::*;

#[link(name = "c")]
extern {
  pub fn socket(domain: c_int, sock_type: c_int, protocol: c_int) -> c_int;

  pub fn close(sockfd: c_int) -> c_int;

  pub fn bind(sockfd: c_int, my_addr: *const sockaddr, addrlen: socklen_t) -> c_int;

  pub fn listen(sockfd: c_int, backlog: c_int) -> c_int;

  pub fn accept(sockfd: c_int, cliaddr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;

  pub fn connect(sockfd: c_int, serv_addr: *const sockaddr, addrlen: socklen_t) -> c_int;
  
  pub fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;

  pub fn getaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *const addrinfo,
    res:   *mut *mut addrinfo
    ) -> c_int;

  pub fn freeaddrinfo(res: *mut addrinfo);

  pub fn gai_strerror(errcode: c_int) -> *const c_char;
}
