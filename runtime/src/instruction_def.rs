#[derive(Debug, Clone)]
pub enum InstructionDef {
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
    JUMP(crate::builder::BasicBlock),
    JTRUE(crate::builder::BasicBlock),
    RET,
    MKARRTYPE,
    MKRECTYPE,
    NEWARR,
    ARRGET,
    ARRSET,
    ARRLEN,
}
impl InstructionDef {
    pub fn runtime_size(&self) -> usize {
        match self {
            InstructionDef::NOP => 1,
            InstructionDef::DUP => 1,
            InstructionDef::POP => 1,
            InstructionDef::PUSH(_) => 1 + core::mem::size_of::<u16>(),
            InstructionDef::FLOOKUP => 1,
            InstructionDef::TLOOKUP => 1,
            InstructionDef::CALL => 1,
            InstructionDef::TYPEOF => 1,
            InstructionDef::FROMSLOT(_) => 1 + core::mem::size_of::<u16>(),
            InstructionDef::TOSLOT(_) => 1 + core::mem::size_of::<u16>(),
            InstructionDef::ADD => 1,
            InstructionDef::EQUAL => 1,
            InstructionDef::JUMP(_) => 1 + core::mem::size_of::<u16>(),
            InstructionDef::JTRUE(_) => 1 + core::mem::size_of::<u16>(),
            InstructionDef::RET => 1,
            InstructionDef::MKARRTYPE => 1,
            InstructionDef::MKRECTYPE => 1,
            InstructionDef::NEWARR => 1,
            InstructionDef::ARRGET => 1,
            InstructionDef::ARRSET => 1,
            InstructionDef::ARRLEN => 1,
        }
    }
}
impl InstructionDef {
    pub fn is_terminal(&self) -> bool {
        match self {
            InstructionDef::NOP => false,
            InstructionDef::DUP => false,
            InstructionDef::POP => false,
            InstructionDef::PUSH(_) => false,
            InstructionDef::FLOOKUP => false,
            InstructionDef::TLOOKUP => false,
            InstructionDef::CALL => false,
            InstructionDef::TYPEOF => false,
            InstructionDef::FROMSLOT(_) => false,
            InstructionDef::TOSLOT(_) => false,
            InstructionDef::ADD => false,
            InstructionDef::EQUAL => false,
            InstructionDef::JUMP(_) => true,
            InstructionDef::JTRUE(_) => true,
            InstructionDef::RET => true,
            InstructionDef::MKARRTYPE => false,
            InstructionDef::MKRECTYPE => false,
            InstructionDef::NEWARR => false,
            InstructionDef::ARRGET => false,
            InstructionDef::ARRSET => false,
            InstructionDef::ARRLEN => false,
        }
    }
}
impl InstructionDef {
    pub fn write(&self, bc: &mut crate::bytecode::Bytecode) {
        match self {
            InstructionDef::NOP => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::NOP));
            }
            InstructionDef::DUP => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::DUP));
            }
            InstructionDef::POP => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::POP));
            }
            InstructionDef::PUSH(arg0) => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::PUSH));
                bc.write_u16(*arg0);
            }
            InstructionDef::FLOOKUP => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::FLOOKUP));
            }
            InstructionDef::TLOOKUP => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::TLOOKUP));
            }
            InstructionDef::CALL => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::CALL));
            }
            InstructionDef::TYPEOF => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::TYPEOF));
            }
            InstructionDef::FROMSLOT(arg0) => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::FROMSLOT));
                bc.write_u16(*arg0);
            }
            InstructionDef::TOSLOT(arg0) => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::TOSLOT));
                bc.write_u16(*arg0);
            }
            InstructionDef::ADD => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::ADD));
            }
            InstructionDef::EQUAL => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::EQUAL));
            }
            InstructionDef::JUMP(arg0) => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::JUMP));
                bc.write_u16(arg0.offset() as u16);
            }
            InstructionDef::JTRUE(arg0) => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::JTRUE));
                bc.write_u16(arg0.offset() as u16);
            }
            InstructionDef::RET => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::RET));
            }
            InstructionDef::MKARRTYPE => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::MKARRTYPE));
            }
            InstructionDef::MKRECTYPE => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::MKRECTYPE));
            }
            InstructionDef::NEWARR => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::NEWARR));
            }
            InstructionDef::ARRGET => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::ARRGET));
            }
            InstructionDef::ARRSET => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::ARRSET));
            }
            InstructionDef::ARRLEN => {
                bc.write_u8(u8::from(crate::opcodes::Opcode::ARRLEN));
            }
        }
    }
}
