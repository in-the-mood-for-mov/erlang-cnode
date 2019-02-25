use crate::{err::*, ty::*};
use std::{ffi, fmt, result};

impl NameKind {
  fn validate_c_string(self, name_bytes: &[u8]) -> Result<ffi::CString> {
    if name_bytes.len() > self.max_len() {
      return Err(ErrorKind::NameLengthOutOfRange(name_bytes.into(), self).into());
    }

    match ffi::CString::new(name_bytes) {
      Ok(name) => Ok(name),
      Err(err) => {
        Err(ErrorKind::NameHasEmbeddedNullByte(name_bytes.into(), self, err.nul_position()).into())
      }
    }
  }

  fn max_len(self) -> usize {
    match self {
      NameKind::Node => ei_sys::MAXNODELEN,
      NameKind::Alive => ei_sys::EI_MAXALIVELEN,
      NameKind::Host => ei_sys::EI_MAXHOSTNAMELEN,
    }
  }
}

impl fmt::Display for NameKind {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
    fmt::Display::fmt(
      match self {
        NameKind::Node => "node name",
        NameKind::Alive => "alive name",
        NameKind::Host => "host name",
      },
      formatter,
    )
  }
}

pub struct NodeName {
  full_name: ffi::CString,
  alive_name: ffi::CString,
  host_name: ffi::CString,
}

impl NodeName {
  pub fn new<AliveName, HostName>(alive_name: AliveName, host_name: HostName) -> Result<Self>
  where
    AliveName: AsRef<[u8]>,
    HostName: AsRef<[u8]>,
  {
    let alive_name_bytes = alive_name.as_ref();
    let host_name_bytes = host_name.as_ref();

    let c_alive_name = NameKind::Alive.validate_c_string(alive_name_bytes)?;
    let c_host_name = NameKind::Host.validate_c_string(host_name_bytes)?;

    let mut full_name_bytes = Vec::<u8>::with_capacity(ei_sys::MAXNODELEN);
    full_name_bytes.extend_from_slice(alive_name_bytes);
    full_name_bytes.push(b'@');
    full_name_bytes.extend_from_slice(host_name_bytes);
    let c_full_name = ffi::CString::new(full_name_bytes).unwrap();

    Ok(NodeName {
      full_name: c_full_name,
      alive_name: c_alive_name,
      host_name: c_host_name,
    })
  }

  pub fn full_name(&self) -> &ffi::CStr {
    self.full_name.as_ref()
  }

  pub fn alive_name(&self) -> &ffi::CStr {
    self.alive_name.as_ref()
  }

  pub fn host_name(&self) -> &ffi::CStr {
    self.host_name.as_ref()
  }
}
