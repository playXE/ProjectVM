/*
TODO

use crate::instruction::*;
use crate::types::Type;
use crate::types::Ptr;


pub struct Reader {
    src: Vec<u8>,
    ip: usize,
}

impl Reader {
    pub fn new(src: Vec<u8>) -> Reader {
        Reader {
            src,
            ip: 0,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let byte = &self.src[self.ip];
        self.ip += 1;

        *byte
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut bytes: [u8;4] = [0;4];

        for i in 0..bytes.len() {
            let byte = self.read_u8();
            bytes[i] = byte;
        }

        unsafe {::std::mem::transmute(bytes)}
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut bytes: [u8;8] = [0;8];

        for i in 0..bytes.len() {
            let byte = self.read_u8();
            bytes[i] = byte;
        }

        unsafe {::std::mem::transmute(bytes)}
    }

    pub fn get_instruction(&mut self) -> Result<Instruction,String> {
        match self.read_u8() {
            0x1 => {
                let int_size = self.read_u8();

                if int_size > 128 {
                    return Err(format!("Wrong integer size {}",int_size));
                }
                if int_size > 64 {
                    unimplemented!()
                }
                let reg = self.read_u8() as u32;
                if int_size <= 32 {
                    let int = self.read_u32();
                    return Ok(Instruction::LoadImm(reg,int as i64,Type::Int(int_size)));
                }

                if int_size >= 64 {
                    let int = self.read_u64();
                    return Ok(Instruction::LoadImm(reg,int as i64,Type::Int(int_size)));
                }
                unreachable!()
            }

            0x2 => {
                let reg = self.read_u8() as u32;

                let bits = self.read_u32();
                let f = f32::from_bits(bits);

                return Ok(Instruction::LoadFloat(reg,f));
            }

            0x3 => {
                let reg = self.read_u8() as u32;
                let bits = self.read_u64();
                let f = f64::from_bits(bits);

                return Ok(Instruction::LoadDouble(reg,f));
            }

            0x4 => {
                let op = self.read_u8();

                let binop = BinOp::from(op);

                let r1 = self.read_u8() as u32;
                let r2 = self.read_u8() as u32;

                return Ok(Instruction::IntBin(binop,r1,r2));
            }
            0x5 => {
                let op = self.read_u8();

                let binop = BinOp::from(op);

                let r1 = self.read_u8() as u32;
                let r2 = self.read_u8() as u32;

                return Ok(Instruction::FloatBin(binop,r1,r2));
            }

            0x6 => {
                let r = self.read_u8() as u32;
                let psize = self.read_u32();
                let mut bytes = Vec::with_capacity(psize as usize);
                for i in 0..psize as usize {
                    let byte = self.read_u8();
                    bytes[i] = byte;
                }

                return Ok(Instruction::LoadPtr(r,Ptr(bytes)));
            }

            0x7 => {
                let r = self.read_u8() as u32;
                let argc = self.read_u8() as u32;

                return Ok(Instruction::Call(r,argc));
            }

            0x8 => {
                let r = self.read_u8() as u32;
                return Ok(Instruction::Push(r));
            }

            0x9 => {
                let to = self.read_u8() as u32;
                let from = self.read_u8() as u32;
                let off = self.read_u32() as i32;

                return Ok(Instruction::LoadOffset(to,from,off));
            }
            0xc1 => {
                let to = self.read_u8() as u32;
                let from = self.read_u8() as u32;
                let off = self.read_u32() as i32;
                return Ok(Instruction::StoreOffset(to,from,off));
            }





            v => Err(format!("Unrecognized opcode `{:02}`",v))
        }
    }
}*/