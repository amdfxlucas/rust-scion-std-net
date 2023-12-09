use std::fmt::Write;

use std::fmt::{Result,Error};
use std::mem::MaybeUninit;
use std::str;



/// Used for slow path in `Display` implementations when alignment is required.
pub struct DisplayBuffer<const SIZE: usize> {
    buf: [MaybeUninit<u8>; SIZE],
    len: usize,
}


impl<const SIZE: usize> DisplayBuffer<SIZE> {
    
    #[inline]
    pub const fn new() -> Self {
        Self { buf: [MaybeUninit::uninit();SIZE], len: 0 }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        // SAFETY: `buf` is only written to by the `fmt::Write::write_str` implementation
        // which writes a valid UTF-8 string to `buf` and correctly sets `len`.
       /* unsafe {
            let s = MaybeUninit::slice_assume_init_ref(&self.buf[..self.len]);
            str::from_utf8_unchecked(s)
        }*/

        let initialized_array: [u8; SIZE] = unsafe {
            let mut result: [u8; SIZE] = MaybeUninit::uninit().assume_init();
            for i in 0..SIZE {
                result[i] = self.buf[i].assume_init();
            }
            result
        };
    
        // Use transmute to get a stable reference to the initialized slice
        let initialized_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(
                initialized_array.as_ptr(),
                SIZE,
            )
        };

        unsafe{
        str::from_utf8_unchecked(initialized_slice)
        }
    }
}

impl<const SIZE: usize> std::fmt::Write for DisplayBuffer<SIZE> {
   /* fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        if let Some(buf) = self.buf.get_mut(self.len..(self.len + bytes.len())) {
            MaybeUninit::write_slice(buf, bytes);
            self.len += bytes.len();
            Ok(())
        } else {
            Err(fmt::Error)
        }
    } */

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let bytes = s.as_bytes();

        if let Some(buf) = self.buf.get_mut(self.len..(self.len + bytes.len())) {
            for (dest, &src) in buf.iter_mut().zip(bytes.iter()) {
                unsafe {
                    // Write each byte manually
                    dest.write(src);
                }
            }
            self.len += bytes.len();
            Ok(())
        } else {
            Err(std::fmt::Error)
        }
    }
}