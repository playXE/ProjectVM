use super::Type;
use super::Ptr;
use crate::transmute;

#[derive(Clone,Debug)]
pub struct Value {
    pub ty: Type,
    pub mem: Vec<u8>,
}



impl Value {
    pub fn new_int(ty: Type,imm: i64) -> Value {
        let bytes: [u8;8] = unsafe {::std::mem::transmute(imm)};
        Value {
            ty,
            mem: bytes.to_vec()
        }
    }

    pub fn new(ty: Type) -> Value {
        Value {
            ty: ty,
            mem: Vec::new(),
        }
    }

    pub fn get_float(&self) -> f32 {
        if self.ty.is_float() {
            let mut bits: [u8;4] = [0;4];
            for i in 0..bits.len() {
                let bit = &self.mem[i];
                bits[i] = *bit;
            }
            let u = unsafe {::std::mem::transmute(bits)};
            let f = f32::from_bits(u);

            f
        } else {
            panic!("Value type not float: {:?}",self.ty)
        }
    }

    pub fn get_double(&self) -> f64 {
        if self.ty.is_double() {
            let mut bits: [u8;8] = [0;8];
            for i in 0..bits.len() {
                let bit = &self.mem[i];
                bits[i] = *bit;
            }
            let u = unsafe {::std::mem::transmute(bits)};
            let f = f64::from_bits(u);

            f
        } else {
            panic!("Value type not float: {:?}",self.ty)
        }
    }

    pub fn get_int8(&self) -> i8 {
        if self.ty.is_int() {
            match self.ty {
                Type::Int8 => self.mem[0] as i8,
                Type::Int16 => self.get_int16() as i8,
                Type::Int32 => self.get_int32() as i8,
                Type::Int64 => self.get_int64() as i8,
                _ => unreachable!()
            }
        } else {
            panic!("Expected Int type,found `{:?}`",self.ty);
        }
    }

    pub fn get_int16(&self) -> i16 {
        if self.ty.is_int() {
            match self.ty {
                Type::Int8 => self.get_int8() as i16,
                Type::Int16 => {
                    let mut bits: [u8;2] = [0;2];
                    bits[0] = self.mem[0];
                    bits[1] = self.mem[1];
                
                    transmute!(bits)
                }

                Type::Int32 => self.get_int32() as i16,
                Type::Int64 => self.get_int64() as i16,

                _ => unreachable!()
            }
            
            
        } else {
            panic!("Expected Int type,found `{:?}`",self.ty);
        }
    }

    pub fn get_int32(&self) -> i32 {
        if self.ty.is_int() {
            match self.ty {
                Type::Int8 => self.get_int8() as i32,
                Type::Int16 => self.get_int16() as i32,
                Type::Int32 => {
                    let mut bits: [u8;4] = [0;4];
                    bits[0] = self.mem[0];
                    bits[1] = self.mem[1];
                    bits[2] = self.mem[2];
                    bits[3] = self.mem[3];
                    transmute!(bits)
                }
                Type::Int64 => self.get_int64() as i32,
                _ => unreachable!()
            }
            
        } else {
            panic!("Expected Int type,found `{:?}`",self.ty);
        }
    }

    pub fn get_int64(&self) -> i64 {
        if self.ty.is_int() {
            match self.ty {
                Type::Int8 => self.get_int8() as i64,
                Type::Int32 => self.get_int32() as i64,
                Type::Int16 => self.get_int16() as i64,
                Type::Int64 => {
                    let mut bits: [u8;8] = [0;8];
                    bits[0] = self.mem[0]; bits[1] = self.mem[1];
                    bits[2] = self.mem[2]; bits[3] = self.mem[3];
                    bits[4] = self.mem[4]; bits[5] = self.mem[5];
                    bits[6] = self.mem[6]; bits[7] = self.mem[7];
                    transmute!(bits)
                }
                _ => unreachable!()
            }
            
        } else {
            panic!("Expected Int type,found `{:?}`",self.ty);
        }
    }

    pub fn get_ptr(&self) -> Ptr<Type> {
        if self.ty.is_ptr() {
            return self.ty.get_ptr();
        } else {
            panic!()
        }
    }
}