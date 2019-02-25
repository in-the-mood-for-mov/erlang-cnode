use crate::ty::*;

impl From<Atom> for Term {
  fn from(atom: Atom) -> Self {
    Term::Atom(atom)
  }
}

impl From<Pid> for Term {
  fn from(pid: Pid) -> Self {
    Term::Pid(pid)
  }
}

impl From<Reference> for Term {
  fn from(reference: Reference) -> Self {
    Term::Reference(reference)
  }
}

impl From<Tuple> for Term {
  fn from(tuple: Tuple) -> Self {
    Term::Tuple(tuple)
  }
}

impl Term {
  pub fn kind(&self) -> TermKind {
    match self {
      Term::Integer(_) => TermKind::Integer,
      Term::Float(_) => TermKind::Float,
      Term::Atom(_) => TermKind::Atom,
      Term::Pid { .. } => TermKind::Pid,
      Term::Reference { .. } => TermKind::Reference,
      Term::Tuple(_) => TermKind::Tuple,
    }
  }
}
