use crate::assembler::assembler::BaseAssembler;
use crate::assembler::binemit::{Label, LabelUse};
use crate::assembler::codeholder::CodeBuffer;
use crate::encdec::riscv::*;
use crate::AsmError;
/// An underlying assembler behind `Assembler`. This assembler type solely implements
/// instruction encoding without labels and relocations. For full-fledged assembler look into [`Assembler`].
pub struct RawRISCVAssembler<'a> {
    base: BaseAssembler<'a>,
}

impl<'a> RawRISCVAssembler<'a> {
    pub fn emit(&mut self, inst: u32) {
        self.base.buffer.write_u32(inst);
    }

    pub fn emit_compressed(&mut self, inst: u16) {
        self.base.buffer.write_u16(inst);
    }
}

impl<'a> core::ops::Deref for RawRISCVAssembler<'a> {
    type Target = BaseAssembler<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> core::ops::DerefMut for RawRISCVAssembler<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

include!("impl.rs");

/// RV32/RV64 assembler implementation.
///
/// [`riscv::Assembler`](Assembler) is a code emitter that emits machine code directly into the [CodeBuffer]. The assembler is capable
/// of targeting both 32-bit and 64-bit instruction sets.
///
/// ## Supported extensions
///
/// We aim to support all extensions from RISCV specification (except unratified ones). At the moment *every* extension except `C` is supported
/// in full capacity. For compressed encoding we're lacking in immediate and register encoding, you still can try to emit compressed opcodes
/// but they're not guaranteed to be valid at the moment (they're all prefixed with `c_` in API).
pub struct Assembler<'a> {
    pub base: RawRISCVAssembler<'a>,
}

impl<'a> Assembler<'a> {
    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Self {
            base: RawRISCVAssembler {
                base: BaseAssembler { buffer },
            },
        }
    }


    pub fn jal(&mut self, rd: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer.use_label_at_offset(offset, label, LabelUse::RVJal20);
        self.base.jal(rd, 0)
    }

    pub fn beq(&mut self, rs1: Reg, rs2: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer.use_label_at_offset(offset, label, LabelUse::RVB12);
        self.base.beq(0, rs1, rs2)
    }
}

const fn sext<const B: i64>(val: i64) -> i64 {
    (val << (64 - B)) >> (64 - B)
}

const fn sext_(val: i64, b: i64) -> i64 {
    (val << (64 - b)) >> (64 - b)
}

impl<'a> core::ops::Deref for Assembler<'a> {
    type Target = RawRISCVAssembler<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> core::ops::DerefMut for Assembler<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub use crate::encdec::riscv::regs::*;
