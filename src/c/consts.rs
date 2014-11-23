extern crate libc;

use self::libc::c_int;

//Taken from netdb.h
//Possible values for `ai_flags' field in `addrinfo' structure.
pub const AI_PASSIVE    : c_int = 0x0001; //Socket address is intended for `bind'.
pub const AI_CANONNAME  : c_int = 0x0002; //Request for canonical name.
pub const AI_NUMERICHOST: c_int = 0x0004; //Don't use name resolution.
pub const AI_V4MAPPED   : c_int = 0x0008; //IPv4 mapped addresses are acceptable.
pub const AI_ALL        : c_int = 0x0010; //Return IPv4 mapped and IPv6 addresses.
pub const AI_ADDRCONFIG : c_int = 0x0020; //Use configuration of this host to choose returned address type.

//Taken from bits/socket_type.h
//Sockets type
pub const SOCK_STREAM   : c_int = 1;        //TCP
pub const SOCK_DGRAM    : c_int = 2;        //UDP
pub const SOCK_RAW      : c_int = 3;        //Raw
pub const SOCK_RDM      : c_int = 4;        //Reliably delivered
pub const SOCK_SEQPACKET: c_int = 5;        //Sequenced, reliable, connection-based, datagrams of fixed maximum length
pub const SOCK_DCCP     : c_int = 6;        //Datagram congestion protocol
pub const SOCK_PACKET   : c_int = 10;       //Linux dev level for getting packets.
pub const SOCK_CLOEXEC  : c_int = 0o_2000000; //Atomically set close-on-exec flag for the new descriptors
pub const SOCK_NONBLOCK : c_int = 0o_0004000; //Atomically mark descriptos as non-blockin

//Taken from bits/socket.h
//Protocol families.
pub const PF_UNSPEC    : c_int = 0; //Unspecified.
pub const PF_LOCAL     : c_int = 1; //Local to host (pipes and file-domain).
pub const PF_UNIX      : c_int = PF_LOCAL; // POSIX name for PF_LOCAL.
pub const PF_FILE      : c_int = PF_LOCAL; // Another non-standard name for PF_LOCAL.
pub const PF_INET      : c_int = 2; // IP protocol family.
pub const PF_AX25      : c_int = 3; // Amateur Radio AX.25.
pub const PF_IPX       : c_int = 4; // Novell Internet Protocol.
pub const PF_APPLETALK : c_int = 5; // Appletalk DDP.
pub const PF_NETROM    : c_int = 6; // Amateur radio NetROM.
pub const PF_BRIDGE    : c_int = 7; // Multiprotocol bridge.
pub const PF_ATMPVC    : c_int = 8; // ATM PVCs.
pub const PF_X25       : c_int = 9; // Reserved for X.25 project.
pub const PF_INET6     : c_int = 10;  // IP version 6.
pub const PF_ROSE      : c_int = 11;  // Amateur Radio X.25 PLP.
pub const PF_DECnet    : c_int = 12;  // Reserved for DECnet project.
pub const PF_NETBEUI   : c_int = 13;  // Reserved for 802.2LLC project.
pub const PF_SECURITY  : c_int = 14;  // Security callback pseudo AF.
pub const PF_KEY       : c_int = 15;  // PF_KEY key management API.
pub const PF_NETLINK   : c_int = 16;
pub const PF_ROUTE     : c_int = PF_NETLINK; // Alias to emulate 4.4BSD.
pub const PF_PACKET    : c_int = 17;  // Packet family.
pub const PF_ASH       : c_int = 18;  // Ash.
pub const PF_ECONET    : c_int = 19;  // Acorn Econet.
pub const PF_ATMSVC    : c_int = 20;  // ATM SVCs.
pub const PF_RDS       : c_int = 21;  // RDS sockets.
pub const PF_SNA       : c_int = 22;  // Linux SNA Project
pub const PF_IRDA      : c_int = 23;  // IRDA sockets.
pub const PF_PPPOX     : c_int = 24;  // PPPoX sockets.
pub const PF_WANPIPE   : c_int = 25;  // Wanpipe API sockets.
pub const PF_LLC       : c_int = 26;  // Linux LLC.
pub const PF_CAN       : c_int = 29;  // Controller Area Network.
pub const PF_TIPC      : c_int = 30;  // TIPC sockets.
pub const PF_BLUETOOTH : c_int = 31;  // Bluetooth sockets.
pub const PF_IUCV      : c_int = 32;  // IUCV sockets.
pub const PF_RXRPC     : c_int = 33;  // RxRPC sockets.
pub const PF_ISDN      : c_int = 34;  // mISDN sockets.
pub const PF_PHONET    : c_int = 35;  // Phonet sockets.
pub const PF_IEEE802154: c_int = 36;  // IEEE 802.15.4 sockets.
pub const PF_CAIF      : c_int = 37;  // CAIF sockets.
pub const PF_ALG       : c_int = 38;  // Algorithm sockets.
pub const PF_NFC       : c_int = 39;  // NFC sockets.
pub const PF_VSOCK     : c_int = 40;  // vSockets.
pub const PF_MAX       : c_int = 41;  // For now..

