#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
    NOP,
    DUP,
    POP,
    PUSH,
    FLOOKUP,
    TLOOKUP,
    CALL,
    TYPEOF,
    FROMSLOT,
    TOSLOT,
    ADD,
    EQUAL,
    JUMP,
    JTRUE,
    RET,
    MKARRTYPE,
    MKRECTYPE,
    NEWARR,
    ARRGET,
    ARRSET,
    ARRLEN,
    MAX,
}
impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe {
            if value >= std::mem::transmute(Opcode::MAX) {
                panic!("invalid opcode {value}")
            } else {
                std::mem::transmute(value)
            }
        }
    }
}
impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
