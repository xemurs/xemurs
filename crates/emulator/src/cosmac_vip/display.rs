use std::ops::{Index, IndexMut};
use std::ptr;

pub const SCREEN_AREA: usize = SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize;
pub const SCREEN_HEIGHT: u32 = 32;
pub const SCREEN_WIDTH: u32 = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayBuffer(pub(crate) [u8; SCREEN_AREA]);

impl DisplayBuffer {
    pub fn reset(&mut self) {
        unsafe {
            let buff = self.0.as_mut_ptr();
            ptr::write_bytes(buff, 0, SCREEN_AREA);
        }
    }
}

impl Default for DisplayBuffer {
    fn default() -> Self {
        DisplayBuffer([0x0; SCREEN_AREA])
    }
}

impl Index<usize> for DisplayBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for DisplayBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
