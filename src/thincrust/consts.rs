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

//Protocol families.
typedef!(ProtocolFamily, c_int)

pub const PF_UNSPEC    : ProtocolFamily = ProtocolFamily{ get: consts::PF_UNSPEC };      //Unspecified.
pub const PF_LOCAL     : ProtocolFamily = ProtocolFamily{ get: consts::PF_LOCAL };       //Local to host (pipes and file-domain).
pub const PF_UNIX      : ProtocolFamily = ProtocolFamily{ get: consts::PF_UNIX };        // POSIX name for PF_LOCAL.
pub const PF_FILE      : ProtocolFamily = ProtocolFamily{ get: consts::PF_FILE };        // Another non-standard name for PF_LOCAL.
pub const PF_INET      : ProtocolFamily = ProtocolFamily{ get: consts::PF_INET };        // IP protocol family.
pub const PF_AX25      : ProtocolFamily = ProtocolFamily{ get: consts::PF_AX25 };        // Amateur Radio AX.25.
pub const PF_IPX       : ProtocolFamily = ProtocolFamily{ get: consts::PF_IPX };         // Novell Internet Protocol.
pub const PF_APPLETALK : ProtocolFamily = ProtocolFamily{ get: consts::PF_APPLETALK };   // Appletalk DDP.
pub const PF_NETROM    : ProtocolFamily = ProtocolFamily{ get: consts::PF_NETROM };      // Amateur radio NetROM.
pub const PF_BRIDGE    : ProtocolFamily = ProtocolFamily{ get: consts::PF_BRIDGE };      // Multiprotocol bridge.
pub const PF_ATMPVC    : ProtocolFamily = ProtocolFamily{ get: consts::PF_ATMPVC };      // ATM PVCs.
pub const PF_X25       : ProtocolFamily = ProtocolFamily{ get: consts::PF_X25 };         // Reserved for X.25 project.
pub const PF_INET6     : ProtocolFamily = ProtocolFamily{ get: consts::PF_INET6 };       // IP version 6.
pub const PF_ROSE      : ProtocolFamily = ProtocolFamily{ get: consts::PF_ROSE };        // Amateur Radio X.25 PLP.
pub const PF_DECnet    : ProtocolFamily = ProtocolFamily{ get: consts::PF_DECnet };      // Reserved for DECnet project.
pub const PF_NETBEUI   : ProtocolFamily = ProtocolFamily{ get: consts::PF_NETBEUI };     // Reserved for 802.2LLC project.
pub const PF_SECURITY  : ProtocolFamily = ProtocolFamily{ get: consts::PF_SECURITY };    // Security callback pseudo AF.
pub const PF_KEY       : ProtocolFamily = ProtocolFamily{ get: consts::PF_KEY };         // PF_KEY key management API.
pub const PF_NETLINK   : ProtocolFamily = ProtocolFamily{ get: consts::PF_NETLINK };
pub const PF_ROUTE     : ProtocolFamily = ProtocolFamily{ get: consts::PF_ROUTE };       // Alias to emulate 4.4BSD.
pub const PF_PACKET    : ProtocolFamily = ProtocolFamily{ get: consts::PF_PACKET };      // Packet family.
pub const PF_ASH       : ProtocolFamily = ProtocolFamily{ get: consts::PF_ASH };         // Ash.
pub const PF_ECONET    : ProtocolFamily = ProtocolFamily{ get: consts::PF_ECONET };      // Acorn Econet.
pub const PF_ATMSVC    : ProtocolFamily = ProtocolFamily{ get: consts::PF_ATMSVC };      // ATM SVCs.
pub const PF_RDS       : ProtocolFamily = ProtocolFamily{ get: consts::PF_RDS };         // RDS sockets.
pub const PF_SNA       : ProtocolFamily = ProtocolFamily{ get: consts::PF_SNA };         // Linux SNA Project
pub const PF_IRDA      : ProtocolFamily = ProtocolFamily{ get: consts::PF_IRDA };        // IRDA sockets.
pub const PF_PPPOX     : ProtocolFamily = ProtocolFamily{ get: consts::PF_PPPOX };       // PPPoX sockets.
pub const PF_WANPIPE   : ProtocolFamily = ProtocolFamily{ get: consts::PF_WANPIPE };     // Wanpipe API sockets.
pub const PF_LLC       : ProtocolFamily = ProtocolFamily{ get: consts::PF_LLC };         // Linux LLC.
pub const PF_CAN       : ProtocolFamily = ProtocolFamily{ get: consts::PF_CAN };         // Controller Area Network.
pub const PF_TIPC      : ProtocolFamily = ProtocolFamily{ get: consts::PF_TIPC };        // TIPC sockets.
pub const PF_BLUETOOTH : ProtocolFamily = ProtocolFamily{ get: consts::PF_BLUETOOTH };   // Bluetooth sockets.
pub const PF_IUCV      : ProtocolFamily = ProtocolFamily{ get: consts::PF_IUCV };        // IUCV sockets.
pub const PF_RXRPC     : ProtocolFamily = ProtocolFamily{ get: consts::PF_RXRPC };       // RxRPC sockets.
pub const PF_ISDN      : ProtocolFamily = ProtocolFamily{ get: consts::PF_ISDN };        // mISDN sockets.
pub const PF_PHONET    : ProtocolFamily = ProtocolFamily{ get: consts::PF_PHONET };      // Phonet sockets.
pub const PF_IEEE802154: ProtocolFamily = ProtocolFamily{ get: consts::PF_IEEE802154 };  // IEEE 802.15.4 sockets.
pub const PF_CAIF      : ProtocolFamily = ProtocolFamily{ get: consts::PF_CAIF };        // CAIF sockets.
pub const PF_ALG       : ProtocolFamily = ProtocolFamily{ get: consts::PF_ALG };         // Algorithm sockets.
pub const PF_NFC       : ProtocolFamily = ProtocolFamily{ get: consts::PF_NFC };         // NFC sockets.
pub const PF_VSOCK     : ProtocolFamily = ProtocolFamily{ get: consts::PF_VSOCK };       // vSockets.
pub const PF_MAX       : ProtocolFamily = ProtocolFamily{ get: consts::PF_MAX };         // For now..


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


