mod fcall;
mod fromslot;
mod imp;
mod jtrue;
mod jump;
mod push;
mod toslot;
use either::Either;
#[derive(Debug, Clone)]
pub enum Instruction {
    NOP,
    ADD,
    RET,
    FLOOKUP,
    TLOOKUP,
    CALL,
    NEWARR,
    EQUAL,
    DUP,
    POP,
    ARRGET,
    ARRSET,
    ARRLEN,
    TYPEOF,
    MKARRTYPE,
    MKRECTYPE,
    PUSH(Either<u16, String>),
    JUMP(String),
    JTRUE(String),
    FCALL(String),
    FROMSLOT(u16),
    TOSLOT(u16),
}
