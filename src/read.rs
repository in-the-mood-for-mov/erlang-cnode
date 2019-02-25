use crate::err;

pub type IResult<'input, T> = err::Result<(&'input [u8], T)>;

pub fn take(input: &[u8], len: usize) -> IResult<&[u8]> {
  if input.len() >= len {
    let (result, input) = input.split_at(len);
    Ok((input, result))
  } else {
    Err(err::ErrorKind::TruncatedTerm.into())
  }
}

pub fn be_u8<T: From<u8>>(input: &[u8]) -> IResult<T> {
  if let Some((head, rest)) = input.split_first() {
    Ok((rest, T::from(*head)))
  } else {
    Err(err::ErrorKind::TruncatedTerm.into())
  }
}

pub fn be_u16<T: From<u16>>(input: &[u8]) -> IResult<T> {
  if input.len() < 2 {
    return Err(err::ErrorKind::TruncatedTerm.into());
  }

  Ok((
    &input[2..],
    T::from(((input[0] as u16) << 8) + (input[1] as u16)),
  ))
}

pub fn be_u32<T: From<u32>>(input: &[u8]) -> IResult<T> {
  if input.len() < 4 {
    return Err(err::ErrorKind::TruncatedTerm.into());
  }

  Ok((
    &input[4..],
    T::from(
      ((input[0] as u32) << 24)
        + ((input[1] as u32) << 16)
        + ((input[2] as u32) << 8)
        + (input[3] as u32),
    ),
  ))
}

pub fn be_i32(input: &[u8]) -> IResult<i32> {
  let (input, value) = be_u32::<u32>(input)?;
  Ok((input, value as i32))
}

pub fn be_u64(input: &[u8]) -> IResult<u64> {
  if input.len() < 8 {
    return Err(err::ErrorKind::TruncatedTerm.into());
  }

  Ok((
    &input[8..],
    ((input[0] as u64) << 56)
      + ((input[1] as u64) << 48)
      + ((input[2] as u64) << 40)
      + ((input[3] as u64) << 32)
      + ((input[4] as u64) << 24)
      + ((input[5] as u64) << 16)
      + ((input[6] as u64) << 8)
      + (input[7] as u64),
  ))
}

pub fn be_f64(input: &[u8]) -> IResult<f64> {
  let (input, value) = be_u64(input)?;
  Ok((input, value as f64))
}
