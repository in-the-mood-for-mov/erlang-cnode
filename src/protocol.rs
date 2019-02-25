use crate::{err::*, ext, read, ty::*};
use ei_sys as ei;
use std::str;

struct AtomCacheReferenceFlags {
  is_new_entry: bool,
  segment_index: AtomCacheSegment,
}

impl From<u8> for AtomCacheReferenceFlags {
  fn from(value: u8) -> Self {
    AtomCacheReferenceFlags {
      is_new_entry: value & 0x8 != 0,
      segment_index: match value & 0x7 {
        0 => AtomCacheSegment::S0,
        1 => AtomCacheSegment::S1,
        2 => AtomCacheSegment::S2,
        3 => AtomCacheSegment::S3,
        4 => AtomCacheSegment::S4,
        5 => AtomCacheSegment::S5,
        6 => AtomCacheSegment::S6,
        7 => AtomCacheSegment::S7,
        _ => unreachable!(),
      },
    }
  }
}

pub fn read_version_magic<'input>(input: &[u8]) -> read::IResult<()> {
  let (input, version) = read::be_u8::<u8>(input)?;
  if version == ei::VERSION_MAGIC {
    Ok((input, ()))
  } else {
    Err(ErrorKind::UnsupportedProtocolVersion(version).into())
  }
}

pub fn read_distribution_header<'input>(
  original_input: &'input [u8],
  atom_cache: &mut AtomCache,
) -> read::IResult<'input, ()> {
  let (input, tag) = read::be_u8::<u8>(original_input)?;
  if tag != ei::DIST_HEADER {
    return Ok((original_input, ()));
  }

  let (input, atom_reference_count) = read::be_u8::<usize>(input)?;
  if atom_reference_count == 0 {
    return Ok((input, ()));
  }

  let flag_bytes_counts: usize = atom_reference_count / 2 + 1;
  let (input, flags_bytes) = read::take(input, flag_bytes_counts)?;

  let flags = {
    let mut flags = Vec::<AtomCacheReferenceFlags>::with_capacity(flag_bytes_counts.into());
    for i in 0..atom_reference_count {
      flags.push(get_nth_half_byte(flags_bytes, i).into());
    }
    flags
  };

  let cache_flags = get_nth_half_byte(flags_bytes, atom_reference_count);
  let atom_size_format = if cache_flags & 0x1 != 0 {
    ext::AtomSizeFormat::Regular
  } else {
    ext::AtomSizeFormat::Small
  };

  let input = flags.iter().try_fold(input, |input, flag| -> Result<_> {
    let (input, internal_index) = read::be_u8::<u8>(input)?;
    let key = AtomCacheKey {
      segment_index: flag.segment_index,
      internal_index,
    };

    let input = if flag.is_new_entry {
      let (input, atom_byte_len) = atom_size_format.read(input)?;
      let (input, atom_bytes) = read::take(input, atom_byte_len)?;
      atom_cache.insert(key, Atom::new(str::from_utf8(atom_bytes)?)?);
      input
    } else {
      input
    };

    Ok(input)
  })?;

  Ok((input, ()))
}

fn get_nth_half_byte(input: &[u8], index: usize) -> u8 {
  let byte = input[index >> 1];
  if index & 0x01 == 0 {
    byte >> 4
  } else {
    byte & 0x0f
  }
}
