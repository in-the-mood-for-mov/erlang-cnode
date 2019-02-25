use crate::{err::*, ty::*};
use ei_sys as ei;
use std::{borrow, collections, fmt, ops, result};

impl Atom {
  pub fn new<Text>(text: Text) -> Result<Self>
  where
    Text: Into<String>,
  {
    let text_string = text.into();
    let text_len = text_string.chars().count();
    let text_str = text_string.into_boxed_str();
    if text_len > ei::MAXATOMLEN {
      return Err(ErrorKind::AtomLengthOutOfRange(text_str, text_len).into());
    }
    Ok(Atom(text_str))
  }
}

impl borrow::Borrow<str> for Atom {
  fn borrow(&self) -> &str {
    let Atom(text) = self;
    borrow::Borrow::borrow(text)
  }
}

impl fmt::Display for Atom {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
    formatter.write_str("'")?;
    formatter.write_str(&*self.0)?;
    formatter.write_str("'")
  }
}

impl AtomCache {
  pub fn new() -> Self {
    AtomCache {
      entries: collections::HashMap::new(),
    }
  }

  pub fn insert(&mut self, key: AtomCacheKey, atom: Atom) -> Option<Atom> {
    self.entries.insert(key, atom)
  }
}

impl ops::Index<&AtomCacheKey> for AtomCache {
  type Output = Atom;

  fn index(&self, index: &AtomCacheKey) -> &Self::Output {
    self.entries.index(index)
  }
}
