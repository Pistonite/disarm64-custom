#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for UCVTF_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UCVTF_Sd_Sn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ucvtf",
        aliases: &[],
        opcode: 0x7e21d800,
        mask: 0xffbffc00,
        class: InsnClass::ASISDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ucvtf,
            operation: Operation::ASISDMISC(ASISDMISC::UCVTF_Sd_Sn(UCVTF_Sd_Sn::from(bits))),
        }
    }
}
impl InsnOpcode for UCVTF_Sd_Sn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UCVTF_Vd_V_4H_Vn_V_4H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ucvtf",
        aliases: &[],
        opcode: 0x2e79d800,
        mask: 0xbffffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ucvtf,
            operation: Operation::ASIMDMISC(ASIMDMISC::UCVTF_Vd_V_4H_Vn_V_4H(
                UCVTF_Vd_V_4H_Vn_V_4H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for UCVTF_Vd_V_4H_Vn_V_4H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UCVTF_Sd_S_H_Sn_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ucvtf",
        aliases: &[],
        opcode: 0x7e79d800,
        mask: 0xfffffc00,
        class: InsnClass::ASISDMISC,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ucvtf,
            operation: Operation::ASISDMISC(ASISDMISC::UCVTF_Sd_S_H_Sn_S_H(
                UCVTF_Sd_S_H_Sn_S_H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for UCVTF_Sd_S_H_Sn_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UCVTF_Vd_Vn_IMM_VLSR {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ucvtf",
        aliases: &[],
        opcode: 0x2f10e400,
        mask: 0xbf90fc00,
        class: InsnClass::ASIMDSHF,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::IMM_VLSR,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ucvtf,
            operation: Operation::ASIMDSHF(ASIMDSHF::UCVTF_Vd_Vn_IMM_VLSR(
                UCVTF_Vd_Vn_IMM_VLSR::from(bits),
            )),
        }
    }
}
impl InsnOpcode for UCVTF_Vd_Vn_IMM_VLSR {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ucvtf",
        aliases: &[],
        opcode: 0x2f00e400,
        mask: 0xbf80fc00,
        class: InsnClass::ASIMDSHF,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::IMM_VLSR,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ucvtf,
            operation: Operation::ASIMDSHF(ASIMDSHF::UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(
                UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UCVTF_Sd_Sn_IMM_VLSR {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ucvtf",
        aliases: &[],
        opcode: 0x7f10e400,
        mask: 0xff90fc00,
        class: InsnClass::ASISDSHF,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::IMM_VLSR,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ucvtf,
            operation: Operation::ASISDSHF(ASISDSHF::UCVTF_Sd_Sn_IMM_VLSR(
                UCVTF_Sd_Sn_IMM_VLSR::from(bits),
            )),
        }
    }
}
impl InsnOpcode for UCVTF_Sd_Sn_IMM_VLSR {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ucvtf",
        aliases: &[],
        opcode: 0x7f00e400,
        mask: 0xff80fc00,
        class: InsnClass::ASISDSHF,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::IMM_VLSR,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ucvtf,
            operation: Operation::ASISDSHF(ASISDSHF::UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(
                UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl InsnOpcode for AARCH64_MISC {
    fn definition(&self) -> &'static Insn {
        match self {
            AARCH64_MISC::TSTART_Rd(opcode) => opcode.definition(),
            AARCH64_MISC::TTEST_Rd(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            AARCH64_MISC::TSTART_Rd(opcode) => opcode.bits(),
            AARCH64_MISC::TTEST_Rd(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ADDSUB_CARRY {
    fn definition(&self) -> &'static Insn {
        match self {
            ADDSUB_CARRY::SBCS_Rd_Rn_Rm(opcode) => opcode.definition(),
            ADDSUB_CARRY::SBC_Rd_Rn_Rm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ADDSUB_CARRY::SBCS_Rd_Rn_Rm(opcode) => opcode.bits(),
            ADDSUB_CARRY::SBC_Rd_Rn_Rm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ADDSUB_EXT {
    fn definition(&self) -> &'static Insn {
        match self {
            ADDSUB_EXT::ADDS_Rd_Rn_SP_Rm_EXT(opcode) => opcode.definition(),
            ADDSUB_EXT::ADD_Rd_SP_Rn_SP_Rm_EXT(opcode) => opcode.definition(),
            ADDSUB_EXT::SUBS_Rd_Rn_SP_Rm_EXT(opcode) => opcode.definition(),
            ADDSUB_EXT::SUB_Rd_SP_Rn_SP_Rm_EXT(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ADDSUB_EXT::ADDS_Rd_Rn_SP_Rm_EXT(opcode) => opcode.bits(),
            ADDSUB_EXT::ADD_Rd_SP_Rn_SP_Rm_EXT(opcode) => opcode.bits(),
            ADDSUB_EXT::SUBS_Rd_Rn_SP_Rm_EXT(opcode) => opcode.bits(),
            ADDSUB_EXT::SUB_Rd_SP_Rn_SP_Rm_EXT(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ADDSUB_IMM {
    fn definition(&self) -> &'static Insn {
        match self {
            ADDSUB_IMM::ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG(opcode) => opcode.definition(),
            ADDSUB_IMM::ADDS_Rd_Rn_SP_AIMM(opcode) => opcode.definition(),
            ADDSUB_IMM::ADD_Rd_SP_Rn_SP_AIMM(opcode) => opcode.definition(),
            ADDSUB_IMM::SUBG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG(opcode) => opcode.definition(),
            ADDSUB_IMM::SUBS_Rd_Rn_SP_AIMM(opcode) => opcode.definition(),
            ADDSUB_IMM::SUB_Rd_SP_Rn_SP_AIMM(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ADDSUB_IMM::ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG(opcode) => opcode.bits(),
            ADDSUB_IMM::ADDS_Rd_Rn_SP_AIMM(opcode) => opcode.bits(),
            ADDSUB_IMM::ADD_Rd_SP_Rn_SP_AIMM(opcode) => opcode.bits(),
            ADDSUB_IMM::SUBG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG(opcode) => opcode.bits(),
            ADDSUB_IMM::SUBS_Rd_Rn_SP_AIMM(opcode) => opcode.bits(),
            ADDSUB_IMM::SUB_Rd_SP_Rn_SP_AIMM(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ADDSUB_SHIFT {
    fn definition(&self) -> &'static Insn {
        match self {
            ADDSUB_SHIFT::ADDS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            ADDSUB_SHIFT::ADD_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            ADDSUB_SHIFT::SUBS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            ADDSUB_SHIFT::SUB_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ADDSUB_SHIFT::ADDS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            ADDSUB_SHIFT::ADD_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            ADDSUB_SHIFT::SUBS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            ADDSUB_SHIFT::SUB_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDALL {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDALL::ADDV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::UADDLV_Fd_Vn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDALL::ADDV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::UADDLV_Fd_Vn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDDIFF {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDDIFF::ADDHN2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::ADDHN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMULL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMULL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SUBHN2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SUBHN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDW2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDW_Vd_Vn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDDIFF::ADDHN2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::ADDHN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMULL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMULL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SUBHN2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SUBHN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDW2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDW_Vd_Vn_Vm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDELEM {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDELEM::FCMLA_Vd_Vn_Em_IMM_ROT2(opcode) => opcode.definition(),
            ASIMDELEM::FMULX_Vd_Vn_Em(opcode) => opcode.definition(),
            ASIMDELEM::FMULX_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMUL_Vd_Vn_Em(opcode) => opcode.definition(),
            ASIMDELEM::FMUL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::MUL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMULL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMULL_Vd_Vn_Em16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDELEM::FCMLA_Vd_Vn_Em_IMM_ROT2(opcode) => opcode.bits(),
            ASIMDELEM::FMULX_Vd_Vn_Em(opcode) => opcode.bits(),
            ASIMDELEM::FMULX_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMUL_Vd_Vn_Em(opcode) => opcode.bits(),
            ASIMDELEM::FMUL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::MUL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMULL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMULL_Vd_Vn_Em16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDIMM {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDIMM::BIC_Vd_SIMD_IMM_SFT(opcode) => opcode.definition(),
            ASIMDIMM::BIC_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.definition(),
            ASIMDIMM::FMOV_Vd_SIMD_FPIMM(opcode) => opcode.definition(),
            ASIMDIMM::FMOV_Vd_V_2D_SIMD_FPIMM(opcode) => opcode.definition(),
            ASIMDIMM::FMOV_Vd_V_4H_SIMD_FPIMM(opcode) => opcode.definition(),
            ASIMDIMM::MOVI_Sd_SIMD_IMM(opcode) => opcode.definition(),
            ASIMDIMM::MOVI_Vd_SIMD_IMM(opcode) => opcode.definition(),
            ASIMDIMM::MOVI_Vd_SIMD_IMM_SFT(opcode) => opcode.definition(),
            ASIMDIMM::MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL(opcode) => opcode.definition(),
            ASIMDIMM::MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.definition(),
            ASIMDIMM::MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL(opcode) => opcode.definition(),
            ASIMDIMM::MVNI_Vd_SIMD_IMM_SFT(opcode) => opcode.definition(),
            ASIMDIMM::MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL(opcode) => opcode.definition(),
            ASIMDIMM::MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.definition(),
            ASIMDIMM::ORR_Vd_SIMD_IMM_SFT(opcode) => opcode.definition(),
            ASIMDIMM::ORR_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDIMM::BIC_Vd_SIMD_IMM_SFT(opcode) => opcode.bits(),
            ASIMDIMM::BIC_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.bits(),
            ASIMDIMM::FMOV_Vd_SIMD_FPIMM(opcode) => opcode.bits(),
            ASIMDIMM::FMOV_Vd_V_2D_SIMD_FPIMM(opcode) => opcode.bits(),
            ASIMDIMM::FMOV_Vd_V_4H_SIMD_FPIMM(opcode) => opcode.bits(),
            ASIMDIMM::MOVI_Sd_SIMD_IMM(opcode) => opcode.bits(),
            ASIMDIMM::MOVI_Vd_SIMD_IMM(opcode) => opcode.bits(),
            ASIMDIMM::MOVI_Vd_SIMD_IMM_SFT(opcode) => opcode.bits(),
            ASIMDIMM::MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL(opcode) => opcode.bits(),
            ASIMDIMM::MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.bits(),
            ASIMDIMM::MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL(opcode) => opcode.bits(),
            ASIMDIMM::MVNI_Vd_SIMD_IMM_SFT(opcode) => opcode.bits(),
            ASIMDIMM::MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL(opcode) => opcode.bits(),
            ASIMDIMM::MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.bits(),
            ASIMDIMM::ORR_Vd_SIMD_IMM_SFT(opcode) => opcode.bits(),
            ASIMDIMM::ORR_Vd_V_4H_SIMD_IMM_SFT_LSL(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDMISC {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDMISC::CLS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::CLZ_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::CMEQ_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMGE_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMGT_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMLE_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMLT_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CNT_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMEQ_Vd_Vn_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMGE_Vd_Vn_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMGT_Vd_Vn_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMLE_Vd_Vn_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCMLT_Vd_Vn_FPIMM0(opcode) => opcode.definition(),
            ASIMDMISC::FCVTAS_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTAS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTAU_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTAU_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTL2_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTL_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTMS_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTMS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTMU_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTMU_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTN2_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTNS_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTNS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTNU_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTNU_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTN_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTPS_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTPS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTPU_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTPU_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTXN2_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTXN_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTZS_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTZS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FCVTZU_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FCVTZU_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::NEG_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::SCVTF_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::UADDLP_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::UCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::UCVTF_Vd_Vn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDMISC::CLS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::CLZ_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::CMEQ_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMGE_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMGT_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMLE_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMLT_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CNT_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMEQ_Vd_Vn_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMGE_Vd_Vn_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMGT_Vd_Vn_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMLE_Vd_Vn_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCMLT_Vd_Vn_FPIMM0(opcode) => opcode.bits(),
            ASIMDMISC::FCVTAS_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTAS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTAU_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTAU_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTL2_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTL_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTMS_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTMS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTMU_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTMU_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTN2_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTNS_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTNS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTNU_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTNU_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTN_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTPS_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTPS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTPU_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTPU_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTXN2_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTXN_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTZS_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTZS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FCVTZU_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FCVTZU_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::NEG_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::SCVTF_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::UADDLP_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::UCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::UCVTF_Vd_Vn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDPERM {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDPERM::TRN1_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDPERM::TRN2_Vd_Vn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDPERM::TRN1_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDPERM::TRN2_Vd_Vn_Vm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDSAME {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDSAME::ADDP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::ADD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::AND_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::BIC_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::BIF_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::BIT_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMEQ_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMGE_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMGT_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMHI_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMHS_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMTST_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::EOR_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FCADD_Vd_Vn_Vm_IMM_ROT3(opcode) => opcode.definition(),
            ASIMDSAME::FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FCMEQ_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FCMGE_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FCMGT_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FCMLA_Vd_Vn_Vm_IMM_ROT1(opcode) => opcode.definition(),
            ASIMDSAME::FDIV_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FDIV_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMULX_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMUL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::MUL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::ORN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::ORR_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SUB_Vd_Vn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDSAME::ADDP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::ADD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::AND_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::BIC_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::BIF_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::BIT_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMEQ_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMGE_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMGT_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMHI_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMHS_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMTST_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::EOR_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FCADD_Vd_Vn_Vm_IMM_ROT3(opcode) => opcode.bits(),
            ASIMDSAME::FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FCMEQ_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FCMGE_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FCMGT_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FCMLA_Vd_Vn_Vm_IMM_ROT1(opcode) => opcode.bits(),
            ASIMDSAME::FDIV_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FDIV_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMULX_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMUL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::MUL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::ORN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::ORR_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SUB_Vd_Vn_Vm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDSHF {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDSHF::FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.definition(),
            ASIMDSHF::FCVTZS_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.definition(),
            ASIMDSHF::FCVTZU_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.definition(),
            ASIMDSHF::SCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.definition(),
            ASIMDSHF::UCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDSHF::FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::FCVTZS_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::FCVTZU_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::SCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::UCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDTBL {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDTBL::TBL_Vd_LVn_Vm(opcode) => opcode.definition(),
            ASIMDTBL::TBX_Vd_LVn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDTBL::TBL_Vd_LVn_Vm(opcode) => opcode.bits(),
            ASIMDTBL::TBX_Vd_LVn_Vm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDELEM {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDELEM::FMULX_Sd_Sn_Em(opcode) => opcode.definition(),
            ASISDELEM::FMULX_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::FMUL_Sd_Sn_Em(opcode) => opcode.definition(),
            ASISDELEM::FMUL_Sd_Sn_Em16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDELEM::FMULX_Sd_Sn_Em(opcode) => opcode.bits(),
            ASISDELEM::FMULX_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::FMUL_Sd_Sn_Em(opcode) => opcode.bits(),
            ASISDELEM::FMUL_Sd_Sn_Em16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDMISC {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDMISC::CMEQ_Sd_Sn_IMM0(opcode) => opcode.definition(),
            ASISDMISC::CMGE_Sd_Sn_IMM0(opcode) => opcode.definition(),
            ASISDMISC::CMGT_Sd_Sn_IMM0(opcode) => opcode.definition(),
            ASISDMISC::CMLE_Sd_Sn_IMM0(opcode) => opcode.definition(),
            ASISDMISC::CMLT_Sd_Sn_IMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMEQ_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMEQ_Sd_Sn_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMGE_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMGE_Sd_Sn_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMGT_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMGT_Sd_Sn_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMLE_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMLE_Sd_Sn_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMLT_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCMLT_Sd_Sn_FPIMM0(opcode) => opcode.definition(),
            ASISDMISC::FCVTAS_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTAS_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTAU_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTAU_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTMS_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTMS_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTMU_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTMU_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTNS_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTNS_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTNU_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTNU_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTPS_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTPS_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTPU_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTPU_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTXN_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTZS_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTZS_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FCVTZU_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FCVTZU_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::NEG_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::SCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::SCVTF_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::UCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::UCVTF_Sd_Sn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDMISC::CMEQ_Sd_Sn_IMM0(opcode) => opcode.bits(),
            ASISDMISC::CMGE_Sd_Sn_IMM0(opcode) => opcode.bits(),
            ASISDMISC::CMGT_Sd_Sn_IMM0(opcode) => opcode.bits(),
            ASISDMISC::CMLE_Sd_Sn_IMM0(opcode) => opcode.bits(),
            ASISDMISC::CMLT_Sd_Sn_IMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMEQ_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMEQ_Sd_Sn_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMGE_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMGE_Sd_Sn_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMGT_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMGT_Sd_Sn_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMLE_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMLE_Sd_Sn_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMLT_Sd_S_H_Sn_S_H_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCMLT_Sd_Sn_FPIMM0(opcode) => opcode.bits(),
            ASISDMISC::FCVTAS_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTAS_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTAU_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTAU_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTMS_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTMS_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTMU_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTMU_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTNS_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTNS_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTNU_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTNU_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTPS_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTPS_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTPU_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTPU_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTXN_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTZS_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTZS_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FCVTZU_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FCVTZU_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::NEG_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::SCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::SCVTF_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::UCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::UCVTF_Sd_Sn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDPAIR {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDPAIR::ADDP_Sd_Vn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDPAIR::ADDP_Sd_Vn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDSAME {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDSAME::ADD_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::CMEQ_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::CMGE_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::CMGT_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::CMHI_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::CMHS_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::CMTST_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FCMEQ_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FCMGE_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FCMGE_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FCMGT_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FCMGT_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FMULX_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FMULX_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SUB_Sd_Sn_Sm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDSAME::ADD_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::CMEQ_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::CMGE_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::CMGT_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::CMHI_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::CMHS_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::CMTST_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FCMEQ_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FCMGE_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FCMGE_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FCMGT_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FCMGT_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FMULX_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FMULX_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SUB_Sd_Sn_Sm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDSHF {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDSHF::FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.definition(),
            ASISDSHF::FCVTZS_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.definition(),
            ASISDSHF::FCVTZU_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.definition(),
            ASISDSHF::SCVTF_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.definition(),
            ASISDSHF::UCVTF_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDSHF::FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.bits(),
            ASISDSHF::FCVTZS_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.bits(),
            ASISDSHF::FCVTZU_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.bits(),
            ASISDSHF::SCVTF_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.bits(),
            ASISDSHF::UCVTF_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for BFLOAT16 {
    fn definition(&self) -> &'static Insn {
        match self {
            BFLOAT16::BFCVTN2_Vd_Vn(opcode) => opcode.definition(),
            BFLOAT16::BFCVTN_Vd_Vn(opcode) => opcode.definition(),
            BFLOAT16::BFCVT_Fd_Fn(opcode) => opcode.definition(),
            BFLOAT16::BFDOT_Vd_Vn_Vm(opcode) => opcode.definition(),
            BFLOAT16::BFMLALB_Vd_Vn_Em16(opcode) => opcode.definition(),
            BFLOAT16::BFMLALB_Vd_Vn_Vm(opcode) => opcode.definition(),
            BFLOAT16::BFMLALT_Vd_Vn_Em16(opcode) => opcode.definition(),
            BFLOAT16::BFMLALT_Vd_Vn_Vm(opcode) => opcode.definition(),
            BFLOAT16::BFMMLA_Vd_Vn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            BFLOAT16::BFCVTN2_Vd_Vn(opcode) => opcode.bits(),
            BFLOAT16::BFCVTN_Vd_Vn(opcode) => opcode.bits(),
            BFLOAT16::BFCVT_Fd_Fn(opcode) => opcode.bits(),
            BFLOAT16::BFDOT_Vd_Vn_Vm(opcode) => opcode.bits(),
            BFLOAT16::BFMLALB_Vd_Vn_Em16(opcode) => opcode.bits(),
            BFLOAT16::BFMLALB_Vd_Vn_Vm(opcode) => opcode.bits(),
            BFLOAT16::BFMLALT_Vd_Vn_Em16(opcode) => opcode.bits(),
            BFLOAT16::BFMLALT_Vd_Vn_Vm(opcode) => opcode.bits(),
            BFLOAT16::BFMMLA_Vd_Vn_Vm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for BITFIELD {
    fn definition(&self) -> &'static Insn {
        match self {
            BITFIELD::BFM_Rd_Rn_IMMR_IMMS(opcode) => opcode.definition(),
            BITFIELD::SBFM_Rd_Rn_IMMR_IMMS(opcode) => opcode.definition(),
            BITFIELD::UBFM_Rd_Rn_IMMR_IMMS(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            BITFIELD::BFM_Rd_Rn_IMMR_IMMS(opcode) => opcode.bits(),
            BITFIELD::SBFM_Rd_Rn_IMMR_IMMS(opcode) => opcode.bits(),
            BITFIELD::UBFM_Rd_Rn_IMMR_IMMS(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for BRANCH_IMM {
    fn definition(&self) -> &'static Insn {
        match self {
            BRANCH_IMM::BL_ADDR_PCREL26(opcode) => opcode.definition(),
            BRANCH_IMM::B_ADDR_PCREL26(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            BRANCH_IMM::BL_ADDR_PCREL26(opcode) => opcode.bits(),
            BRANCH_IMM::B_ADDR_PCREL26(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for BRANCH_REG {
    fn definition(&self) -> &'static Insn {
        match self {
            BRANCH_REG::BLRAAZ_Rn(opcode) => opcode.definition(),
            BRANCH_REG::BLRAA_Rn_Rd_SP(opcode) => opcode.definition(),
            BRANCH_REG::BLRABZ_Rn(opcode) => opcode.definition(),
            BRANCH_REG::BLRAB_Rn_Rd_SP(opcode) => opcode.definition(),
            BRANCH_REG::BLR_Rn(opcode) => opcode.definition(),
            BRANCH_REG::BRAAZ_Rn(opcode) => opcode.definition(),
            BRANCH_REG::BRAA_Rn_Rd_SP(opcode) => opcode.definition(),
            BRANCH_REG::BRABZ_Rn(opcode) => opcode.definition(),
            BRANCH_REG::BRAB_Rn_Rd_SP(opcode) => opcode.definition(),
            BRANCH_REG::BR_Rn(opcode) => opcode.definition(),
            BRANCH_REG::RETAA(opcode) => opcode.definition(),
            BRANCH_REG::RETAB(opcode) => opcode.definition(),
            BRANCH_REG::RET_Rn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            BRANCH_REG::BLRAAZ_Rn(opcode) => opcode.bits(),
            BRANCH_REG::BLRAA_Rn_Rd_SP(opcode) => opcode.bits(),
            BRANCH_REG::BLRABZ_Rn(opcode) => opcode.bits(),
            BRANCH_REG::BLRAB_Rn_Rd_SP(opcode) => opcode.bits(),
            BRANCH_REG::BLR_Rn(opcode) => opcode.bits(),
            BRANCH_REG::BRAAZ_Rn(opcode) => opcode.bits(),
            BRANCH_REG::BRAA_Rn_Rd_SP(opcode) => opcode.bits(),
            BRANCH_REG::BRABZ_Rn(opcode) => opcode.bits(),
            BRANCH_REG::BRAB_Rn_Rd_SP(opcode) => opcode.bits(),
            BRANCH_REG::BR_Rn(opcode) => opcode.bits(),
            BRANCH_REG::RETAA(opcode) => opcode.bits(),
            BRANCH_REG::RETAB(opcode) => opcode.bits(),
            BRANCH_REG::RET_Rn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for COMPBRANCH {
    fn definition(&self) -> &'static Insn {
        match self {
            COMPBRANCH::CBNZ_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
            COMPBRANCH::CBZ_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            COMPBRANCH::CBNZ_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
            COMPBRANCH::CBZ_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CONDCMP_IMM {
    fn definition(&self) -> &'static Insn {
        match self {
            CONDCMP_IMM::CCMN_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.definition(),
            CONDCMP_IMM::CCMP_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CONDCMP_IMM::CCMN_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.bits(),
            CONDCMP_IMM::CCMP_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CONDCMP_REG {
    fn definition(&self) -> &'static Insn {
        match self {
            CONDCMP_REG::CCMN_Rn_Rm_NZCV_COND(opcode) => opcode.definition(),
            CONDCMP_REG::CCMP_Rn_Rm_NZCV_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CONDCMP_REG::CCMN_Rn_Rm_NZCV_COND(opcode) => opcode.bits(),
            CONDCMP_REG::CCMP_Rn_Rm_NZCV_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CONDSEL {
    fn definition(&self) -> &'static Insn {
        match self {
            CONDSEL::CSEL_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
            CONDSEL::CSINC_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
            CONDSEL::CSINV_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
            CONDSEL::CSNEG_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CONDSEL::CSEL_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
            CONDSEL::CSINC_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
            CONDSEL::CSINV_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
            CONDSEL::CSNEG_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CRYPTOSHA3 {
    fn definition(&self) -> &'static Insn {
        match self {
            CRYPTOSHA3::EOR3_Vd_Vn_Vm_Va(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CRYPTOSHA3::EOR3_Vd_Vn_Vm_Va(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CSSC {
    fn definition(&self) -> &'static Insn {
        match self {
            CSSC::CNT_Rd_Rn(opcode) => opcode.definition(),
            CSSC::CTZ_Rd_Rn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CSSC::CNT_Rd_Rn(opcode) => opcode.bits(),
            CSSC::CTZ_Rd_Rn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for DOTPRODUCT {
    fn definition(&self) -> &'static Insn {
        match self {
            DOTPRODUCT::BFDOT_Vd_Vn_Em(opcode) => opcode.definition(),
            DOTPRODUCT::SDOT_Vd_Vn_Em(opcode) => opcode.definition(),
            DOTPRODUCT::SDOT_Vd_Vn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            DOTPRODUCT::BFDOT_Vd_Vn_Em(opcode) => opcode.bits(),
            DOTPRODUCT::SDOT_Vd_Vn_Em(opcode) => opcode.bits(),
            DOTPRODUCT::SDOT_Vd_Vn_Vm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for DP_1SRC {
    fn definition(&self) -> &'static Insn {
        match self {
            DP_1SRC::CLS_Rd_Rn(opcode) => opcode.definition(),
            DP_1SRC::CLZ_Rd_Rn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            DP_1SRC::CLS_Rd_Rn(opcode) => opcode.bits(),
            DP_1SRC::CLZ_Rd_Rn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for DP_2SRC {
    fn definition(&self) -> &'static Insn {
        match self {
            DP_2SRC::ASRV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::LSLV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::LSRV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::SDIV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::SUBPS_Rd_Rn_SP_Rm_SP(opcode) => opcode.definition(),
            DP_2SRC::SUBP_Rd_Rn_SP_Rm_SP(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            DP_2SRC::ASRV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::LSLV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::LSRV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::SDIV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::SUBPS_Rd_Rn_SP_Rm_SP(opcode) => opcode.bits(),
            DP_2SRC::SUBP_Rd_Rn_SP_Rm_SP(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for DP_3SRC {
    fn definition(&self) -> &'static Insn {
        match self {
            DP_3SRC::MADD_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::MSUB_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::SMADDL_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::SMULH_Rd_Rn_Rm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            DP_3SRC::MADD_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::MSUB_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::SMADDL_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::SMULH_Rd_Rn_Rm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for EXCEPTION {
    fn definition(&self) -> &'static Insn {
        match self {
            EXCEPTION::BRK_EXCEPTION(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            EXCEPTION::BRK_EXCEPTION(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOAT2FIX {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOAT2FIX::FCVTZS_Rd_Fn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.definition(),
            FLOAT2FIX::FCVTZU_Rd_Fn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.definition(),
            FLOAT2FIX::SCVTF_Fd_Rn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.definition(),
            FLOAT2FIX::UCVTF_Fd_Rn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOAT2FIX::FCVTZS_Rd_Fn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.bits(),
            FLOAT2FIX::FCVTZU_Rd_Fn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.bits(),
            FLOAT2FIX::SCVTF_Fd_Rn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.bits(),
            FLOAT2FIX::UCVTF_Fd_Rn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOAT2INT {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOAT2INT::FCVTAS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTAS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTAU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTAU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FJCVTZS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Fd_Rn(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Fd_S_S_Rn_W(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Rd_VnD1(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Rd_W_Fn_S_S(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_VdD1_Rn(opcode) => opcode.definition(),
            FLOAT2INT::SCVTF_Fd_Rn(opcode) => opcode.definition(),
            FLOAT2INT::SCVTF_Fd_S_D_Rn_W(opcode) => opcode.definition(),
            FLOAT2INT::UCVTF_Fd_Rn(opcode) => opcode.definition(),
            FLOAT2INT::UCVTF_Fd_S_D_Rn_W(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOAT2INT::FCVTAS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTAS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTAU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTAU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FJCVTZS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Fd_Rn(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Fd_S_S_Rn_W(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Rd_VnD1(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Rd_W_Fn_S_S(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_VdD1_Rn(opcode) => opcode.bits(),
            FLOAT2INT::SCVTF_Fd_Rn(opcode) => opcode.bits(),
            FLOAT2INT::SCVTF_Fd_S_D_Rn_W(opcode) => opcode.bits(),
            FLOAT2INT::UCVTF_Fd_Rn(opcode) => opcode.bits(),
            FLOAT2INT::UCVTF_Fd_S_D_Rn_W(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATCCMP {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATCCMP::FCCMPE_Fn_Fm_NZCV_COND(opcode) => opcode.definition(),
            FLOATCCMP::FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.definition(),
            FLOATCCMP::FCCMP_Fn_Fm_NZCV_COND(opcode) => opcode.definition(),
            FLOATCCMP::FCCMP_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATCCMP::FCCMPE_Fn_Fm_NZCV_COND(opcode) => opcode.bits(),
            FLOATCCMP::FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.bits(),
            FLOATCCMP::FCCMP_Fn_Fm_NZCV_COND(opcode) => opcode.bits(),
            FLOATCCMP::FCCMP_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATCMP {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATCMP::FCMPE_Fn_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMPE_Fn_Fm(opcode) => opcode.definition(),
            FLOATCMP::FCMPE_Fn_S_S_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMPE_Fn_S_S_Fm_S_S(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_Fm(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_S_S_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_S_S_Fm_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATCMP::FCMPE_Fn_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMPE_Fn_Fm(opcode) => opcode.bits(),
            FLOATCMP::FCMPE_Fn_S_S_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMPE_Fn_S_S_Fm_S_S(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_Fm(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_S_S_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_S_S_Fm_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATDP1 {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATDP1::FCVT_Fd_Fn(opcode) => opcode.definition(),
            FLOATDP1::FMOV_Fd_Fn(opcode) => opcode.definition(),
            FLOATDP1::FMOV_Fd_S_S_Fn_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATDP1::FCVT_Fd_Fn(opcode) => opcode.bits(),
            FLOATDP1::FMOV_Fd_Fn(opcode) => opcode.bits(),
            FLOATDP1::FMOV_Fd_S_S_Fn_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATDP2 {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATDP2::FDIV_Fd_Fn_Fm(opcode) => opcode.definition(),
            FLOATDP2::FDIV_Fd_S_S_Fn_S_S_Fm_S_S(opcode) => opcode.definition(),
            FLOATDP2::FMUL_Fd_Fn_Fm(opcode) => opcode.definition(),
            FLOATDP2::FMUL_Fd_S_S_Fn_S_S_Fm_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATDP2::FDIV_Fd_Fn_Fm(opcode) => opcode.bits(),
            FLOATDP2::FDIV_Fd_S_S_Fn_S_S_Fm_S_S(opcode) => opcode.bits(),
            FLOATDP2::FMUL_Fd_Fn_Fm(opcode) => opcode.bits(),
            FLOATDP2::FMUL_Fd_S_S_Fn_S_S_Fm_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATDP3 {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATDP3::FMSUB_Fd_Fn_Fm_Fa(opcode) => opcode.definition(),
            FLOATDP3::FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATDP3::FMSUB_Fd_Fn_Fm_Fa(opcode) => opcode.bits(),
            FLOATDP3::FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATIMM {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATIMM::FMOV_Fd_FPIMM(opcode) => opcode.definition(),
            FLOATIMM::FMOV_Fd_S_S_FPIMM(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATIMM::FMOV_Fd_FPIMM(opcode) => opcode.bits(),
            FLOATIMM::FMOV_Fd_S_S_FPIMM(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATSEL {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATSEL::FCSEL_Fd_Fn_Fm_COND(opcode) => opcode.definition(),
            FLOATSEL::FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATSEL::FCSEL_Fd_Fn_Fm_COND(opcode) => opcode.bits(),
            FLOATSEL::FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for IC_SYSTEM {
    fn definition(&self) -> &'static Insn {
        match self {
            IC_SYSTEM::CFINV(opcode) => opcode.definition(),
            IC_SYSTEM::CHKFEAT_X16(opcode) => opcode.definition(),
            IC_SYSTEM::CLREX_UIMM4(opcode) => opcode.definition(),
            IC_SYSTEM::SB(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            IC_SYSTEM::CFINV(opcode) => opcode.bits(),
            IC_SYSTEM::CHKFEAT_X16(opcode) => opcode.bits(),
            IC_SYSTEM::CLREX_UIMM4(opcode) => opcode.bits(),
            IC_SYSTEM::SB(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTEXCL {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTEXCL::STXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STZGM_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTEXCL::STXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STZGM_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTPAIR_INDEXED {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTPAIR_OFF {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_IMM10 {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(opcode) => opcode.definition(),
            LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(opcode) => opcode.bits(),
            LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_IMM9 {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_IMM9::LDRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
            LDST_IMM9::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_IMM9::LDRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
            LDST_IMM9::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_POS {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_POS::LDRB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSW_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDR_Ft_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDR_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STRB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STRH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STR_Ft_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STR_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_POS::LDRB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSW_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDR_Ft_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDR_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STRB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STRH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STR_Ft_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STR_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_REGOFF {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDR_Ft_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDR_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STRB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STRH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STR_Ft_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STR_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDR_Ft_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDR_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STRB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STRH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STR_Ft_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STR_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_UNPRIV {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_UNSCALED {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOADLIT {
    fn definition(&self) -> &'static Insn {
        match self {
            LOADLIT::LDRSW_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
            LOADLIT::LDR_Ft_ADDR_PCREL19(opcode) => opcode.definition(),
            LOADLIT::LDR_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOADLIT::LDRSW_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
            LOADLIT::LDR_Ft_ADDR_PCREL19(opcode) => opcode.bits(),
            LOADLIT::LDR_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOG_IMM {
    fn definition(&self) -> &'static Insn {
        match self {
            LOG_IMM::ANDS_Rd_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::AND_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::EOR_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::ORR_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOG_IMM::ANDS_Rd_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::AND_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::EOR_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::ORR_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOG_SHIFT {
    fn definition(&self) -> &'static Insn {
        match self {
            LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::AND_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::EOR_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::ORR_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::AND_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::EOR_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::ORR_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for MOVEWIDE {
    fn definition(&self) -> &'static Insn {
        match self {
            MOVEWIDE::MOVK_Rd_HALF(opcode) => opcode.definition(),
            MOVEWIDE::MOVN_Rd_HALF(opcode) => opcode.definition(),
            MOVEWIDE::MOVZ_Rd_HALF(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            MOVEWIDE::MOVK_Rd_HALF(opcode) => opcode.bits(),
            MOVEWIDE::MOVN_Rd_HALF(opcode) => opcode.bits(),
            MOVEWIDE::MOVZ_Rd_HALF(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for PCRELADDR {
    fn definition(&self) -> &'static Insn {
        match self {
            PCRELADDR::ADRP_Rd_ADDR_ADRP(opcode) => opcode.definition(),
            PCRELADDR::ADR_Rd_ADDR_PCREL21(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            PCRELADDR::ADRP_Rd_ADDR_ADRP(opcode) => opcode.bits(),
            PCRELADDR::ADR_Rd_ADDR_PCREL21(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME2_MOV {
    fn definition(&self) -> &'static Insn {
        match self {
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME2_MOV::MOVA_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.definition(),
            SME2_MOV::MOVA_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME2_MOV::MOVA_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.bits(),
            SME2_MOV::MOVA_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME2_MOVAZ {
    fn definition(&self) -> &'static Insn {
        match self {
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_INT_SD {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.definition()
            }
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B(opcode) => {
                opcode.definition()
            }
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.definition()
            }
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.bits()
            }
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B(opcode) => {
                opcode.bits()
            }
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.bits()
            }
        }
    }
}
impl InsnOpcode for SME_LDR {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_LDR::LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_LDR::LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_MISC {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_MISC::ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.definition(),
            SME_MISC::ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.definition(),
            SME_MISC::ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::BFCVTN_SVE_Zd_SME_Znx2(opcode) => opcode.definition(),
            SME_MISC::BFCVT_SVE_Zd_SME_Znx2(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.definition(),
            SME_MISC::FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.definition(),
            SME_MISC::FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::LDR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            SME_MISC::MOVT_Rt_SME_ZT0_INDEX(opcode) => opcode.definition(),
            SME_MISC::MOVT_SME_ZT0_INDEX_Rt(opcode) => opcode.definition(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408(opcode) => {
                opcode.definition()
            }
            SME_MISC::STR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_MISC::ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.bits(),
            SME_MISC::ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.bits(),
            SME_MISC::ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::BFCVTN_SVE_Zd_SME_Znx2(opcode) => opcode.bits(),
            SME_MISC::BFCVT_SVE_Zd_SME_Znx2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => opcode.bits(),
            SME_MISC::BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => opcode.bits(),
            SME_MISC::FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.bits(),
            SME_MISC::FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.bits(),
            SME_MISC::FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::LDR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            SME_MISC::MOVT_Rt_SME_ZT0_INDEX(opcode) => opcode.bits(),
            SME_MISC::MOVT_SME_ZT0_INDEX_Rt(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408(opcode) => {
                opcode.bits()
            }
            SME_MISC::STR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
