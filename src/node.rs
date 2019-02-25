use crate::{err::*, ty::*};

impl Node {
  pub fn new(name: Atom, serial_number: u32) -> Result<Self> {
    const SERIAL_NUMBER_MAX: u32 = (1 << 2) - 1;

    if serial_number <= SERIAL_NUMBER_MAX {
      Ok(Node {
        name,
        serial_number: serial_number as u8,
      })
    } else {
      Err(ErrorKind::NodeSerialNumberOutOfRange(name, serial_number).into())
    }
  }
}
