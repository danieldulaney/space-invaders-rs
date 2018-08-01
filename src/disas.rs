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
        // 0x08 is undefined
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
        // 0x10 is undefined
        0x11 => LXI {
            src: fetch_two(data)?,
            dest: DE,
        },
        0x12 => STAX(DE),
        0x13 => INX(DE),
        0x14 => INRReg(D),
        0x15 => DCRReg(D),
        0x16 => MVIReg {
            src: fetch_one(data)?,
            dest: D,
        },
        0x17 => RAL,
        // 0x18 is undefined
        0x19 => DAD(DE),
        0x1a => LDAX(DE),
        0x1b => DCX(DE),
        0x1c => INRReg(E),
        0x1d => DCRReg(E),
        0x1e => MVIReg {
            src: fetch_one(data)?,
            dest: E,
        },
        0x1f => RAR,
        // 0x20 is not defined on 8080; may be defined for others
        0x21 => LXI {
            src: fetch_two(data)?,
            dest: HL,
        },
        0x22 => SHLD {
            dest: fetch_two(data)?,
        },
        0x23 => INX(HL),
        0x24 => INRReg(H),
        0x25 => DCRReg(H),
        0x26 => MVIReg {
            src: fetch_one(data)?,
            dest: H,
        },
        0x27 => DAA,
        // 0x28 is undefined
        0x29 => DAD(HL),
        0x2a => LHLD {
            src: fetch_two(data)?,
        },
        0x2b => DCX(HL),
        0x2c => INRReg(L),
        0x2d => DCRReg(L),
        0x2e => MVIReg {
            src: fetch_one(data)?,
            dest: L,
        },
        0x2f => CMA,
        // 0x30 is not defined on 8080; may be defined for others
        0x31 => LXI {
            src: fetch_two(data)?,
            dest: SP,
        },
        0x32 => STA {
            dest: fetch_two(data)?,
        },
        0x33 => INX(SP),
        0x34 => INRMem,
        0x35 => DCRMem,
        0x36 => MVIMem {
            src: fetch_one(data)?,
        },
        0x37 => STC,
        // 0x38 is undefined
        0x39 => DAD(SP),
        0x40 => MOVRegReg { src: B, dest: B },
        0x41 => MOVRegReg { src: C, dest: B },
        0x42 => MOVRegReg { src: D, dest: B },
        0x43 => MOVRegReg { src: E, dest: B },
        0x44 => MOVRegReg { src: H, dest: B },
        0x45 => MOVRegReg { src: L, dest: B },
        0x46 => MOVMemReg { dest: B },
        0x47 => MOVRegReg { src: A, dest: B },
        0x48 => MOVRegReg { src: B, dest: C },
        0x49 => MOVRegReg { src: C, dest: C },
        0x4a => MOVRegReg { src: D, dest: C },
        0x4b => MOVRegReg { src: E, dest: C },
        0x4c => MOVRegReg { src: H, dest: C },
        0x4d => MOVRegReg { src: L, dest: C },
        0x4e => MOVMemReg { dest: C },
        0x4f => MOVRegReg { src: A, dest: C },
        0x50 => MOVRegReg { src: B, dest: D },
        0x51 => MOVRegReg { src: C, dest: D },
        0x52 => MOVRegReg { src: D, dest: D },
        0x53 => MOVRegReg { src: E, dest: D },
        0x54 => MOVRegReg { src: H, dest: D },
        0x55 => MOVRegReg { src: L, dest: D },
        0x56 => MOVMemReg { dest: D },
        0x57 => MOVRegReg { src: A, dest: D },
        0x58 => MOVRegReg { src: B, dest: E },
        0x59 => MOVRegReg { src: C, dest: E },
        0x5a => MOVRegReg { src: D, dest: E },
        0x5b => MOVRegReg { src: E, dest: E },
        0x5c => MOVRegReg { src: H, dest: E },
        0x5d => MOVRegReg { src: L, dest: E },
        0x5e => MOVMemReg { dest: E },
        0x5f => MOVRegReg { src: A, dest: E },
        0x60 => MOVRegReg { src: B, dest: H },
        0x61 => MOVRegReg { src: C, dest: H },
        0x62 => MOVRegReg { src: D, dest: H },
        0x63 => MOVRegReg { src: E, dest: H },
        0x64 => MOVRegReg { src: H, dest: H },
        0x65 => MOVRegReg { src: L, dest: H },
        0x66 => MOVMemReg { dest: H },
        0x67 => MOVRegReg { src: A, dest: H },
        0x68 => MOVRegReg { src: B, dest: L },
        0x69 => MOVRegReg { src: C, dest: L },
        0x6a => MOVRegReg { src: D, dest: L },
        0x6b => MOVRegReg { src: E, dest: L },
        0x6c => MOVRegReg { src: H, dest: L },
        0x6d => MOVRegReg { src: L, dest: L },
        0x6e => MOVMemReg { dest: L },
        0x6f => MOVRegReg { src: A, dest: L },
        0x70 => MOVRegMem { src: B },
        0x71 => MOVRegMem { src: C },
        0x72 => MOVRegMem { src: D },
        0x73 => MOVRegMem { src: E },
        0x74 => MOVRegMem { src: H },
        0x75 => MOVRegMem { src: L },
        0x76 => HLT,
        0x77 => MOVRegMem { src: A },
        0x78 => MOVRegReg { src: B, dest: A },
        0x79 => MOVRegReg { src: C, dest: A },
        0x7a => MOVRegReg { src: D, dest: A },
        0x7b => MOVRegReg { src: E, dest: A },
        0x7c => MOVRegReg { src: H, dest: A },
        0x7d => MOVRegReg { src: L, dest: A },
        0x7e => MOVMemReg { dest: A },
        0x7f => MOVRegReg { src: A, dest: A },
        0x80 => ADDReg(B),
        0x81 => ADDReg(C),
        0x82 => ADDReg(D),
        0x83 => ADDReg(E),
        0x84 => ADDReg(H),
        0x85 => ADDReg(L),
        0x86 => ADDMem,
        0x87 => ADDReg(A),
        0x88 => ADCReg(B),
        0x89 => ADCReg(C),
        0x8a => ADCReg(D),
        0x8b => ADCReg(E),
        0x8c => ADCReg(H),
        0x8d => ADCReg(L),
        0x8e => ADCMem,
        0x8f => ADCReg(A),
        0x90 => SUBReg(B),
        0x91 => SUBReg(C),
        0x92 => SUBReg(D),
        0x93 => SUBReg(E),
        0x94 => SUBReg(H),
        0x95 => SUBReg(L),
        0x96 => SUBMem,
        0x97 => SUBReg(A),
        0x98 => SBBReg(B),
        0x99 => SBBReg(C),
        0x9a => SBBReg(D),
        0x9b => SBBReg(E),
        0x9c => SBBReg(H),
        0x9d => SBBReg(L),
        0x9e => SBBMem,
        0x9f => SBBReg(A),
        0xa0 => ANAReg(B),
        0xa1 => ANAReg(C),
        0xa2 => ANAReg(D),
        0xa3 => ANAReg(E),
        0xa4 => ANAReg(H),
        0xa5 => ANAReg(L),
        0xa6 => ANAMem,
        0xa7 => ANAReg(A),
        0xa8 => XRAReg(B),
        0xa9 => XRAReg(C),
        0xaa => XRAReg(D),
        0xab => XRAReg(E),
        0xac => XRAReg(H),
        0xad => XRAReg(L),
        0xae => XRAMem,
        0xaf => XRAReg(A),
        0xb0 => ORAReg(B),
        0xb1 => ORAReg(C),
        0xb2 => ORAReg(D),
        0xb3 => ORAReg(E),
        0xb4 => ORAReg(H),
        0xb5 => ORAReg(L),
        0xb6 => ORAMem,
        0xb7 => ORAReg(A),
        0xb8 => CMPReg(B),
        0xb9 => CMPReg(C),
        0xba => CMPReg(D),
        0xbb => CMPReg(E),
        0xbc => CMPReg(H),
        0xbd => CMPReg(L),
        0xbe => CMPMem,
        0xbf => CMPReg(A),
        data => return Err(UnknownOpcode(data)),
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
