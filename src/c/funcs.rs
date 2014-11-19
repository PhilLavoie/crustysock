//For now, only linux is supported. Support for os specific functions like WSAGetLastError
//will be considered later.
use c::types::*;

#[link(name = "c")]
extern {
  //Socket related functions.
  pub fn socket(domain: c_int, sock_type: c_int, protocol: c_int) -> c_int;

  pub fn close(sockfd: c_int) -> c_int;

  //Apparently, some implementations might modify a field. sockaddr_un for example, can see its path
  //appended with a null character if not present. However, it's relatively safe to assume the structure
  //will remain constant.
  pub fn bind(sockfd: c_int, my_addr: *const sockaddr, addrlen: socklen_t) -> c_int;

  pub fn listen(sockfd: c_int, backlog: c_int) -> c_int;

  pub fn accept(sockfd: c_int, cliaddr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;

  pub fn connect(sockfd: c_int, serv_addr: *const sockaddr, addrlen: socklen_t) -> c_int;
  
  pub fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;

  //Addrinfo related functions.
  pub fn getaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *const addrinfo,
    res:   *mut *mut addrinfo
    ) -> c_int;

  pub fn freeaddrinfo(res: *mut addrinfo);

  pub fn gai_strerror(errcode: c_int) -> *const c_char;

  //Protocol entry functions.
  pub fn getprotoent() -> *const protoent;

  pub fn getprotobyname(name: *const c_char) -> *const protoent;

  pub fn getprotobynumber(proto: c_int) -> *const protoent;

  pub fn setprotoent(stayopen: c_int);

  pub fn endprotoent();

  //Consider using std::os::errno
  static errno: c_int;
}

