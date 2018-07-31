use asm::{Address, Condition, Instruction, Pair, Port, Register};
use std::iter::IntoIterator;

use asm::Instruction::*;
use asm::Pair::*;
use asm::Register::*;

use self::DecodeError::{MissingTrailing, NoData, UnknownOpcode};

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    NoData,
    MissingTrailing(u8),
    UnknownOpcode(u8),
}

pub fn decode(data: &[u8]) -> Result<Instruction, DecodeError> {
    if data.len() == 0 {
        return Err(NoData);
    }

    Ok(match data[0] {
        0x00 => NOP,
        0x01 => LXI {
            src: fetch_two(data)?,
            dest: BC,
        },
        0x02 => STAX(BC),
        0x03 => INX(BC),
        0x04 => INRReg(B),
        0x05 => DCRReg(B),
        0x06 => MVIReg {
            src: fetch_one(data)?,
            dest: B,
        },
        0x07 => RLC,
        0x08 => return Err(UnknownOpcode(0x08)),
        0x09 => DAD(BC),
        0x0a => LDAX(BC),
        0x0b => DCX(BC),
        0x0c => INRReg(C),
        0x0d => DCRReg(C),
        0x0e => MVIReg {
            src: fetch_one(data)?,
            dest: C,
        },
        0x0f => RRC,
        data => unimplemented!(),
    })
}

fn fetch_one(data: &[u8]) -> Result<u8, DecodeError> {
    if data.len() >= 2 {
        Ok(data[1])
    } else {
        Err(MissingTrailing(data[0]))
    }
}

fn fetch_two(data: &[u8]) -> Result<u16, DecodeError> {
    if data.len() >= 3 {
        Ok((data[2] as u16) << 8 | (data[1] as u16))
    } else {
        Err(MissingTrailing(data[0]))
    }
}
