//! Port of the external term format.
//!
//! Why not use the `ei_decode_*` family of functions? These functions take a pointer to a buffer
//! and index into it, without bound checking. To provide a safe interface to these functions means
//! to perform bound checking before passing the buffer to `ei`, but you need to understand the
//! format to do it. This module takes the approach of decoding it directly and panicking on an out
//! of bound access rather than risking an out of bound access in case of an incorrect
//! implementation.

use crate::{err::*, read, ty::*};
use ei_sys;
use std::str;

#[derive(Debug, Copy, Clone)]
enum CreationFormat {
  Old,
  New,
}

impl CreationFormat {
  fn read(self, input: &[u8]) -> read::IResult<u32> {
    match self {
      CreationFormat::Old => read::be_u8::<u32>(input),
      CreationFormat::New => read::be_u32(input),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum AtomSizeFormat {
  Small,
  Regular,
}

impl AtomSizeFormat {
  pub fn read(self, input: &[u8]) -> read::IResult<usize> {
    match self {
      AtomSizeFormat::Small => read::be_u8::<usize>(input),
      AtomSizeFormat::Regular => read::be_u16::<usize>(input),
    }
  }
}

#[derive(Debug, Copy, Clone)]
enum TupleSizeFormat {
  Small,
  Large,
}

impl TupleSizeFormat {
  fn read(self, input: &[u8]) -> read::IResult<usize> {
    match self {
      TupleSizeFormat::Small => read::be_u8::<usize>(input),
      TupleSizeFormat::Large => {
        read::be_u32::<u32>(input).map(|(input, size)| (input, size as usize))
      }
    }
  }
}

pub fn term<'input>(input: &'input [u8], atom_cache: &AtomCache) -> read::IResult<'input, Term> {
  let (input, tag) = read::be_u8(input)?;
  match tag {
    ei_sys::ATOM_CACHE_REF => unimplemented!("ATOM_CACHE_REF"),
    ei_sys::SMALL_INTEGER_EXT => small_integer(input),
    ei_sys::INTEGER_EXT => integer(input),
    ei_sys::SMALL_BIG_EXT => small_big_integer(input),
    ei_sys::LARGE_BIG_EXT => large_big_integer(input),
    ei_sys::REFERENCE_EXT => reference(input, atom_cache),
    ei_sys::FLOAT_EXT => unimplemented!("FLOAT_EXT"),
    ei_sys::NEW_FLOAT_EXT => new_float(input),
    ei_sys::ATOM_UTF8_EXT => atom_utf8(input, AtomSizeFormat::Regular),
    ei_sys::SMALL_ATOM_UTF8_EXT => atom_utf8(input, AtomSizeFormat::Small),
    ei_sys::PID_EXT => pid(input, CreationFormat::Old, atom_cache),
    ei_sys::NEW_PID_EXT => pid(input, CreationFormat::New, atom_cache),
    ei_sys::SMALL_TUPLE_EXT => tuple(input, TupleSizeFormat::Small, atom_cache),
    ei_sys::LARGE_TUPLE_EXT => tuple(input, TupleSizeFormat::Large, atom_cache),
    _ => Err(ErrorKind::UnknownTermTag(tag).into()),
  }
}

fn small_integer(input: &[u8]) -> read::IResult<Term> {
  let (input, value) = read::be_u8::<i32>(input)?;
  Ok((input, Term::Integer(value)))
}

fn integer(input: &[u8]) -> read::IResult<Term> {
  let (input, value) = read::be_i32(input)?;
  Ok((input, Term::Integer(value)))
}

fn small_big_integer(_input: &[u8]) -> read::IResult<Term> {
  unimplemented!("SMALL_BIG_EXT")
}

fn large_big_integer(_input: &[u8]) -> read::IResult<Term> {
  unimplemented!("LARGE_BIG_EXT")
}

fn new_float(input: &[u8]) -> read::IResult<Term> {
  let (input, value) = read::be_f64(input)?;
  Ok((&input[31..], Term::Float(value)))
}

fn atom_utf8(input: &[u8], size_format: AtomSizeFormat) -> read::IResult<Term> {
  let (input, size) = size_format.read(input)?;
  let (input, atom_bytes) = read::take(input, size)?;
  Ok((input, Atom::new(str::from_utf8(atom_bytes)?)?.into()))
}

fn pid<'input>(
  input: &'input [u8],
  creation_format: CreationFormat,
  atom_cache: &AtomCache,
) -> read::IResult<'input, Term> {
  let (input, node_name) = read_node_name(input, atom_cache)?;
  let (input, id) = read::be_u32(input)?;
  let (input, serial) = read::be_u32(input)?;
  let (input, node_serial_number) = creation_format.read(input)?;
  let node = Node::new(node_name, node_serial_number)?;
  let pid = Pid::new(node, id, serial)?;
  Ok((input, pid.into()))
}

fn reference<'input>(input: &'input [u8], atom_cache: &AtomCache) -> read::IResult<'input, Term> {
  let (input, node_name) = read_node_name(input, atom_cache)?;
  let (input, id) = read::be_u32(input)?;
  let (input, serial_number) = read::be_u8::<u32>(input)?;
  let node = Node::new(node_name, serial_number)?;
  Ok((input, Reference { node, id }.into()))
}

fn read_node_name<'input>(
  input: &'input [u8],
  atom_cache: &AtomCache,
) -> read::IResult<'input, Atom> {
  match term(input, atom_cache)? {
    (input, Term::Atom(node_name)) => Ok((input, node_name)),
    (_, term) => Err(ErrorKind::NodeIsNotAnAtom(term.kind()).into()),
  }
}

fn tuple<'input>(
  input: &'input [u8],
  size_format: TupleSizeFormat,
  atom_cache: &AtomCache,
) -> read::IResult<'input, Term> {
  let (input, size) = size_format.read(input)?;
  let mut elements = Vec::with_capacity(size);
  let input = (0..size).try_fold(input, |input, _| -> Result<_> {
    let (input, element) = term(input, atom_cache)?;
    elements.push(element);
    Ok(input)
  })?;

  Ok((input, Tuple(elements.into_boxed_slice()).into()))
}
