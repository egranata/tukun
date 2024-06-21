// this file is autogenerated, do not edit manually
// to change this file consult gen/genall.sh
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeInstruction {
    NOP,
    DUP,
    SWAP,
    POP,
    PUSH(u16),
    FLOOKUP,
    TLOOKUP,
    CALL,
    TYPEOF,
    FROMSLOT(u16),
    TOSLOT(u16),
    ADD,
    SUB,
    EQ,
    LT,
    GT,
    AND,
    OR,
    I2B,
    I2F,
    B2I,
    F2I,
    NOT,
    JUMP(u16),
    JTRUE(u16),
    RET,
    MKARRTYPE,
    MKRECTYPE,
    NEWARR,
    ARRGET,
    ARRSET,
    ARRLEN,
    NEWREC,
    RECGET,
    RECSET,
}
// this file is autogenerated, do not edit manually
// to change this file consult gen/genall.sh
impl RuntimeInstruction {
    pub fn from_bytecode(
        bc: &crate::bytecode::Bytecode,
        i: usize,
    ) -> Option<(RuntimeInstruction, usize)> {
        let mut idx = i;
        let b = crate::opcodes::Opcode::from(bc.read_u8(idx));
        idx += 1;
        match b {
            crate::opcodes::Opcode::NOP => Some((RuntimeInstruction::NOP, idx)),
            crate::opcodes::Opcode::DUP => Some((RuntimeInstruction::DUP, idx)),
            crate::opcodes::Opcode::SWAP => Some((RuntimeInstruction::SWAP, idx)),
            crate::opcodes::Opcode::POP => Some((RuntimeInstruction::POP, idx)),
            crate::opcodes::Opcode::PUSH => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                Some((RuntimeInstruction::PUSH(arg0), idx))
            }
            crate::opcodes::Opcode::FLOOKUP => Some((RuntimeInstruction::FLOOKUP, idx)),
            crate::opcodes::Opcode::TLOOKUP => Some((RuntimeInstruction::TLOOKUP, idx)),
            crate::opcodes::Opcode::CALL => Some((RuntimeInstruction::CALL, idx)),
            crate::opcodes::Opcode::TYPEOF => Some((RuntimeInstruction::TYPEOF, idx)),
            crate::opcodes::Opcode::FROMSLOT => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                Some((RuntimeInstruction::FROMSLOT(arg0), idx))
            }
            crate::opcodes::Opcode::TOSLOT => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                Some((RuntimeInstruction::TOSLOT(arg0), idx))
            }
            crate::opcodes::Opcode::ADD => Some((RuntimeInstruction::ADD, idx)),
            crate::opcodes::Opcode::SUB => Some((RuntimeInstruction::SUB, idx)),
            crate::opcodes::Opcode::EQ => Some((RuntimeInstruction::EQ, idx)),
            crate::opcodes::Opcode::LT => Some((RuntimeInstruction::LT, idx)),
            crate::opcodes::Opcode::GT => Some((RuntimeInstruction::GT, idx)),
            crate::opcodes::Opcode::AND => Some((RuntimeInstruction::AND, idx)),
            crate::opcodes::Opcode::OR => Some((RuntimeInstruction::OR, idx)),
            crate::opcodes::Opcode::I2B => Some((RuntimeInstruction::I2B, idx)),
            crate::opcodes::Opcode::I2F => Some((RuntimeInstruction::I2F, idx)),
            crate::opcodes::Opcode::B2I => Some((RuntimeInstruction::B2I, idx)),
            crate::opcodes::Opcode::F2I => Some((RuntimeInstruction::F2I, idx)),
            crate::opcodes::Opcode::NOT => Some((RuntimeInstruction::NOT, idx)),
            crate::opcodes::Opcode::JUMP => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                Some((RuntimeInstruction::JUMP(arg0), idx))
            }
            crate::opcodes::Opcode::JTRUE => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                Some((RuntimeInstruction::JTRUE(arg0), idx))
            }
            crate::opcodes::Opcode::RET => Some((RuntimeInstruction::RET, idx)),
            crate::opcodes::Opcode::MKARRTYPE => Some((RuntimeInstruction::MKARRTYPE, idx)),
            crate::opcodes::Opcode::MKRECTYPE => Some((RuntimeInstruction::MKRECTYPE, idx)),
            crate::opcodes::Opcode::NEWARR => Some((RuntimeInstruction::NEWARR, idx)),
            crate::opcodes::Opcode::ARRGET => Some((RuntimeInstruction::ARRGET, idx)),
            crate::opcodes::Opcode::ARRSET => Some((RuntimeInstruction::ARRSET, idx)),
            crate::opcodes::Opcode::ARRLEN => Some((RuntimeInstruction::ARRLEN, idx)),
            crate::opcodes::Opcode::NEWREC => Some((RuntimeInstruction::NEWREC, idx)),
            crate::opcodes::Opcode::RECGET => Some((RuntimeInstruction::RECGET, idx)),
            crate::opcodes::Opcode::RECSET => Some((RuntimeInstruction::RECSET, idx)),
            _ => None,
        }
    }
}
