use crate::ty::*;
use error_chain::*;
use std::{io, os, str};

error_chain! {
  foreign_links {
    Io(io::Error);
    Utf8(str::Utf8Error);
  }

  errors {
    ErlangError(errno: os::raw::c_int) {
      description("Erlang error"),
      display("errno: {}", errno),
    }

    Domain {
      description("domain error"),
    }

    UnsupportedProtocolVersion(version: u8) {
      description("unsupported protocol version"),
      display(
        "found protocol version {}, but only {} is supported",
        version,
        ei_sys::VERSION_MAGIC,
      ),
    }

    UnknownTermTag(tag: u8) {
      description("unknown term type"),
      display("unknown term tag '{}'", tag),
    }

    NodeIsNotAnAtom(term_kind: TermKind) {
      description("expected ")
    }

    TruncatedTerm {
      description(""),
    }

    UnknownMessageType(raw_type: os::raw::c_long) {
    }

    NameLengthOutOfRange(name: Box<[u8]>, name_kind: NameKind) {
    }

    NameHasEmbeddedNullByte(name: Box<[u8]>, name_kind: NameKind, position: usize) {
    }

    AtomLengthOutOfRange(text: Box<str>, text_len: usize) {
      description("an atom is invalid because it is larger than 255 Unicode code points"),
      display(
        "an atom is invalid because it has a length of {}, which is larger than 255 Unicode code \
         points: {}",
         text_len,
         &*text,
      ),
    }

    RunawayAtom(atom: Box<[os::raw::c_char]>) {
      description("an atom is invalid because it is not terminated by a null byte"),
      display("an atom is invalid because it is not terminated by a null byte: {:?}", atom),
    }

    NodeSerialNumberOutOfRange(name: Atom, serial_number: u32) {
      description("a node has a serial number that is out of range"),
      display("the node {} has a serial number that is out of range: {}", name, serial_number),
    }

    PidOutOfRange(node: Node, id: u32, serial: u32) {
      description("a PID is out of range"),
      display("a PID from node {} is out of range: {:x}+{:x}", node.name, id, serial),
    }
  }
}
