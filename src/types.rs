use derive_more::Display;

// the numbers correspond to register numbers in ModR/M encoding
#[derive(Debug, Display, Clone, Copy)]
pub enum FullSizeGeneralPurposeRegister {
    EAX = 0,
    EBX = 1,
    ECX = 2,
    EDX = 3,
    ESP = 4,
    EBP = 5,
    ESI = 6,
    EDI = 7,
}

impl TryFrom<Register> for FullSizeGeneralPurposeRegister {
    type Error = ();

    fn try_from(value: Register) -> Result<Self, Self::Error> {
        use FullSizeGeneralPurposeRegister::*;
        match value {
            Register::EAX => Ok(EAX),
            Register::EBX => Ok(EBX),
            Register::ECX => Ok(ECX),
            Register::EDX => Ok(EDX),
            Register::ESP => Ok(ESP),
            Register::EBP => Ok(EBP),
            Register::ESI => Ok(ESI),
            Register::EDI => Ok(EDI),
            _ => Err(())
        }
    }
}

// TODO add more registers
// TODO add subregisters metainfo (stuff like AX is the lower 16 bits of EAX)
#[derive(Debug, Display, Clone, Copy)]
pub enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,

    AX,
    BX,
    CX,
    DX,
    SP,
    BP,
    SI,
    DI,

    AH,
    BH,
    CH,
    DH,

    AL,
    BL,
    CL,
    DL,
}

impl Register {
    pub fn size(self) -> IntType {
        use IntType::*;
        use Register::*;
        match self {
            EAX | EBX | ECX | EDX | ESP | EBP | ESI | EDI => I32,
            AX | BX | CX | DX | SP | BP | SI | DI => I16,
            AH | BH | CH | DH | AL | BL | CL | DL => I8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SegmentRegister {
    CS,
    DS,
    ES,
    FS,
    GS,
    SS
}

#[repr(C)] // for interoperability with llvm-generated functions
pub struct CpuContext {
    pub gp_regs: [u32; 8],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64
}

impl IntType {
    pub fn double_sized(self) -> Self {
        use IntType::*;
        match self {
            I8 => I16,
            I16 => I32,
            I32 => I64,
            I64 => panic!("Can't created a double-sided type for I64"),
        }
    }

    pub fn bit_width(self) -> u8 {
        use IntType::*;
        match self {
            I8 => 8,
            I16 => 16,
            I32 => 32,
            I64 => 64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryOperand {
    pub base: Option<Register>,
    pub displacement: i64,
    pub scale: u8,
    pub index: Option<Register>,
    pub size: Option<IntType>,
    pub segment: Option<SegmentRegister>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(Register),

    Immediate8(u8),
    Immediate16(u16),
    Immediate32(u32),
    Immediate64(u64),

    FarBranch(u16, u32),

    Memory(MemoryOperand),
}

impl Operand {
    pub fn size(&self) -> IntType {
        match self {
            Operand::Register(reg) => reg.size(),
            Operand::Immediate8(_) => IntType::I8,
            Operand::Immediate16(_) => IntType::I16,
            Operand::Immediate32(_) => IntType::I32,
            Operand::Immediate64(_) => IntType::I64,
            Operand::FarBranch(_, _) => todo!(),
            Operand::Memory(m) => m.size.unwrap(),
        }
    }
}