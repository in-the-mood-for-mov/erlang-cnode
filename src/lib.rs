#![recursion_limit = "1024"]

pub use crate::{
  c_node::{CNode, Connection, Listener},
  err::{Error, ErrorKind, Result, ResultExt},
  name::NodeName,
  ty::{Atom, ControlMessage, Message, Pid, Reference, Term, Tuple},
};

mod atom;
mod c;
mod c_node;
mod err;
mod ext;
mod message;
mod name;
mod node;
mod pid;
mod protocol;
mod read;
mod term;
mod ty;
mod x;