//Address families.
pub const AF_UNSPEC    : c_int =  PF_UNSPEC;
pub const AF_LOCAL     : c_int =  PF_LOCAL;
pub const AF_UNIX      : c_int =  PF_UNIX;
pub const AF_FILE      : c_int =  PF_FILE;
pub const AF_INET      : c_int =  PF_INET;
pub const AF_AX25      : c_int =  PF_AX25;
pub const AF_IPX       : c_int =  PF_IPX;
pub const AF_APPLETALK : c_int = PF_APPLETALK;
pub const AF_NETROM    : c_int =  PF_NETROM;
pub const AF_BRIDGE    : c_int =  PF_BRIDGE;
pub const AF_ATMPVC    : c_int =  PF_ATMPVC;
pub const AF_X25       : c_int =  PF_X25;
pub const AF_INET6     : c_int =  PF_INET6;
pub const AF_ROSE      : c_int =  PF_ROSE;
pub const AF_DECnet    : c_int =  PF_DECnet;
pub const AF_NETBEUI   : c_int =  PF_NETBEUI;
pub const AF_SECURITY  : c_int =  PF_SECURITY;
pub const AF_KEY       : c_int =  PF_KEY;
pub const AF_NETLINK   : c_int =  PF_NETLINK;
pub const AF_ROUTE     : c_int =  PF_ROUTE;
pub const AF_PACKET    : c_int =  PF_PACKET;
pub const AF_ASH       : c_int =  PF_ASH;
pub const AF_ECONET    : c_int =  PF_ECONET;
pub const AF_ATMSVC    : c_int =  PF_ATMSVC;
pub const AF_RDS       : c_int =  PF_RDS;
pub const AF_SNA       : c_int =  PF_SNA;
pub const AF_IRDA      : c_int =  PF_IRDA;
pub const AF_PPPOX     : c_int =  PF_PPPOX;
pub const AF_WANPIPE   : c_int =  PF_WANPIPE;
pub const AF_LLC       : c_int =  PF_LLC;
pub const AF_CAN       : c_int =  PF_CAN;
pub const AF_TIPC      : c_int =  PF_TIPC;
pub const AF_BLUETOOTH : c_int =  PF_BLUETOOTH;
pub const AF_IUCV      : c_int =  PF_IUCV;
pub const AF_RXRPC     : c_int =  PF_RXRPC;
pub const AF_ISDN      : c_int =  PF_ISDN;
pub const AF_PHONET    : c_int =  PF_PHONET;
pub const AF_IEEE802154: c_int =  PF_IEEE802154;
pub const AF_CAIF      : c_int =  PF_CAIF;
pub const AF_ALG       : c_int =  PF_ALG;
pub const AF_NFC       : c_int =  PF_NFC;
pub const AF_VSOCK     : c_int =  PF_VSOCK;
pub const AF_MAX       : c_int =  PF_MAX;

//Ip protocols are 8 bits.
pub const IPPROTO_IP      : c_int = 0;
pub const IPPROTO_ICMP    : c_int = 1;
pub const IPPROTO_IGMP    : c_int = 2;
pub const IPPROTO_IPIP    : c_int = 4;
pub const IPPROTO_TCP     : c_int = 6;
pub const IPPROTO_EGP     : c_int = 8;
pub const IPPROTO_PUP     : c_int = 12;
pub const IPPROTO_UDP     : c_int = 17;
pub const IPPROTO_IDP     : c_int = 22;
pub const IPPROTO_TP      : c_int = 29;
pub const IPPROTO_DCCP    : c_int = 33;
pub const IPPROTO_IPV6    : c_int = 41;
pub const IPPROTO_RSVP    : c_int = 46;
pub const IPPROTO_GRE     : c_int = 47;
pub const IPPROTO_ESP     : c_int = 50;
pub const IPPROTO_AH      : c_int = 51;
pub const IPPROTO_MTP     : c_int = 92;
pub const IPPROTO_BEETPH  : c_int = 94;
pub const IPPROTO_ENCAP   : c_int = 98;
pub const IPPROTO_PIM     : c_int = 103;
pub const IPPROTO_COMP    : c_int = 108;
pub const IPPROTO_SCTP    : c_int = 132;
pub const IPPROTO_UDPLITE : c_int = 136;
pub const IPPROTO_RAW     : c_int = 255;
pub const IPPROTO_MAX     : c_int = 256;


pub const MSG_OOB: c_int          = 0x01;           /* Process out-of-band data.  */
pub const MSG_PEEK: c_int         = 0x02;           /* Peek at incoming messages.  */
pub const MSG_DONTROUTE: c_int    = 0x04;           /* Don't use local routing.  */
/* DECnet uses a different name.  */
pub const MSG_TRYHARD: c_int      = MSG_DONTROUTE;
pub const MSG_CTRUNC: c_int       = 0x08;           /* Control data lost before delivery.  */
pub const MSG_PROXY: c_int        = 0x10;           /* Supply or ask second address.  */
pub const MSG_TRUNC: c_int        = 0x20;
pub const MSG_DONTWAIT: c_int     = 0x40;           /* Nonblocking IO.  */
pub const MSG_EOR: c_int          = 0x80;           /* End of record.  */
pub const MSG_WAITALL: c_int      = 0x100;          /* Wait for a full request.  */
pub const MSG_FIN: c_int          = 0x200;
pub const MSG_SYN: c_int          = 0x400;
pub const MSG_CONFIRM: c_int      = 0x800;          /* Confirm path validity.  */
pub const MSG_RST: c_int          = 0x1000;
pub const MSG_ERRQUEUE: c_int     = 0x2000;         /* Fetch message from error queue.  */
pub const MSG_NOSIGNAL: c_int     = 0x4000;         /* Do not generate SIGPIPE.  */
pub const MSG_MORE: c_int         = 0x8000;         /* Sender will send more.  */
pub const MSG_WAITFORONE: c_int   = 0x10000;        /* Wait for at least one packet to return.*/
pub const MSG_FASTOPEN: c_int     = 0x20000000;     /* Send data in TCP SYN.  */
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;     /* Set close_on_exit for file descriptor received through SCM_RIGHTS.  */
