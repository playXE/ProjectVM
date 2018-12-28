use crate::types::Type;
use crate::types::Ptr;

#[derive(Debug,Clone)]
pub enum Instruction {
    /// Load immediate value
    LoadImm(u32,i64,Type),
    /// Load float
    LoadFloat(u32,f32),
    /// Load double
    LoadDouble(u32,f64),
    /// Load pointer to some type
    LoadPtr(u32,Ptr<Vec<u8>>,Option<Type>),


    /// Integer binary operation
    IntBin(BinOp,u32,u32),

    /// Float binary operation
    FloatBin(BinOp,u32,u32),
    ConvOp(u32,Type,Type),
    /// Call value at reg with some argument counter
    Call(u32,u32),

    /// Branch if value at reg != 0 (>= 0)
    BranchNZ(u32,u32),
    /// Branch if value at reg is zero
    BranchZ(u32,u32),
    /// Goto block
    Branch(u32),
    /// Push value to stack
    Push(u32),

    //LoadOffset(u32,u32,i32),
    //StoreOffset(u32,u32,i32),

    LoadGlobal(u32,u32),
    StoreGlobal(u32,u32),

    RetInt(u32),
    RetFloat(u32),
    RetDouble(u32),
    RetPtr(u32),
    RetVoid,
}


#[derive(Clone,Debug,PartialEq,Eq,Copy)]
#[repr(u8)]
pub enum BinOp {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    
    Shr,
    Shl,

    Gt,
    Ge,
    Lt,
    Le,

    Eq,
    Ne,
    And,
    Or,

    Xor,
    BitAnd,
    BitOr,
}

impl From<u8> for BinOp {
    fn from(u: u8) -> BinOp {
        use self::BinOp::*;
        match u {
            0 => Add,
            1 => Sub,
            2 => Div,
            3 => Mul,
            4 => Mod,
            5 => Shr,
            6 => Shl,
            7 => Gt,
            8 => Ge,
            9 => Lt,
            10 => Le,
            11 => Eq,
            12 => Ne,
            13 => And,
            14 => Or,
            15 => Xor,
            16 => BitAnd,
            17 => BitOr,
            _ => unreachable!()
        }
    }
}

#[derive(Clone,Debug)]
pub struct Block {
    pub instructions: Vec<Instruction>,
    pub ip: usize,
}

impl Block {
    pub fn new() -> Block {
        Block {
            instructions: Vec::new(),
            ip: 0,
        }
    }

    pub fn next_ins(&mut self) -> &Instruction {
        let ins = &self.instructions[self.ip];
        ins
    }
}

impl From<Vec<Instruction>> for Block {
    fn from(v: Vec<Instruction>) -> Block {
        let mut block = Block::new();
        block.instructions = v;
        block
    }
}