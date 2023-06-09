use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Registers([u8; 0x0016]);

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "0: {:#04x} 1: {:#04x} 2: {:#04x} 3: {:#04x} 4: {:#04x} 5: {:#04x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl Default for Registers {
    fn default() -> Self {
        Registers([0x0; 0x0016])
    }
}

impl Index<usize> for Registers {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Registers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
