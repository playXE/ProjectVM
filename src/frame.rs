use crate::types::*;
use crate::instruction::*;

use std::collections::HashMap;

pub type Reg = u32;

#[derive(Clone,Debug)]
pub struct Frame {
    pub registers: Vec<Value>,
//pub current_sig: FunctionSig,
    pub blocks: Vec<Block>,
    pub stack: Vec<Value>,

    /// Block pointer
    pub bp: usize,
}

impl Frame {
    pub fn init_regs(&mut self) {
        for _ in 0..256 {
            self.registers.push(Value {ty: Type::Void,mem: vec![]});
        }
    }

    pub fn curr_block<'r>(&'r self) -> &'r Block {
        &self.blocks[self.bp]
    }

    pub fn update_ip(&mut self,ip: usize) {
        let mut block = &mut self.blocks[self.bp];
        block.ip += ip;
    }

    pub fn fetch_op(&mut self) -> Instruction {
        let block = &mut self.blocks[self.bp];
        let ins = block.next_ins();
        ins.clone()
    }

    pub fn dispatch_block(&mut self) {
        self.bp += 1;
    }
}



