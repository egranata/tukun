use std::{cell::RefCell, rc::Rc};

use crate::instruction_def::InstructionDef;
use crate::{bytecode::Bytecode, module_definition::FunctionDef};

#[derive(Debug)]
struct BasicBlockImpl {
    name: String,
    content: Vec<InstructionDef>,
    offset: usize,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    val: Rc<RefCell<BasicBlockImpl>>,
}

impl BasicBlock {
    pub fn new(name: &str) -> Self {
        Self {
            val: Rc::new(RefCell::new(BasicBlockImpl {
                name: name.to_owned(),
                content: vec![],
                offset: 0,
            })),
        }
    }

    pub fn append_instruction(&mut self, i: InstructionDef) -> &mut Self {
        self.val.borrow_mut().content.push(i);
        self
    }

    pub fn name(&self) -> String {
        self.val.as_ref().borrow().name.clone()
    }

    pub fn runtime_size(&self) -> usize {
        let mut sz: usize = 0;
        let content = self.val.as_ref().borrow();
        for i in &content.content {
            sz += i.runtime_size()
        }
        sz
    }

    pub fn set_offset(&mut self, o: usize) {
        let mut content = self.val.as_ref().borrow_mut();
        content.offset = o;
    }

    pub fn offset(&self) -> usize {
        let content = self.val.as_ref().borrow();
        content.offset
    }

    pub fn write(&self, bc: &mut Bytecode) {
        let content = self.val.as_ref().borrow();
        for i in &content.content {
            i.write(bc);
        }
    }

    pub fn is_terminated(&self) -> bool {
        let content = self.val.as_ref().borrow();
        for i in &content.content {
            if i.is_terminal() {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct Builder {
    name: String,
    blocks: Vec<BasicBlock>,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            blocks: vec![],
        }
    }

    pub fn append_block(&mut self, name: &str) -> BasicBlock {
        self.blocks.push(BasicBlock::new(name));
        self.blocks.last().cloned().unwrap()
    }

    pub fn find_block(&self, name: &str) -> Option<BasicBlock> {
        for blk in &self.blocks {
            if blk.name() == name {
                return Some(blk.clone());
            }
        }
        None
    }

    pub fn is_terminated(&self) -> bool {
        for b in &self.blocks {
            if !b.is_terminated() {
                return false;
            }
        }
        true
    }

    pub fn generate(&mut self) -> FunctionDef {
        let mut bc = Bytecode::default();
        let mut i = 0;
        let mut sz: usize = 0;
        while i < self.blocks.len() {
            self.blocks[i].set_offset(sz);
            sz += self.blocks[i].runtime_size();
            i += 1;
        }

        i = 0;
        while i < self.blocks.len() {
            let block = &self.blocks[i];
            block.write(&mut bc);
            i += 1;
        }

        FunctionDef::new(&self.name, bc)
    }
}
