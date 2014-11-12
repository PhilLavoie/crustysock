use c::types::{c_int};
use c::consts;

macro_rules! typedef(
  ($new_type:ident, $wrapped:ty) => (
    pub struct $new_type {
      get: $wrapped
    }

    impl $new_type {
      pub fn get(&self) -> $wrapped { self.get }
    }
   );
)

//Address families.
typedef!(AddressFamily, c_int)

pub const AF_UNSPEC    : AddressFamily = AddressFamily{ get: consts::AF_UNSPEC };
pub const AF_LOCAL     : AddressFamily = AddressFamily{ get: consts::AF_LOCAL };
pub const AF_UNIX      : AddressFamily = AddressFamily{ get: consts::AF_UNIX };
pub const AF_FILE      : AddressFamily = AddressFamily{ get: consts::AF_FILE };
pub const AF_INET      : AddressFamily = AddressFamily{ get: consts::AF_INET };
pub const AF_AX25      : AddressFamily = AddressFamily{ get: consts::AF_AX25 };
pub const AF_IPX       : AddressFamily = AddressFamily{ get: consts::AF_IPX };
pub const AF_APPLETALK : AddressFamily = AddressFamily{ get: consts::AF_APPLETALK };
pub const AF_NETROM    : AddressFamily = AddressFamily{ get: consts::AF_NETROM };
pub const AF_BRIDGE    : AddressFamily = AddressFamily{ get: consts::AF_BRIDGE };
pub const AF_ATMPVC    : AddressFamily = AddressFamily{ get: consts::AF_ATMPVC };
pub const AF_X25       : AddressFamily = AddressFamily{ get: consts::AF_X25 };
pub const AF_INET6     : AddressFamily = AddressFamily{ get: consts::AF_INET6 };
pub const AF_ROSE      : AddressFamily = AddressFamily{ get: consts::AF_ROSE };
pub const AF_DECnet    : AddressFamily = AddressFamily{ get: consts::AF_DECnet };
pub const AF_NETBEUI   : AddressFamily = AddressFamily{ get: consts::AF_NETBEUI };
pub const AF_SECURITY  : AddressFamily = AddressFamily{ get: consts::AF_SECURITY };
pub const AF_KEY       : AddressFamily = AddressFamily{ get: consts::AF_KEY };
pub const AF_NETLINK   : AddressFamily = AddressFamily{ get: consts::AF_NETLINK };
pub const AF_ROUTE     : AddressFamily = AddressFamily{ get: consts::AF_ROUTE };
pub const AF_PACKET    : AddressFamily = AddressFamily{ get: consts::AF_PACKET };
pub const AF_ASH       : AddressFamily = AddressFamily{ get: consts::AF_ASH };
pub const AF_ECONET    : AddressFamily = AddressFamily{ get: consts::AF_ECONET };
pub const AF_ATMSVC    : AddressFamily = AddressFamily{ get: consts::AF_ATMSVC };
pub const AF_RDS       : AddressFamily = AddressFamily{ get: consts::AF_RDS };
pub const AF_SNA       : AddressFamily = AddressFamily{ get: consts::AF_SNA };
pub const AF_IRDA      : AddressFamily = AddressFamily{ get: consts::AF_IRDA };
pub const AF_PPPOX     : AddressFamily = AddressFamily{ get: consts::AF_PPPOX };
pub const AF_WANPIPE   : AddressFamily = AddressFamily{ get: consts::AF_WANPIPE };
pub const AF_LLC       : AddressFamily = AddressFamily{ get: consts::AF_LLC };
pub const AF_CAN       : AddressFamily = AddressFamily{ get: consts::AF_CAN };
pub const AF_TIPC      : AddressFamily = AddressFamily{ get: consts::AF_TIPC };
pub const AF_BLUETOOTH : AddressFamily = AddressFamily{ get: consts::AF_BLUETOOTH };
pub const AF_IUCV      : AddressFamily = AddressFamily{ get: consts::AF_IUCV };
pub const AF_RXRPC     : AddressFamily = AddressFamily{ get: consts::AF_RXRPC };
pub const AF_ISDN      : AddressFamily = AddressFamily{ get: consts::AF_ISDN };
pub const AF_PHONET    : AddressFamily = AddressFamily{ get: consts::AF_PHONET };
pub const AF_IEEE802154: AddressFamily = AddressFamily{ get: consts::AF_IEEE802154 };
pub const AF_CAIF      : AddressFamily = AddressFamily{ get: consts::AF_CAIF };
pub const AF_ALG       : AddressFamily = AddressFamily{ get: consts::AF_ALG };
pub const AF_NFC       : AddressFamily = AddressFamily{ get: consts::AF_NFC };
pub const AF_VSOCK     : AddressFamily = AddressFamily{ get: consts::AF_VSOCK };
pub const AF_MAX       : AddressFamily = AddressFamily{ get: consts::AF_MAX };

//Socket types.
typedef!(SocketType, c_int)

pub const SOCK_STREAM   : SocketType = SocketType{ get: consts::SOCK_STREAM };    //TCP
pub const SOCK_DGRAM    : SocketType = SocketType{ get: consts::SOCK_DGRAM };     //UDP
pub const SOCK_RAW      : SocketType = SocketType{ get: consts::SOCK_RAW };       //Raw
pub const SOCK_RDM      : SocketType = SocketType{ get: consts::SOCK_RDM };       //Reliably delivered
pub const SOCK_SEQPACKET: SocketType = SocketType{ get: consts::SOCK_SEQPACKET }; //Sequenced, reliable, connection-based, datagrams of fixed maximum length
pub const SOCK_DCCP     : SocketType = SocketType{ get: consts::SOCK_DCCP };      //Datagram congestion protocol
pub const SOCK_PACKET   : SocketType = SocketType{ get: consts::SOCK_PACKET };    //Linux dev level for getting packets.
pub const SOCK_CLOEXEC  : SocketType = SocketType{ get: consts::SOCK_CLOEXEC };   //Atomically set close-on-exec flag for the new descriptors
pub const SOCK_NONBLOCK : SocketType = SocketType{ get: consts::SOCK_NONBLOCK };  //Atomically mark descriptos as non-blockin


