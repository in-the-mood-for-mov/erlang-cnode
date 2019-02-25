use crate::{err::*, ty::*};

impl Pid {
  /// Creates a new `Id`.
  ///
  /// Even though the protocol reserves 4 bytes for it, only the 15 lower bits are significant.
  /// This function returns a new `Id` if the given integer in within the range of a 15 bits
  /// integer and an error otherwise.
  pub fn new(node: Node, id: u32, serial: u32) -> Result<Self> {
    const ID_MAX: u32 = (1 << 15) - 1;
    const SERIAL_MAX: u32 = (1 << 13) - 1;

    if id <= ID_MAX && serial <= SERIAL_MAX {
      Ok(Pid {
        node,
        id: id as u16,
        serial: serial as u16,
      })
    } else {
      Err(ErrorKind::PidOutOfRange(node, id, serial).into())
    }
  }

  pub fn id(self) -> u32 {
    self.id.into()
  }
}
