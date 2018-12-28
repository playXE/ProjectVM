use crate::types::*;
use crate::instruction::*;
use crate::frame::*;
use crate::transmute;

use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct Interpreter {
    functions: HashMap<FunctionSig,Function>,
    functions_sig: HashMap<String,FunctionSig>,


    structures: HashMap<StructSig,Struct>,
    structures_sig: HashMap<String,StructSig>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            functions: HashMap::new(),
            functions_sig: HashMap::new(),

            structures: HashMap::new(),
            structures_sig: HashMap::new(),
        }
    }

    pub fn run_blocks(&mut self,blocks: Vec<Block>) -> Value {

        let mut frame = Frame {
            bp: 0,
            blocks,
            registers: vec![],
            stack: vec![],
        };
        frame.init_regs();

        let ret = self.run_instructions(&mut frame);
        ret
    }

    pub fn run_instructions(&mut self,f: &mut Frame) -> Value {
        let mut ret = false;
        let mut retv = None;
        while f.curr_block().ip < f.curr_block().instructions.len() {  
            let ins = f.fetch_op();
            if ret {
                break;
            }
            f.update_ip(1);

            match ins {
                Instruction::LoadImm(dest,imm,ty) => {
                    let bytes: [u8;8] = unsafe {::std::mem::transmute(imm)};
                    let value = Value {
                        ty,
                        mem: bytes.to_vec(),
                    };

                    f.registers[dest as usize] = value;
                }
                Instruction::IntBin(op,r1,r2) => {
                    
                    let (v1,v2) = (&f.registers[r1 as usize],&f.registers[r2 as usize]);

                    let result = match v1.ty {
                        Type::Int8 => {
                            match op {
                                BinOp::Add => Value::new_int(Type::Int8, (v1.get_int8() + v2.get_int8()) as i64),
                                BinOp::Sub => Value::new_int(Type::Int8, (v1.get_int8() - v2.get_int8()) as i64),
                                BinOp::Div => Value::new_int(Type::Int8, (v1.get_int8() / v2.get_int8()) as i64),
                                BinOp::Mul => Value::new_int(Type::Int8, (v1.get_int8() * v2.get_int8()) as i64),
                                _ => unimplemented!()
                            }
                        }
                        Type::Int16 => {
                            match op {
                                BinOp::Add => Value::new_int(Type::Int16, (v1.get_int16() + v2.get_int16()) as i64),
                                BinOp::Sub => Value::new_int(Type::Int16, (v1.get_int16() - v2.get_int16()) as i64),
                                BinOp::Div => Value::new_int(Type::Int16, (v1.get_int16() / v2.get_int16()) as i64),
                                BinOp::Mul => Value::new_int(Type::Int16, (v1.get_int16() * v2.get_int16()) as i64),
                                _ => unimplemented!()
                            }
                        }

                        Type::Int32 => {
                            match op {
                                BinOp::Add => Value::new_int(Type::Int32, (v1.get_int32() + v2.get_int32()) as i64),
                                BinOp::Sub => Value::new_int(Type::Int32, (v1.get_int32() - v2.get_int32()) as i64),
                                BinOp::Div => Value::new_int(Type::Int32, (v1.get_int32() / v2.get_int32()) as i64),
                                BinOp::Mul => Value::new_int(Type::Int32, (v1.get_int32() * v2.get_int32()) as i64),
                                _ => unimplemented!()
                            }
                        }

                        Type::Int64 => {
                            match op {
                                BinOp::Add => Value::new_int(Type::Int64, v1.get_int64() + v2.get_int64()),
                                BinOp::Sub => Value::new_int(Type::Int64, v1.get_int64() - v2.get_int64()),
                                BinOp::Div => Value::new_int(Type::Int64, v1.get_int64() / v2.get_int64()),
                                BinOp::Mul => Value::new_int(Type::Int64, v1.get_int64() * v2.get_int64()),
                                _ => unimplemented!()
                            }
                        }



                        _ => panic!("Expected int type"),
                    };

                    f.registers[r1 as usize] = result;
                }

                Instruction::Branch(bp) => {
                    f.bp = bp as usize;
                }

                Instruction::RetVoid => {
                    ret = true;
                }
                Instruction::RetInt(i) => {
                    ret = true;

                    let v = &f.registers[i as usize];
                    retv = Some(v.clone());
                }

                _ => unimplemented!()
            }
        }

        return retv.unwrap();
    }
}