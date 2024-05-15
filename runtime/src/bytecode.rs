use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Bytecode {
    val: Vec<u8>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self { val: vec![] }
    }

    pub fn write_u8(&mut self, b: u8) -> &mut Self {
        self.val.push(b);
        self
    }

    pub fn write_u16(&mut self, v: u16) -> &mut Self {
        for b in v.to_le_bytes() {
            self.write_u8(b);
        }
        self
    }

    pub fn write_u32(&mut self, v: u32) -> &mut Self {
        for b in v.to_le_bytes() {
            self.write_u8(b);
        }
        self
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn is_empty(&self) -> bool {
        self.val.is_empty()
    }

    pub fn read_u8(&self, i: usize) -> u8 {
        self.val[i]
    }

    pub fn read_u16(&self, i: usize) -> u16 {
        let s = self.val.as_slice();
        let s: [u8; 2] = [s[i], s[i + 1]];
        u16::from_le_bytes(s)
    }

    pub fn read_u32(&self, i: usize) -> u32 {
        let s = self.val.as_slice();
        let s: [u8; 4] = [s[i], s[i + 1], s[i + 2], s[i + 3]];
        u32::from_le_bytes(s)
    }
}
