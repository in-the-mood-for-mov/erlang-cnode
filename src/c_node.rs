use crate::{c, err::*, name::NodeName, protocol, ty::*, x};
use ei_sys as ei;
use in_addr;
use std::{borrow, ffi, i32, mem, net, os};

pub struct CNode {
  c_node: ei_sys::ei_cnode,
  addr: net::Ipv4Addr,
}

impl CNode {
  pub fn new(node_name: &NodeName, host_addr: net::Ipv4Addr, cookie: &str) -> Result<Self> {
    unsafe {
      let mut node = CNode {
        c_node: mem::zeroed(),
        addr: host_addr,
      };
      let result = ei_sys::ei_connect_xinit(
        &mut node.c_node,
        node_name.host_name().as_ptr(),
        node_name.alive_name().as_ptr(),
        node_name.full_name().as_ptr(),
        &mut in_addr::InAddr::new(node.addr).into(),
        ffi::CString::new(cookie).unwrap().as_ptr(),
        0,
      );

      if result < 0 {
        return Err(c::last_error());
      }

      Ok(node)
    }
  }

  pub fn alive_name(&self) -> borrow::Cow<str> {
    let c_alive_name = unsafe {
      let ptr = ei::ei_thisalivename(&self.c_node);
      ffi::CStr::from_ptr(ptr)
    };
    c_alive_name.to_string_lossy()
  }

  pub fn host_name(&self) -> borrow::Cow<str> {
    let c_alive_name = unsafe {
      let ptr = ei::ei_thishostname(&self.c_node);
      ffi::CStr::from_ptr(ptr)
    };
    c_alive_name.to_string_lossy()
  }

  pub fn node_name(&self) -> borrow::Cow<str> {
    let c_alive_name = unsafe {
      let ptr = ei::ei_thisnodename(&self.c_node);
      ffi::CStr::from_ptr(ptr)
    };
    c_alive_name.to_string_lossy()
  }

  pub fn connect(
    &mut self,
    remote_alive_name: &str,
    remote_addr: net::Ipv4Addr,
  ) -> Result<Connection> {
    unsafe {
      let c_remote_alive_name = ffi::CString::new(remote_alive_name).unwrap();
      let result = ei::ei_xconnect_tmo(
        &mut self.c_node,
        &mut in_addr::InAddr::new(remote_addr).into(),
        c_remote_alive_name.as_ptr() as *mut _,
        2000,
      );
      if result < 0 {
        return Err(c::last_error());
      }

      Ok(Connection::new(socket_from_fd(result)))
    }
  }

  pub fn publish(mut self, port: u16) -> Result<Listener> {
    let tcp_listener = net::TcpListener::bind((self.addr, port))?;
    let result = unsafe { ei::ei_publish(&mut self.c_node, port as i32) };
    if result < 0 {
      return Err(c::last_error());
    }

    Ok(Listener::new(self, tcp_listener))
  }
}

pub struct Listener {
  node: CNode,
  listener: net::TcpListener,
}

impl Listener {
  fn new(node: CNode, listener: net::TcpListener) -> Listener {
    Listener { node, listener }
  }

  pub fn accept(&mut self) -> Result<Connection> {
    unsafe {
      let mut connection = mem::zeroed::<ei_sys::ErlConnect>();
      let result = ei::ei_accept(
        &mut self.node.c_node,
        socket_as_fd(&self.listener),
        &mut connection,
      );
      if result < 0 {
        return Err(c::last_error());
      }
      Ok(Connection::new(socket_from_fd(result)))
    }
  }
}

pub struct Connection {
  tcp_stream: net::TcpStream,
  atom_cache: AtomCache,
}

impl Connection {
  unsafe fn new(tcp_stream: net::TcpStream) -> Connection {
    Connection {
      tcp_stream,
      atom_cache: AtomCache::new(),
    }
  }

  pub fn receive(&mut self) -> Result<Message> {
    unsafe {
      let mut c_message = mem::uninitialized::<ei_sys::erlang_msg>();
      let mut buffer = x::XBuffer::new();
      loop {
        match ei_sys::ei_xreceive_msg(
          socket_as_fd(&self.tcp_stream),
          &mut c_message,
          buffer.inner_mut(),
        ) {
          ei_sys::ERL_ERROR => return Err(c::last_error()),
          ei_sys::ERL_TICK => (),
          ei_sys::ERL_MSG => {
            let (input, ()) =
              protocol::read_distribution_header(buffer.as_slice(), &mut self.atom_cache)?;
            let control_message = ControlMessage::from_c(&c_message)?;
            let (_, message) = control_message.read_message(input, &self.atom_cache)?;
            return Ok(message);
          }
          result => panic!("unknown result from ei_xreceive_msg: {}", result),
        }
      }
    }
  }
}

#[cfg(unix)]
fn socket_as_fd<T>(t: &T) -> os::raw::c_int
where
  T: os::unix::io::AsRawFd,
{
  t.as_raw_fd()
}

#[cfg(unix)]
unsafe fn socket_from_fd<T>(fd: os::raw::c_int) -> T
where
  T: os::unix::io::FromRawFd,
{
  T::from_raw_fd(fd)
}

#[cfg(windows)]
fn socket_as_fd<T>(t: &T) -> os::raw::c_int
where
  T: os::windows::io::AsRawSocket,
{
  const RAW_SOCKET_MAX: os::windows::io::RawSocket = os::raw::c_int::max_value() as _;

  let raw = t.as_raw_socket();
  assert!(raw < RAW_SOCKET_MAX);
  raw as os::raw::c_int
}

#[cfg(windows)]
unsafe fn socket_from_fd<T>(fd: os::raw::c_int) -> T
where
  T: os::windows::io::FromRawSocket,
{
  assert!(fd >= 0);
  T::from_raw_socket(fd as os::windows::io::RawSocket)
}
