#![cfg(target_os = "windows")]
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// IP Helper Reference ( Network Interface && routing table ）
//      https://msdn.microsoft.com/en-us/library/windows/desktop/aa366072(v=vs.85).aspx

// Network Driver Interface Specification
//      https://en.wikipedia.org/wiki/Network_Driver_Interface_Specification
//      https://docs.microsoft.com/en-us/windows-hardware/drivers/network/ndis-drivers
 
use libc;
use winapi;

pub use libc::{
    time_t, time64_t, clock_t, stat, utimbuf, tm, timeval, timespec, 
    EXIT_FAILURE, EXIT_SUCCESS, RAND_MAX, EOF, BUFSIZ, FOPEN_MAX, FILENAME_MAX, 
};

pub use winapi::ctypes::{
    c_void, c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar,
    c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, wchar_t,
};

pub use winapi::shared::minwindef::{
    // types
    ATOM, BOOL, BYTE, DWORD, FARPROC, FLOAT, GLOBALHANDLE,
    HFILE, HGLOBAL, HINSTANCE, HKEY, HKL, HLOCAL, HLSURF, 
    HMETAFILE, HMODULE, HRGN, HRSRC, HSPRITE, HSTR, HTASK, 
    HWINSTA, INT, LOCALHANDLE, LPARAM, LPBOOL, LPBYTE, LPCVOID, 
    LPDWORD, LPFILETIME, LPHANDLE, LPINT, LPLONG, LPVOID, LPWORD, 
    LRESULT, NEARPROC, PBOOL, PBYTE, PDWORD, PFILETIME, PFLOAT, 
    PHKEY, PINT, PROC, PSZ, PUCHAR, PUINT, PULONG, PUSHORT, 
    PWORD, SPHANDLE, UCHAR, UINT, ULONG, USHORT, WORD, WPARAM,
    // functions
    HIBYTE, HIWORD, LOBYTE, LOWORD, MAKELONG, MAKEWORD,
    // Constants
    FALSE, MAX_PATH, TRUE,
    // Structs
    FILETIME,
};

pub use winapi::shared::basetsd::{
    DWORD32, DWORD64, DWORD_PTR, HALF_PTR, HANDLE_PTR, INT8, INT16, 
    INT32, INT64, INT_PTR, KAFFINITY, LONG32, LONG64, LONG_PTR, 
    PDWORD32, PDWORD64, PDWORD_PTR, PHALF_PTR, PINT8, PINT16, PINT32, 
    PINT64, PINT_PTR, PKAFFINITY, PLONG32, PLONG64, PLONG_PTR, 
    POINTER_64_INT, PSIZE_T, PSSIZE_T, PUHALF_PTR, PUINT8, PUINT16, 
    PUINT32, PUINT64, PUINT_PTR, PULONG32, PULONG64, PULONG_PTR, 
    SHANDLE_PTR, SIZE_T, SSIZE_T, UHALF_PTR, UINT8, UINT16, UINT32, 
    UINT64, UINT_PTR, ULONG32, ULONG64, ULONG_PTR,
};

pub use winapi::shared::inaddr::{
    in_addr, in_addr_S_un, in_addr_S_un_b, in_addr_S_un_w,
    IN_ADDR, LPIN_ADDR, PIN_ADDR,
};

pub use winapi::shared::in6addr::{
    in6_addr, in6_addr_u,
    IN6_ADDR, LPIN6_ADDR, PIN6_ADDR,
};

pub use winapi::shared::ntdef::{
    CSTRING, FLOAT128, GROUP_AFFINITY, LARGE_INTEGER, LARGE_INTEGER_s,
    LIST_ENTRY, LIST_ENTRY32, LIST_ENTRY64, LUID, OBJECTID, OBJECT_ATTRIBUTES,
    OBJECT_ATTRIBUTES32, OBJECT_ATTRIBUTES64, PROCESSOR_NUMBER, QUAD,
    RTL_BALANCED_NODE, RTL_BALANCED_NODE_s, RTL_BALANCED_NODE_u, SINGLE_LIST_ENTRY,
    SINGLE_LIST_ENTRY32, STRING, STRING32, STRING64, ULARGE_INTEGER, ULARGE_INTEGER_s,
    UNICODE_STRING, WNF_STATE_NAME,

    PVOID, PVOID64, PWCHAR, PCHAR,
};

