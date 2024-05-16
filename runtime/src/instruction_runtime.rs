#[derive(Debug, Clone)]
pub enum RuntimeInstruction {
    NOP,
    DUP,
    POP,
    PUSH(u16),
    FLOOKUP,
    TLOOKUP,
    CALL,
    TYPEOF,
    FROMSLOT(u16),
    TOSLOT(u16),
    ADD,
    EQUAL,
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
}
impl RuntimeInstruction {
    pub fn from_bytecode(bc: &crate::bytecode::Bytecode, i: usize) -> (RuntimeInstruction, usize) {
        let mut idx = i;
        let b = crate::opcodes::Opcode::from(bc.read_u8(idx));
        idx += 1;
        match b {
            crate::opcodes::Opcode::NOP => (RuntimeInstruction::NOP, idx),
            crate::opcodes::Opcode::DUP => (RuntimeInstruction::DUP, idx),
            crate::opcodes::Opcode::POP => (RuntimeInstruction::POP, idx),
            crate::opcodes::Opcode::PUSH => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                (RuntimeInstruction::PUSH(arg0), idx)
            }
            crate::opcodes::Opcode::FLOOKUP => (RuntimeInstruction::FLOOKUP, idx),
            crate::opcodes::Opcode::TLOOKUP => (RuntimeInstruction::TLOOKUP, idx),
            crate::opcodes::Opcode::CALL => (RuntimeInstruction::CALL, idx),
            crate::opcodes::Opcode::TYPEOF => (RuntimeInstruction::TYPEOF, idx),
            crate::opcodes::Opcode::FROMSLOT => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                (RuntimeInstruction::FROMSLOT(arg0), idx)
            }
            crate::opcodes::Opcode::TOSLOT => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                (RuntimeInstruction::TOSLOT(arg0), idx)
            }
            crate::opcodes::Opcode::ADD => (RuntimeInstruction::ADD, idx),
            crate::opcodes::Opcode::EQUAL => (RuntimeInstruction::EQUAL, idx),
            crate::opcodes::Opcode::NOT => (RuntimeInstruction::NOT, idx),
            crate::opcodes::Opcode::JUMP => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                (RuntimeInstruction::JUMP(arg0), idx)
            }
            crate::opcodes::Opcode::JTRUE => {
                let arg0 = bc.read_u16(idx);
                idx += 2;
                (RuntimeInstruction::JTRUE(arg0), idx)
            }
            crate::opcodes::Opcode::RET => (RuntimeInstruction::RET, idx),
            crate::opcodes::Opcode::MKARRTYPE => (RuntimeInstruction::MKARRTYPE, idx),
            crate::opcodes::Opcode::MKRECTYPE => (RuntimeInstruction::MKRECTYPE, idx),
            crate::opcodes::Opcode::NEWARR => (RuntimeInstruction::NEWARR, idx),
            crate::opcodes::Opcode::ARRGET => (RuntimeInstruction::ARRGET, idx),
            crate::opcodes::Opcode::ARRSET => (RuntimeInstruction::ARRSET, idx),
            crate::opcodes::Opcode::ARRLEN => (RuntimeInstruction::ARRLEN, idx),
            _ => {
                panic!("invalid opcode value")
            }
        }
    }
}
