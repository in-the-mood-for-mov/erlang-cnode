use erlang_cnode as erl;
use std::net;

fn main() -> erlang_cnode::Result<()> {
  let name = erl::NodeName::new("backend", "localhost")?;
  let node = erl::CNode::new(&name, net::Ipv4Addr::LOCALHOST, "hello")?;
  let mut listener = node.publish(42332)?;
  let mut connection = listener.accept()?;

  let stop = erl::Atom::new("stop")?;
  loop {
    let message = connection.receive()?;
    println!("{:?}", message);
    if let erl::Message::Send {
      term: erl::Term::Atom(atom),
      ..
    } = message
    {
      if atom == stop {
        break;
      }
    }
  }

  Ok(())
}
