use ei_sys as ei;
use std::{mem, slice};

pub struct XBuffer {
  inner: ei::ei_x_buff,
}

impl XBuffer {
  /// Create a new `XBuffer`.
  ///
  /// # Panics
  ///
  /// This function panics if the memory allocation fails.
  pub fn new() -> XBuffer {
    unsafe {
      let mut term_buffer = XBuffer {
        inner: mem::uninitialized(),
      };
      let result = ei::ei_x_new(&mut term_buffer.inner);
      if result < 0 {
        panic!("failed to allocate memory");
      }
      term_buffer
    }
  }

  pub fn inner(&self) -> &ei::ei_x_buff {
    &self.inner
  }

  pub fn inner_mut(&mut self) -> &mut ei::ei_x_buff {
    &mut self.inner
  }

  pub fn as_slice(&self) -> &[u8] {
    let inner = self.inner();
    unsafe { slice::from_raw_parts(inner.buff as *const u8, inner.index as usize) }
  }
}

impl Drop for XBuffer {
  fn drop(&mut self) {
    unsafe {
      ei::ei_x_free(&mut self.inner);
    }
  }
}
