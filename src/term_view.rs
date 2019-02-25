use crate::{err::*, ty::*};

impl<'term> TermViewBuffer<'term> {
  pub fn new() -> Self {
    TermViewBuffer {
      atoms: Default::default(),
    }
  }

  pub fn view(&self, term: &'term Term) -> TermView<'term> {
    match term {
      Term::Nil => TermView::Nil,
      Term::Integer(value) => TermView::Integer(*value),
      Term::Float(value) => TermView::Float(*value),
      Term::Atom(atom) => TermView::Atom(atom.name()),
      Term::Pid(_) => unimplemented!(),
      Term::Reference(_) => unimplemented!(),
      Term::Tuple(_) => unimplemented!(),
      Term::List(_) => unimplemented!(),
      Term::Binary(_) => unimplemented!(),
    }
  }
}
