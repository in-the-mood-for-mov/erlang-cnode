use crate::{ext, read, ty::*};

impl ControlMessage {
  pub fn read_message<'input>(
    self,
    input: &'input [u8],
    atom_cache: &AtomCache,
  ) -> read::IResult<'input, Message> {
    match self {
      ControlMessage::Send {
        from,
        to,
        trace_token,
      } => ext::read_term(input, atom_cache).map(|(input, term)| {
        (
          input,
          Message::Send {
            from,
            to,
            trace_token,
            term,
          },
        )
      }),
      ControlMessage::RegisteredSend {
        from,
        to,
        trace_token,
      } => ext::read_term(input, atom_cache).map(|(input, term)| {
        (
          input,
          Message::RegisteredSend {
            from,
            to,
            trace_token,
            term,
          },
        )
      }),
      ControlMessage::Link { .. } => unimplemented!(),
      ControlMessage::Unlink { .. } => unimplemented!(),
      ControlMessage::Exit { .. } => unimplemented!(),
    }
  }
}
