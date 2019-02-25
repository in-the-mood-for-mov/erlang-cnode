use erlang_cnode as erl;
use std::net;

fn main() -> erl::Result<()> {
  let name = erl::NodeName::new("backend", "localhost")?;
  let node = erl::CNode::new(&name, net::Ipv4Addr::LOCALHOST, "hello")?;
  let mut listener = node.publish(42332)?;
  let mut connection = listener.accept()?;

  loop {
    if let erl::Message::RegisteredSend { term, .. } = connection.receive()? {
      let buffer = erl::TermViewBuffer::new();
      match buffer.view(&term) {
        erl::atom!("stop") => break,
        view => { dbg!(view); }
      }
    }
  }

  Ok(())
}
