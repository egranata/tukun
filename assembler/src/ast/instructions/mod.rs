// this file is autogenerated, do not edit manually
// to change this file consult gen/genall.sh
mod fcall;
mod fromslot;
mod imp;
mod jtrue;
mod jump;
mod lpush;
mod push;
mod toslot;
use either::Either;
use runtime::intern_value::InternValue;
#[derive(Debug, Clone)]
pub enum Instruction {
    NOP,
    ADD,
    SUB,
    RET,
    FLOOKUP,
    TLOOKUP,
    CALL,
    NEWARR,
    NEWREC,
    EQ,
    GT,
    LT,
    NOT,
    OR,
    AND,
    DUP,
    SWAP,
    POP,
    ARRGET,
    ARRSET,
    ARRLEN,
    RECGET,
    RECSET,
    TYPEOF,
    I2B,
    I2F,
    B2I,
    F2I,
    MKARRTYPE,
    MKRECTYPE,
    PUSH(Either<u16, String>),
    LPUSH(InternValue),
    JUMP(String),
    JTRUE(String),
    FCALL(String),
    FROMSLOT(u16),
    TOSLOT(u16),
}