pub use winapi::shared::ws2def::{
    LPSOCKADDR, SOCKET_ADDRESS, PSOCKET_ADDRESS, SOCKADDR, ADDRESS_FAMILY,
    SOCKADDR_IN,
    
    AF_12844, AF_APPLETALK, AF_ATM, AF_BAN, AF_BTH, AF_CCITT, AF_CHAOS, AF_CLUSTER,
    AF_DATAKIT, AF_DECnet, AF_DLI, AF_ECMA, AF_FIREFOX, AF_HYLINK, AF_HYPERV,
    AF_ICLFXBM, AF_IMPLINK, AF_INET, AF_INET6, AF_IPX, AF_IRDA, AF_ISO, AF_LAT,
    AF_LINK, AF_MAX, AF_NETBIOS, AF_NETDES, AF_NS, AF_OSI, AF_PUP, AF_SNA,
    AF_TCNMESSAGE, AF_TCNPROCESS, AF_UNIX, AF_UNKNOWN1, AF_UNSPEC, AF_VOICEVIEW, 

    AI_ADDRCONFIG, AI_ALL, AI_CANONNAME, AI_DISABLE_IDN_ENCODING, AI_DNS_ONLY,
    AI_EXTENDED, AI_FILESERVER, AI_FQDN, AI_NON_AUTHORITATIVE, AI_NUMERICHOST,
    AI_NUMERICSERV, AI_PASSIVE, AI_RESOLUTION_HANDLE, AI_RETURN_PREFERRED_NAMES,
    AI_SECURE, AI_V4MAPPED, 

    INADDR_ANY, INADDR_BROADCAST, INADDR_LOOPBACK, INADDR_NONE, 
    
    IOCPARM_MASK, 
    
    IOC_IN, IOC_INOUT, IOC_OUT, IOC_PROTOCOL, IOC_UNIX, IOC_VENDOR, IOC_VOID, IOC_WS2, IOC_WSK, 

    IPPORT_BIFFUDP, IPPORT_CHARGEN, IPPORT_CMDSERVER, IPPORT_DAYTIME, IPPORT_DISCARD,
    IPPORT_DYNAMIC_MAX, IPPORT_DYNAMIC_MIN, IPPORT_ECHO, IPPORT_EFSSERVER, IPPORT_EPMAP,
    IPPORT_EXECSERVER, IPPORT_FINGER, IPPORT_FTP, IPPORT_FTP_DATA, IPPORT_HTTPS, IPPORT_IMAP,
    IPPORT_IMAP3, IPPORT_LDAP, IPPORT_LOGINSERVER, IPPORT_MICROSOFT_DS, IPPORT_MSP,
    IPPORT_MTP, IPPORT_NAMESERVER, IPPORT_NETBIOS_DGM, IPPORT_NETBIOS_NS, IPPORT_NETBIOS_SSN,
    IPPORT_NETSTAT, IPPORT_NTP, IPPORT_POP3, IPPORT_QOTD, IPPORT_REGISTERED_MAX, IPPORT_REGISTERED_MIN,
    IPPORT_RESERVED, IPPORT_RJE, IPPORT_ROUTESERVER, IPPORT_SMTP, IPPORT_SNMP, IPPORT_SNMP_TRAP,
    IPPORT_SUPDUP, IPPORT_SYSTAT, IPPORT_TCPMUX, IPPORT_TELNET, IPPORT_TFTP, IPPORT_TIMESERVER,
    IPPORT_TTYLINK, IPPORT_WHOIS, IPPORT_WHOSERVER, 

    IPPROTO_AH, IPPROTO_CBT, IPPROTO_DSTOPTS, IPPROTO_EGP, IPPROTO_ESP, IPPROTO_FRAGMENT,
    IPPROTO_GGP, IPPROTO_HOPOPTS, IPPROTO_ICLFXBM, IPPROTO_ICMP, IPPROTO_ICMPV6, IPPROTO_IDP,
    IPPROTO_IGMP, IPPROTO_IGP, IPPROTO_IP, IPPROTO_IPV4, IPPROTO_IPV6, IPPROTO_L2TP, IPPROTO_MAX,
    IPPROTO_ND, IPPROTO_NONE, IPPROTO_PGM, IPPROTO_PIM, IPPROTO_PUP, IPPROTO_RAW, IPPROTO_RDP,
    IPPROTO_RESERVED_IPSEC, IPPROTO_RESERVED_IPSECOFFLOAD, IPPROTO_RESERVED_MAX, IPPROTO_RESERVED_RAW,
    IPPROTO_RESERVED_WNV, IPPROTO_ROUTING, IPPROTO_SCTP, IPPROTO_ST, IPPROTO_TCP, IPPROTO_UDP, 

    MSG_BCAST, MSG_CTRUNC, MSG_MCAST, MSG_TRUNC,

    NI_DGRAM, NI_MAXHOST, NI_MAXSERV, NI_NAMEREQD, NI_NOFQDN, NI_NUMERICHOST, NI_NUMERICSERV, 
    
    NS_ALL, NS_BTH, NS_DHCP, NS_DNS, NS_EMAIL, NS_MS, NS_NBP, NS_NDS, NS_NETBT, NS_NETDES,
    NS_NIS, NS_NISPLUS, NS_NLA, NS_NTDS, NS_PEER_BROWSE, NS_PNRPCLOUD, NS_PNRPNAME, NS_SAP,
    NS_SLP, NS_STDA, NS_TCPIP_HOSTS, NS_TCPIP_LOCAL, NS_WINS, NS_WRQ, NS_X500, 

    SIO_ADDRESS_LIST_CHANGE, SIO_ADDRESS_LIST_QUERY, SIO_ADDRESS_LIST_SORT, SIO_ASSOCIATE_HANDLE,
    SIO_ENABLE_CIRCULAR_QUEUEING, SIO_FIND_ROUTE, SIO_FLUSH, SIO_GET_BROADCAST_ADDRESS,
    SIO_GET_EXTENSION_FUNCTION_POINTER, SIO_GET_GROUP_QOS, SIO_GET_MULTIPLE_EXTENSION_FUNCTION_POINTER,
    SIO_GET_QOS, SIO_MULTICAST_SCOPE, SIO_MULTIPOINT_LOOPBACK, SIO_QUERY_RSS_PROCESSOR_INFO,
    SIO_QUERY_TARGET_PNP_HANDLE, SIO_RESERVED_1, SIO_RESERVED_2, SIO_ROUTING_INTERFACE_CHANGE,
    SIO_ROUTING_INTERFACE_QUERY, SIO_SET_GROUP_QOS, SIO_SET_QOS, SIO_TRANSLATE_HANDLE, 

    SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_STREAM,
    
    SOL_SOCKET,
    
    SO_ACCEPTCONN, SO_BROADCAST, SO_BSP_STATE, SO_COMPARTMENT_ID, SO_CONDITIONAL_ACCEPT,
    SO_DEBUG, SO_DONTLINGER, SO_DONTROUTE, SO_ERROR, SO_EXCLUSIVEADDRUSE, SO_GROUP_ID,
    SO_GROUP_PRIORITY, SO_KEEPALIVE, SO_LINGER, SO_MAX_MSG_SIZE, SO_OOBINLINE, SO_PAUSE_ACCEPT,
    SO_PORT_SCALABILITY, SO_RANDOMIZE_PORT, SO_RCVBUF, SO_RCVLOWAT, SO_RCVTIMEO, SO_REUSEADDR,
    SO_REUSE_MULTICASTPORT, SO_REUSE_UNICASTPORT, SO_SNDBUF, SO_SNDLOWAT, SO_SNDTIMEO, SO_TYPE,
    SO_USELOOPBACK, 

    ScopeLevelAdmin, ScopeLevelCount, ScopeLevelGlobal, ScopeLevelInterface, ScopeLevelLink,
    ScopeLevelOrganization, ScopeLevelSite, ScopeLevelSubnet, 

    TCP_NODELAY, WSK_SO_BASE,
};

pub use winapi::um::winsock2::{
    accept, bind, closesocket, connect, gethostbyaddr, gethostbyname, gethostname,
    getpeername, getprotobyname, getprotobynumber, getservbyname, getservbyport,
    getsockname, getsockopt, h_errno, htond, htonf, htonl, htonll, htons, inet_addr,
    inet_ntoa, ioctlsocket, listen, ntohd, ntohf, ntohl, ntohll, ntohs, recv, 
    recvfrom, select, send, sendto, setsockopt, shutdown, socket,
};

pub use winapi::shared::winerror::{
    NO_ERROR, 
};


// https://msdn.microsoft.com/en-us/library/windows/desktop/ms738568(v=vs.85).aspx
pub const IFF_UP: c_int = 0x00000001;
pub const IFF_BROADCAST: c_int = 0x00000002;
pub const IFF_LOOPBACK: c_int = 0x00000004;
pub const IFF_POINTTOPOINT: c_int = 0x00000008;
pub const IFF_POINTOPOINT: c_int = IFF_POINTTOPOINT;
pub const IFF_MULTICAST: c_int = 0x00000010;


mod iphlpapi;

pub use iphlpapi::*;



