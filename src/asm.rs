pub enum Instruction {
    // Page 4-4
    MOVRegReg { dest: Register, src: Register },
    MOVMemReg { dest: Register, src: Address },
    MOVRegMem { dest: Address, src: Register },
    MVIReg { dest: Register, data: u8 },
    MVIMem { data: u8 },
    LXI { dest: Pair, data: u16 },

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

    Unknown(u8),
}

pub type Address = u16;

pub type Port = u8;

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Pair {
    B,
    D,
    H,
    SP,
}

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

