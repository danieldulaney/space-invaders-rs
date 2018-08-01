use asm::Instruction;
use asm::Instruction::*;

pub struct InstructionInfo {
    name: &str,
    cycles: u32,
}

// Lifted from 4-15 (59)
pub fn get_info(instr: Instruction) -> InstructionInfo {
    match instr {
        MOVRegReg { .. } => InstructionInfo {
            name: "MOV",
            cycles: 5,
        },
        MOVRegMem { .. } => InstructionInfo {
            name: "MOV",
            cycles: 7,
        },
        MOVMemReg { .. } => InstructionInfo {
            name: "MOV",
            cycles: 7,
        },
        HLT => InstructionInfo {
            name: "HLT",
            cycles: 7,
        },
        _ => unimplemented!(),
    }
}
