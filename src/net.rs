

extern crate libc;
extern crate nix;

use std::io::{Error, ErrorKind};
use std::{mem, net, ptr};
use std::ffi::CStr;

use std::net::IpAddr;

#[allow(dead_code, non_camel_case_types)]
#[repr(C)]
pub enum SIOCGIFFLAGS {
    IFF_UP = 0x1,           /* Interface is up.  */
    IFF_BROADCAST = 0x2,    /* Broadcast address valid.  */
    IFF_DEBUG = 0x4,        /* Turn on debugging.  */
    IFF_LOOPBACK = 0x8,     /* Is a loopback net.  */
    IFF_POINTOPOINT = 0x10, /* Interface is point-to-point link.  */
    IFF_NOTRAILERS = 0x20,  /* Avoid use of trailers.  */
    IFF_RUNNING = 0x40,     /* Resources allocated.  */
    IFF_NOARP = 0x80,       /* No address resolution protocol.  */
    IFF_PROMISC = 0x100,    /* Receive all packets.  */

    /* Not supported */
    IFF_ALLMULTI = 0x200,   /* Receive all multicast packets.  */

    IFF_MASTER = 0x400,     /* Master of a load balancer.  */
    IFF_SLAVE = 0x800,      /* Slave of a load balancer.  */

    IFF_MULTICAST = 0x1000, /* Supports multicast.  */

    IFF_PORTSEL = 0x2000,   /* Can set media type.  */
    IFF_AUTOMEDIA = 0x4000, /* Auto media select active.  */
    IFF_DYNAMIC = 0x8000    /* Dialup device with changing addresses.  */
}

#[repr(C)]
pub struct union_ifa_ifu {
    pub data: *mut libc::c_void,
}
impl union_ifa_ifu {
    pub fn ifu_broadaddr (&mut self) -> *mut nix::sys::socket::sockaddr {
        self.data as *mut nix::sys::socket::sockaddr
    }
    pub fn ifu_dstaddr (&mut self) -> *mut nix::sys::socket::sockaddr {
        self.data as *mut nix::sys::socket::sockaddr
    }
}

#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut libc::c_char,
    pub ifa_flags: libc::c_uint,
    pub ifa_addr: *mut nix::sys::socket::sockaddr,
    pub ifa_netmask: *mut nix::sys::socket::sockaddr,
    pub ifa_ifu: union_ifa_ifu,
    pub ifa_data: *mut libc::c_void,
}


extern "C" {
    pub fn getifaddrs (ifap: *mut *mut ifaddrs) -> libc::c_int;
    pub fn freeifaddrs (ifa: *mut ifaddrs) -> libc::c_void;
}


pub fn nix_socketaddr_to_sockaddr (sa: *mut nix::sys::socket::sockaddr) -> Option<net::SocketAddr> {
    if sa.is_null() {
        return None;
    }

    let (addr, port) = match unsafe { *sa }.sa_family as i32 {
        nix::sys::socket::AF_INET => {
            let sa: *const nix::sys::socket::sockaddr_in = unsafe { mem::transmute(sa) };
            let sa = & unsafe { *sa };
            let (addr, port) = (sa.sin_addr.s_addr, sa.sin_port);
            (
                IpAddr::V4(net::Ipv4Addr::new(
                    ((addr & 0x000000FF) >>  0) as u8,
                    ((addr & 0x0000FF00) >>  8) as u8,
                    ((addr & 0x00FF0000) >> 16) as u8,
                    ((addr & 0xFF000000) >> 24) as u8,
                    )),
                port
            )
        },
        nix::sys::socket::AF_INET6 => {
            let sa: *const nix::sys::socket::sockaddr_in6 = unsafe { mem::transmute(sa) };
            let sa = & unsafe { *sa };
            let (addr, port) = (sa.sin6_addr.s6_addr, sa.sin6_port);
            (
                IpAddr::V6(net::Ipv6Addr::new(
                    addr[0] as u16,
                    addr[1] as u16,
                    addr[2] as u16,
                    addr[3] as u16,
                    addr[4] as u16,
                    addr[5] as u16,
                    addr[6] as u16,
                    addr[7] as u16,
                    )),
                port
            )
        },
        _ => return None,
    };
    Some(net::SocketAddr::new(addr, port))
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NextHop {
    Broadcast(net::SocketAddr),
    Destination(net::SocketAddr),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Kind {
    Packet,
    Ipv4,
    Ipv6,
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub kind: Kind,
    pub addr: Option<net::SocketAddr>,
    pub mask: Option<net::SocketAddr>,
    pub hop: Option<NextHop>,
}

impl Interface {
    pub fn interfaces () -> Result<Vec<Interface>, Error> {
        let mut ifaddrs_ptr: *mut ifaddrs = ptr::null_mut();
        match unsafe { getifaddrs(&mut ifaddrs_ptr as *mut _) } {
            0 => {
                let mut ret = Vec::new();
                let mut item: *mut ifaddrs = ifaddrs_ptr;
                loop {
                    if item.is_null() {
                        break;
                    }
                    let name = String::from_utf8(unsafe { CStr::from_ptr((*item).ifa_name) }.to_bytes().to_vec());
                    unsafe {
                        if name.is_err() || (*item).ifa_addr.is_null() {
                            break;
                        }
                    }
                    let kind = match unsafe { (*(*item).ifa_addr).sa_family as i32 } {
                        // pub const AF_PACKET: ::c_int = 17;
                        // socket::AF_PACKET
                        17i32 | 18i32 => Some(Kind::Packet),
                        nix::sys::socket::AF_INET => Some(Kind::Ipv4),
                        nix::sys::socket::AF_INET6 => Some(Kind::Ipv6),
                        _  => None,
                    };

                    if kind.is_none() {
                        break;
                    }

                    let addr = nix_socketaddr_to_sockaddr( unsafe { (*item).ifa_addr });
                    let mask = nix_socketaddr_to_sockaddr(unsafe { (*item).ifa_netmask} );
                    let hop =  unsafe { 
                        if (*item).ifa_flags & SIOCGIFFLAGS::IFF_BROADCAST as libc::c_uint == SIOCGIFFLAGS::IFF_BROADCAST as libc::c_uint {
                        match nix_socketaddr_to_sockaddr((*item).ifa_ifu.ifu_broadaddr()) {
                            Some(x) => Some(NextHop::Broadcast(x)),
                            None => None,
                        }
                    } else {
                        match nix_socketaddr_to_sockaddr((*item).ifa_ifu.ifu_dstaddr()) {
                            Some(x) => Some(NextHop::Destination(x)),
                            None => None,
                        }
                    }};
                    ret.push(Interface {
                        name: name.unwrap(),
                        kind: kind.unwrap(),
                        addr: addr,
                        mask: mask,
                        hop: hop,
                    });
                    item = unsafe { (*item).ifa_next };
                }
                unsafe { freeifaddrs(ifaddrs_ptr) };
                Ok(ret)
            },
            _ => Err(Error::new(ErrorKind::Other, "Oh, no ...")) // Err(nix::errno::Errno::last());
        }
    }
}

fn main (){
    let interfaces = Interface::interfaces().unwrap();
    for interface in interfaces.into_iter() {
        println!("{:?}", interface);
    }
    
}