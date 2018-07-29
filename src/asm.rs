// See http://www.nj7p.info/Manuals/PDFs/Intel/9800153B.pdf
#[derive(Debug)]
pub enum Instruction {
    // Page 4-4
    MOVRegReg { dest: Register, src: Register },
    MOVMemReg { dest: Register },
    MOVRegMem { src: Register },
    MVIReg { dest: Register, src: u8 },
    MVIMem { src: u8 },
    LXI { dest: Pair, src: u16 },

    // Page 4-5
    LDA { src: Address },
    STA { dest: Address },
    LHLD { src: Address },
    SHLD { dest: Address },
    LDAX { src: Pair },
    STAX { dest: Pair },
    XCHG,

    // Page 4-6
    ADDReg(Register),
    ADDMem(Address),
    ADI(u8),
    ADCReg(Register),
    ADCMem,
    ACI(u8),
    SUBReg(Register),

    // Page 4-7
    SUBMem,
    SUI(u8),
    SBBReg(Register),
    SBBMem,
    SBI(u8),
    INRReg(Register),
    INRMem,
    DCRReg(Register),

    // Page 4-8
    DCRMem,
    INX(Pair),
    DCX(Pair),
    DAD(Pair),
    DAA,
    ANAReg(Register),
    ANAMem(Address),

    // Page 4-9
    ANI(u8),
    XRAReg(Register),
    XRAMem(Address),
    XRI(u8),
    ORAReg(Register),
    ORAMem(Address),
    ORI(u8),
    CMPReg(Register),

    // Page 4-10
    CMPMem(Address),
    CPI(u8),
    RLC,
    RRC,
    RAL,
    RAR,
    CMA,

    // Page 4-11
    CMC,
    STC,
    JMP(Address),
    JCond(Condition, Address),
    CALL(Address),

    // Page 4-12
    CCond(Condition, Address),
    RET,
    RCond(Condition),
    RST(u8),
    PCHL,

    // Page 4-13
    PUSH(Pair),
    PUSHPSW,
    POP(Pair),
    POPPSW,

    // Page 4-14
    XTHL,
    SPHL,
    IN(Port),
    OUT(Port),
    EI,
    DI,
    HLT,
    NOP,

    Unknown(Option<u8>),
}

pub type Address = u16;

pub type Port = u8;

#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pair {
    B,
    D,
    H,
    SP,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
    PO,
    PE,
    P,
    M,
}

impl Register {
    /// Determine the register based on the three least-significant bits.
    ///
    /// All higher bits are ignored. Uses the code on page 4-3 of the manual.
    ///
    /// # Panics
    ///
    /// Panics on inputs with 6 (`0b110`) in the relevant bits.
    pub fn from_code(code: u8) -> Register {
        match code & 0x07 {
            // 0b00000111
            0x07 => Register::A,
            0x00 => Register::B,
            0x01 => Register::C,
            0x02 => Register::D,
            0x03 => Register::E,
            0x04 => Register::H,
            0x05 => Register::L,
            _ => panic!("Bad register number: {}", code),
        }
    }
}

impl Pair {
    /// Determine the register pair based on the two least-significant bits.
    ///
    /// All higher bits are ignored. Uses the code on page 4-3 of the manual.
    pub fn from_code(code: u8) -> Pair {
        match code & 0x03 {
            // 0b00000011
            0x00 => Pair::B,
            0x01 => Pair::D,
            0x02 => Pair::H,
            0x03 => Pair::SP,
            _ => unreachable!("Used masking to eliminate higher bits; this is impossible"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_registers() {
        assert_eq!(Register::from_code(0xFF), Register::A);
        assert_eq!(Register::from_code(0), Register::B);
        assert_eq!(Register::from_code(65), Register::C);
        assert_eq!(Register::from_code(2), Register::D);
        assert_eq!(Register::from_code(99), Register::E);
        assert_eq!(Register::from_code(0xF4), Register::H);
        assert_eq!(Register::from_code(0xAD), Register::L);
    }

    #[test]
    #[should_panic]
    fn panics_on_register_6() {
        Register::from_code(70);
    }

    #[test]
    fn correct_pairs() {
        assert_eq!(Pair::from_code(0), Pair::B);
        assert_eq!(Pair::from_code(97), Pair::D);
        assert_eq!(Pair::from_code(26), Pair::H);
        assert_eq!(Pair::from_code(0xFF), Pair::SP);
    }
}
