extern crate project_vm;

use self::project_vm::types::*;
use self::project_vm::interpreter::*;
use self::project_vm::instruction::*;

fn main() {
    let blocks = vec![
        Block::from(vec![
            Instruction::LoadImm(0,2,Type::Int32),
            Instruction::LoadImm(1,2,Type::Int32),
            Instruction::IntBin(BinOp::Add,0,1),
            Instruction::RetInt(0)
        ]),
    ];

    let mut interpreter = Interpreter::new();
    let ret = interpreter.run_blocks(blocks);

    let int: i32 = ret.get_int32();
    println!("{}",int);
}
