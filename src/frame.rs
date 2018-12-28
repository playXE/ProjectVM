use crate::types::*;
use crate::instruction::*;

use std::collections::HashMap;

pub type Reg = u32;

pub struct Frame {
    registers: HashMap<Reg,Value>,
    
}
