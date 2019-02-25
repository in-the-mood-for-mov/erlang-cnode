use crate::{err::*, ty::*};
use ei_sys as ei;
use std::{ffi, io, os, ptr};

pub fn last_error() -> Error {
  let code = unsafe { ptr::read_volatile(ei::__erl_errno_place()) };
  io::Error::from_raw_os_error(code).into()
}

type CAtom = [os::raw::c_char; ei::MAXATOMLEN_UTF8];

impl Atom {
  pub fn from_c(c_atom: &CAtom) -> Result<Self> {
    if c_atom.iter().all(|&c| c != 0) {
      return Err(ErrorKind::RunawayAtom(c_atom.to_vec().into_boxed_slice()).into());
    }

    let atom_text = unsafe { ffi::CStr::from_ptr(c_atom.as_ptr()) }.to_str()?;
    Atom::new(atom_text)
  }
}

impl Pid {
  pub fn from_c(c_pid: &ei::erlang_pid) -> Result<Self> {
    let node_name = Atom::from_c(&c_pid.node)?;
    let node = Node::new(node_name, c_pid.creation)?;
    Pid::new(node, c_pid.num, c_pid.serial)
  }
}

impl ControlMessage {
  pub fn from_c(message: &ei::erlang_msg) -> Result<Self> {
    const ERL_LINK: os::raw::c_long = ei::ERL_LINK as _;
    const ERL_SEND: os::raw::c_long = ei::ERL_SEND as _;
    const ERL_EXIT: os::raw::c_long = ei::ERL_EXIT as _;
    const ERL_UNLINK: os::raw::c_long = ei::ERL_UNLINK as _;
    const ERL_NODE_LINK: os::raw::c_long = ei::ERL_NODE_LINK as _;
    const ERL_REG_SEND: os::raw::c_long = ei::ERL_REG_SEND as _;
    const ERL_GROUP_LEADER: os::raw::c_long = ei::ERL_GROUP_LEADER as _;
    const ERL_EXIT2: os::raw::c_long = ei::ERL_EXIT2 as _;

    match message.msgtype {
      ERL_LINK => unimplemented!(),
      ERL_SEND => Ok(ControlMessage::Send {
        from: Pid::from_c(&message.from)?,
        to: Pid::from_c(&message.to)?,
        trace_token: None,
      }),
      ERL_EXIT => unimplemented!(),
      ERL_UNLINK => unimplemented!(),
      ERL_NODE_LINK => unimplemented!(),
      ERL_REG_SEND => Ok(ControlMessage::RegisteredSend {
        from: Pid::from_c(&message.from)?,
        to: Atom::from_c(&message.toname)?,
        trace_token: None,
      }),
      ERL_GROUP_LEADER => unimplemented!(),
      ERL_EXIT2 => unimplemented!(),
      message_type => Err(ErrorKind::UnknownMessageType(message_type).into()),
    }
  }
}
