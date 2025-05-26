#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for UZP1_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UZP2_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "uzp2",
        aliases: &[],
        opcode: 0xe005800,
        mask: 0xbf20fc00,
        class: InsnClass::ASIMDPERM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#uzp2,
            operation: Operation::ASIMDPERM(ASIMDPERM::UZP2_Vd_Vn_Vm(UZP2_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for UZP2_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl XPACD_Rd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "xpacd",
        aliases: &[],
        opcode: 0xdac147e0,
        mask: 0xffffffe0,
        class: InsnClass::DP_1SRC,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#xpacd,
            operation: Operation::DP_1SRC(DP_1SRC::XPACD_Rd(XPACD_Rd::from(bits))),
        }
    }
}
impl InsnOpcode for XPACD_Rd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl XPACI_Rd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "xpaci",
        aliases: &[],
        opcode: 0xdac143e0,
        mask: 0xffffffe0,
        class: InsnClass::DP_1SRC,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#xpaci,
            operation: Operation::DP_1SRC(DP_1SRC::XPACI_Rd(XPACI_Rd::from(bits))),
        }
    }
}
impl InsnOpcode for XPACI_Rd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl XTN_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "xtn",
        aliases: &[],
        opcode: 0xe212800,
        mask: 0xff3ffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_2S,
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
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
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
            mnemonic: Mnemonic::r#xtn,
            operation: Operation::ASIMDMISC(ASIMDMISC::XTN_Vd_Vn(XTN_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for XTN_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl XTN2_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "xtn2",
        aliases: &[],
        opcode: 0x4e212800,
        mask: 0xff3ffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_4S,
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
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
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
            mnemonic: Mnemonic::r#xtn2,
            operation: Operation::ASIMDMISC(ASIMDMISC::XTN2_Vd_Vn(XTN2_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for XTN2_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ZIP1_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "zip1",
        aliases: &[],
        opcode: 0xe003800,
        mask: 0xbf20fc00,
        class: InsnClass::ASIMDPERM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#zip1,
            operation: Operation::ASIMDPERM(ASIMDPERM::ZIP1_Vd_Vn_Vm(ZIP1_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for ZIP1_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ZIP2_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "zip2",
        aliases: &[],
        opcode: 0xe007800,
        mask: 0xbf20fc00,
        class: InsnClass::ASIMDPERM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#zip2,
            operation: Operation::ASIMDPERM(ASIMDPERM::ZIP2_Vd_Vn_Vm(ZIP2_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for ZIP2_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl InsnOpcode for ADDSUB_CARRY {
    fn definition(&self) -> &'static Insn {
        match self {
            ADDSUB_CARRY::ADCS_Rd_Rn_Rm(opcode) => opcode.definition(),
            ADDSUB_CARRY::ADC_Rd_Rn_Rm(opcode) => opcode.definition(),
            ADDSUB_CARRY::SBCS_Rd_Rn_Rm(opcode) => opcode.definition(),
            ADDSUB_CARRY::SBC_Rd_Rn_Rm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ADDSUB_CARRY::ADCS_Rd_Rn_Rm(opcode) => opcode.bits(),
            ADDSUB_CARRY::ADC_Rd_Rn_Rm(opcode) => opcode.bits(),
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
            ASIMDALL::FMAXNMV_Fd_S_S_Vn_V_4S(opcode) => opcode.definition(),
            ASIMDALL::FMAXNMV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::FMAXV_Fd_S_S_Vn_V_4S(opcode) => opcode.definition(),
            ASIMDALL::FMAXV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::FMINNMV_Fd_S_S_Vn_V_4S(opcode) => opcode.definition(),
            ASIMDALL::FMINNMV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::FMINV_Fd_S_S_Vn_V_4S(opcode) => opcode.definition(),
            ASIMDALL::FMINV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::SADDLV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::SMAXV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::SMINV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::UADDLV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::UMAXV_Fd_Vn(opcode) => opcode.definition(),
            ASIMDALL::UMINV_Fd_Vn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDALL::ADDV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::FMAXNMV_Fd_S_S_Vn_V_4S(opcode) => opcode.bits(),
            ASIMDALL::FMAXNMV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::FMAXV_Fd_S_S_Vn_V_4S(opcode) => opcode.bits(),
            ASIMDALL::FMAXV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::FMINNMV_Fd_S_S_Vn_V_4S(opcode) => opcode.bits(),
            ASIMDALL::FMINNMV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::FMINV_Fd_S_S_Vn_V_4S(opcode) => opcode.bits(),
            ASIMDALL::FMINV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::SADDLV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::SMAXV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::SMINV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::UADDLV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::UMAXV_Fd_Vn(opcode) => opcode.bits(),
            ASIMDALL::UMINV_Fd_Vn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDDIFF {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDDIFF::ADDHN2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::ADDHN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::PMULL2_Vd_V_1Q_Vn_V_2D_Vm_V_2D(opcode) => opcode.definition(),
            ASIMDDIFF::PMULL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::PMULL_Vd_V_1Q_Vn_V_1D_Vm_V_1D(opcode) => opcode.definition(),
            ASIMDDIFF::PMULL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::RADDHN2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::RADDHN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::RSUBHN2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::RSUBHN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SABAL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SABAL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SABDL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SABDL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SADDL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SADDL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SADDW2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SADDW_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMLAL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMLAL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMLSL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMLSL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMULL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SMULL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SQDMLAL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SQDMLAL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SQDMLSL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SQDMLSL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SQDMULL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SQDMULL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SSUBL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SSUBL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SSUBW2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SSUBW_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SUBHN2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::SUBHN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UABAL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UABAL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UABDL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UABDL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDW2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UADDW_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UMLAL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UMLAL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UMLSL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UMLSL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UMULL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::UMULL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::USUBL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::USUBL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::USUBW2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDDIFF::USUBW_Vd_Vn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDDIFF::ADDHN2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::ADDHN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::PMULL2_Vd_V_1Q_Vn_V_2D_Vm_V_2D(opcode) => opcode.bits(),
            ASIMDDIFF::PMULL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::PMULL_Vd_V_1Q_Vn_V_1D_Vm_V_1D(opcode) => opcode.bits(),
            ASIMDDIFF::PMULL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::RADDHN2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::RADDHN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::RSUBHN2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::RSUBHN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SABAL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SABAL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SABDL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SABDL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SADDL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SADDL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SADDW2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SADDW_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMLAL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMLAL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMLSL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMLSL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMULL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SMULL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SQDMLAL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SQDMLAL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SQDMLSL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SQDMLSL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SQDMULL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SQDMULL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SSUBL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SSUBL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SSUBW2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SSUBW_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SUBHN2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::SUBHN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UABAL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UABAL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UABDL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UABDL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDW2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UADDW_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UMLAL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UMLAL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UMLSL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UMLSL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UMULL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::UMULL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::USUBL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::USUBL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::USUBW2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDDIFF::USUBW_Vd_Vn_Vm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDELEM {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDELEM::FCMLA_Vd_Vn_Em_IMM_ROT2(opcode) => opcode.definition(),
            ASIMDELEM::FMLAL2_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.definition(),
            ASIMDELEM::FMLAL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMLAL_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.definition(),
            ASIMDELEM::FMLAL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMLA_Vd_Vn_Em(opcode) => opcode.definition(),
            ASIMDELEM::FMLA_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMLSL2_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.definition(),
            ASIMDELEM::FMLSL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMLSL_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.definition(),
            ASIMDELEM::FMLSL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMLS_Vd_Vn_Em(opcode) => opcode.definition(),
            ASIMDELEM::FMLS_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMULX_Vd_Vn_Em(opcode) => opcode.definition(),
            ASIMDELEM::FMULX_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::FMUL_Vd_Vn_Em(opcode) => opcode.definition(),
            ASIMDELEM::FMUL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::MLA_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::MLS_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::MUL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMLAL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMLAL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMLSL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMLSL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMULL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SMULL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQDMLAL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQDMLAL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQDMLSL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQDMLSL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQDMULH_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQDMULL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQDMULL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQRDMLAH_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQRDMLSH_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::SQRDMULH_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::UMLAL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::UMLAL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::UMLSL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::UMLSL_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::UMULL2_Vd_Vn_Em16(opcode) => opcode.definition(),
            ASIMDELEM::UMULL_Vd_Vn_Em16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDELEM::FCMLA_Vd_Vn_Em_IMM_ROT2(opcode) => opcode.bits(),
            ASIMDELEM::FMLAL2_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.bits(),
            ASIMDELEM::FMLAL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMLAL_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.bits(),
            ASIMDELEM::FMLAL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMLA_Vd_Vn_Em(opcode) => opcode.bits(),
            ASIMDELEM::FMLA_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMLSL2_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.bits(),
            ASIMDELEM::FMLSL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMLSL_Vd_V_4S_Vn_V_4H_Em16_S_H(opcode) => opcode.bits(),
            ASIMDELEM::FMLSL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMLS_Vd_Vn_Em(opcode) => opcode.bits(),
            ASIMDELEM::FMLS_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMULX_Vd_Vn_Em(opcode) => opcode.bits(),
            ASIMDELEM::FMULX_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::FMUL_Vd_Vn_Em(opcode) => opcode.bits(),
            ASIMDELEM::FMUL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::MLA_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::MLS_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::MUL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMLAL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMLAL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMLSL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMLSL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMULL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SMULL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQDMLAL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQDMLAL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQDMLSL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQDMLSL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQDMULH_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQDMULL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQDMULL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQRDMLAH_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQRDMLSH_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::SQRDMULH_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::UMLAL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::UMLAL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::UMLSL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::UMLSL_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::UMULL2_Vd_Vn_Em16(opcode) => opcode.bits(),
            ASIMDELEM::UMULL_Vd_Vn_Em16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDEXT {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDEXT::EXT_Vd_Vn_Vm_IDX(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDEXT::EXT_Vd_Vn_Vm_IDX(opcode) => opcode.bits(),
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
impl InsnOpcode for ASIMDINS {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDINS::DUP_Vd_En(opcode) => opcode.definition(),
            ASIMDINS::DUP_Vd_Rn(opcode) => opcode.definition(),
            ASIMDINS::INS_Ed_En(opcode) => opcode.definition(),
            ASIMDINS::INS_Ed_Rn(opcode) => opcode.definition(),
            ASIMDINS::SMOV_Rd_En(opcode) => opcode.definition(),
            ASIMDINS::UMOV_Rd_En(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDINS::DUP_Vd_En(opcode) => opcode.bits(),
            ASIMDINS::DUP_Vd_Rn(opcode) => opcode.bits(),
            ASIMDINS::INS_Ed_En(opcode) => opcode.bits(),
            ASIMDINS::INS_Ed_Rn(opcode) => opcode.bits(),
            ASIMDINS::SMOV_Rd_En(opcode) => opcode.bits(),
            ASIMDINS::UMOV_Rd_En(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDMISC {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDMISC::ABS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::CLS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::CLZ_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::CMEQ_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMGE_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMGT_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMLE_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CMLT_Vd_Vn_IMM0(opcode) => opcode.definition(),
            ASIMDMISC::CNT_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FABS_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FABS_Vd_Vn(opcode) => opcode.definition(),
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
            ASIMDMISC::FNEG_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FNEG_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRECPE_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRECPE_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINT32X_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINT32Z_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINT64X_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINT64Z_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINTA_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRINTA_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINTI_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRINTI_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINTM_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRINTM_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINTN_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRINTN_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINTP_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRINTP_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINTX_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRINTX_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRINTZ_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRINTZ_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FRSQRTE_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FRSQRTE_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::FSQRT_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::FSQRT_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::NEG_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::NOT_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::RBIT_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::REV16_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::REV32_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::REV64_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SADALP_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SADDLP_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::SCVTF_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SHLL2_Vd_Vn_SHLL_IMM(opcode) => opcode.definition(),
            ASIMDMISC::SHLL_Vd_Vn_SHLL_IMM(opcode) => opcode.definition(),
            ASIMDMISC::SQABS_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SQNEG_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SQXTN2_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SQXTN_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SQXTUN2_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SQXTUN_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::SUQADD_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::UADALP_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::UADDLP_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::UCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.definition(),
            ASIMDMISC::UCVTF_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::UQXTN2_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::UQXTN_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::URECPE_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::URSQRTE_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::USQADD_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::XTN2_Vd_Vn(opcode) => opcode.definition(),
            ASIMDMISC::XTN_Vd_Vn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDMISC::ABS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::CLS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::CLZ_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::CMEQ_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMGE_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMGT_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMLE_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CMLT_Vd_Vn_IMM0(opcode) => opcode.bits(),
            ASIMDMISC::CNT_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FABS_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FABS_Vd_Vn(opcode) => opcode.bits(),
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
            ASIMDMISC::FNEG_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FNEG_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRECPE_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRECPE_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINT32X_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINT32Z_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINT64X_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINT64Z_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINTA_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRINTA_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINTI_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRINTI_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINTM_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRINTM_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINTN_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRINTN_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINTP_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRINTP_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINTX_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRINTX_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRINTZ_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRINTZ_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FRSQRTE_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FRSQRTE_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::FSQRT_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::FSQRT_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::NEG_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::NOT_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::RBIT_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::REV16_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::REV32_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::REV64_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SADALP_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SADDLP_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::SCVTF_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SHLL2_Vd_Vn_SHLL_IMM(opcode) => opcode.bits(),
            ASIMDMISC::SHLL_Vd_Vn_SHLL_IMM(opcode) => opcode.bits(),
            ASIMDMISC::SQABS_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SQNEG_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SQXTN2_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SQXTN_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SQXTUN2_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SQXTUN_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::SUQADD_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::UADALP_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::UADDLP_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::UCVTF_Vd_V_4H_Vn_V_4H(opcode) => opcode.bits(),
            ASIMDMISC::UCVTF_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::UQXTN2_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::UQXTN_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::URECPE_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::URSQRTE_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::USQADD_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::XTN2_Vd_Vn(opcode) => opcode.bits(),
            ASIMDMISC::XTN_Vd_Vn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASIMDPERM {
    fn definition(&self) -> &'static Insn {
        match self {
            ASIMDPERM::TRN1_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDPERM::TRN2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDPERM::UZP1_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDPERM::UZP2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDPERM::ZIP1_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDPERM::ZIP2_Vd_Vn_Vm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDPERM::TRN1_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDPERM::TRN2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDPERM::UZP1_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDPERM::UZP2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDPERM::ZIP1_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDPERM::ZIP2_Vd_Vn_Vm(opcode) => opcode.bits(),
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
            ASIMDSAME::BSL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMEQ_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMGE_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMGT_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMHI_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMHS_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::CMTST_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::EOR_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FABD_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FABD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FACGE_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FACGE_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FACGT_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FACGT_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FADDP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FADDP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FADD_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FADD_Vd_Vn_Vm(opcode) => opcode.definition(),
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
            ASIMDSAME::FMAXNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMAXNMP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMAXNM_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMAXNM_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMAXP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMAXP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMAX_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMAX_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMINNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMINNMP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMINNM_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMINNM_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMINP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMINP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMIN_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMIN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMLAL2_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.definition(),
            ASIMDSAME::FMLAL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMLAL_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.definition(),
            ASIMDSAME::FMLAL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMLA_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMLA_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMLSL2_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.definition(),
            ASIMDSAME::FMLSL2_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMLSL_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.definition(),
            ASIMDSAME::FMLSL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMLS_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMLS_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMULX_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FMUL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FRECPS_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FRECPS_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FRSQRTS_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FRSQRTS_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.definition(),
            ASIMDSAME::FSUB_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::MLA_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::MLS_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::MUL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::ORN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::ORR_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::PMUL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SABA_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SABD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SHADD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SHSUB_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SMAXP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SMAX_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SMINP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SMIN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQADD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQDMULH_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQRDMLAH_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASIMDSAME::SQRDMLAH_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQRDMLSH_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASIMDSAME::SQRDMLSH_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQRDMULH_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQRSHL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQSHL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SQSUB_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SRHADD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SRSHL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SSHL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::SUB_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UABA_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UABD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UHADD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UHSUB_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UMAXP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UMAX_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UMINP_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UMIN_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UQADD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UQRSHL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UQSHL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::UQSUB_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::URHADD_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::URSHL_Vd_Vn_Vm(opcode) => opcode.definition(),
            ASIMDSAME::USHL_Vd_Vn_Vm(opcode) => opcode.definition(),
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
            ASIMDSAME::BSL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMEQ_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMGE_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMGT_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMHI_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMHS_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::CMTST_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::EOR_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FABD_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FABD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FACGE_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FACGE_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FACGT_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FACGT_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FADDP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FADDP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FADD_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FADD_Vd_Vn_Vm(opcode) => opcode.bits(),
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
            ASIMDSAME::FMAXNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMAXNMP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMAXNM_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMAXNM_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMAXP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMAXP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMAX_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMAX_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMINNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMINNMP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMINNM_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMINNM_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMINP_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMINP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMIN_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMIN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMLAL2_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.bits(),
            ASIMDSAME::FMLAL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMLAL_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.bits(),
            ASIMDSAME::FMLAL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMLA_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMLA_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMLSL2_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.bits(),
            ASIMDSAME::FMLSL2_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMLSL_Vd_V_4S_Vn_V_4H_Vm_V_4H(opcode) => opcode.bits(),
            ASIMDSAME::FMLSL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMLS_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMLS_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMULX_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FMUL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FRECPS_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FRECPS_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FRSQRTS_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FRSQRTS_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S(opcode) => opcode.bits(),
            ASIMDSAME::FSUB_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::MLA_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::MLS_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::MUL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::ORN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::ORR_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::PMUL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SABA_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SABD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SHADD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SHSUB_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SMAXP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SMAX_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SMINP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SMIN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQADD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQDMULH_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQRDMLAH_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASIMDSAME::SQRDMLAH_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQRDMLSH_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASIMDSAME::SQRDMLSH_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQRDMULH_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQRSHL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQSHL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SQSUB_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SRHADD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SRSHL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SSHL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::SUB_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UABA_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UABD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UHADD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UHSUB_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UMAXP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UMAX_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UMINP_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UMIN_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UQADD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UQRSHL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UQSHL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::UQSUB_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::URHADD_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::URSHL_Vd_Vn_Vm(opcode) => opcode.bits(),
            ASIMDSAME::USHL_Vd_Vn_Vm(opcode) => opcode.bits(),
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
            ASIMDSHF::RSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::RSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.definition(),
            ASIMDSHF::SCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SHL_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::SHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SLI_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::SQRSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SQRSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SQRSHRUN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SQRSHRUN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SQSHLU_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::SQSHL_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::SQSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SQSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SQSHRUN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SQSHRUN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SRI_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SRSHR_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SRSRA_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SSHLL2_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::SSHLL_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::SSHR_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::SSRA_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.definition(),
            ASIMDSHF::UCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::UQRSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::UQRSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::UQSHL_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::UQSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::UQSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::URSHR_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::URSRA_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::USHLL2_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::USHLL_Vd_Vn_IMM_VLSL(opcode) => opcode.definition(),
            ASIMDSHF::USHR_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
            ASIMDSHF::USRA_Vd_Vn_IMM_VLSR(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASIMDSHF::FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::FCVTZS_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::FCVTZU_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::RSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::RSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::SCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SHL_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::SHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SLI_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::SQRSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SQRSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SQRSHRUN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SQRSHRUN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SQSHLU_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::SQSHL_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::SQSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SQSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SQSHRUN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SQSHRUN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SRI_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SRSHR_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SRSRA_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SSHLL2_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::SSHLL_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::SSHR_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::SSRA_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(opcode) => opcode.bits(),
            ASIMDSHF::UCVTF_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::UQRSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::UQRSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::UQSHL_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::UQSHRN2_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::UQSHRN_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::URSHR_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::URSRA_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::USHLL2_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::USHLL_Vd_Vn_IMM_VLSL(opcode) => opcode.bits(),
            ASIMDSHF::USHR_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
            ASIMDSHF::USRA_Vd_Vn_IMM_VLSR(opcode) => opcode.bits(),
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
impl InsnOpcode for ASISDDIFF {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDDIFF::SQDMLAL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDDIFF::SQDMLSL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDDIFF::SQDMULL_Sd_Sn_Sm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDDIFF::SQDMLAL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDDIFF::SQDMLSL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDDIFF::SQDMULL_Sd_Sn_Sm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDELEM {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDELEM::FMLA_Sd_Sn_Em(opcode) => opcode.definition(),
            ASISDELEM::FMLA_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::FMLS_Sd_Sn_Em(opcode) => opcode.definition(),
            ASISDELEM::FMLS_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::FMULX_Sd_Sn_Em(opcode) => opcode.definition(),
            ASISDELEM::FMULX_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::FMUL_Sd_Sn_Em(opcode) => opcode.definition(),
            ASISDELEM::FMUL_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::SQDMLAL_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::SQDMLSL_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::SQDMULH_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::SQDMULL_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::SQRDMLAH_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::SQRDMLSH_Sd_Sn_Em16(opcode) => opcode.definition(),
            ASISDELEM::SQRDMULH_Sd_Sn_Em16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDELEM::FMLA_Sd_Sn_Em(opcode) => opcode.bits(),
            ASISDELEM::FMLA_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::FMLS_Sd_Sn_Em(opcode) => opcode.bits(),
            ASISDELEM::FMLS_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::FMULX_Sd_Sn_Em(opcode) => opcode.bits(),
            ASISDELEM::FMULX_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::FMUL_Sd_Sn_Em(opcode) => opcode.bits(),
            ASISDELEM::FMUL_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::SQDMLAL_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::SQDMLSL_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::SQDMULH_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::SQDMULL_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::SQRDMLAH_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::SQRDMLSH_Sd_Sn_Em16(opcode) => opcode.bits(),
            ASISDELEM::SQRDMULH_Sd_Sn_Em16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDLSE {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDLSE::LD1_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSE::LD2_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSE::LD3_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSE::LD4_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSE::ST1_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSE::ST2_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSE::ST3_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSE::ST4_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDLSE::LD1_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSE::LD2_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSE::LD3_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSE::LD4_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSE::ST1_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSE::ST2_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSE::ST3_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSE::ST4_LVt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDLSEP {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDLSEP::LD1_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSEP::LD2_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSEP::LD3_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSEP::LD4_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSEP::ST1_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSEP::ST2_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSEP::ST3_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSEP::ST4_LVt_SIMD_ADDR_POST(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDLSEP::LD1_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSEP::LD2_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSEP::LD3_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSEP::LD4_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSEP::ST1_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSEP::ST2_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSEP::ST3_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSEP::ST4_LVt_SIMD_ADDR_POST(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDLSO {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDLSO::LD1R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::LD1_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::LD2R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::LD2_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::LD3R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::LD3_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::LD4R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::LD4_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::ST1_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::ST2_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::ST3_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            ASISDLSO::ST4_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDLSO::LD1R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::LD1_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::LD2R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::LD2_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::LD3R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::LD3_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::LD4R_LVt_AL_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::LD4_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::ST1_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::ST2_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::ST3_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            ASISDLSO::ST4_LEt_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDLSOP {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDLSOP::LD1R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::LD1_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::LD2R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::LD2_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::LD3R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::LD3_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::LD4R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::LD4_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::ST1_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::ST2_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::ST3_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
            ASISDLSOP::ST4_LEt_SIMD_ADDR_POST(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDLSOP::LD1R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::LD1_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::LD2R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::LD2_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::LD3R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::LD3_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::LD4R_LVt_AL_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::LD4_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::ST1_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::ST2_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::ST3_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
            ASISDLSOP::ST4_LEt_SIMD_ADDR_POST(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDMISC {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDMISC::ABS_Sd_Sn(opcode) => opcode.definition(),
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
            ASISDMISC::FRECPE_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FRECPE_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FRECPX_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FRECPX_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::FRSQRTE_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::FRSQRTE_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::NEG_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::SCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::SCVTF_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::SQABS_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::SQNEG_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::SQXTN_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::SQXTUN_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::SUQADD_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::UCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.definition(),
            ASISDMISC::UCVTF_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::UQXTN_Sd_Sn(opcode) => opcode.definition(),
            ASISDMISC::USQADD_Sd_Sn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDMISC::ABS_Sd_Sn(opcode) => opcode.bits(),
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
            ASISDMISC::FRECPE_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FRECPE_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FRECPX_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FRECPX_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::FRSQRTE_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::FRSQRTE_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::NEG_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::SCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::SCVTF_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::SQABS_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::SQNEG_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::SQXTN_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::SQXTUN_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::SUQADD_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::UCVTF_Sd_S_H_Sn_S_H(opcode) => opcode.bits(),
            ASISDMISC::UCVTF_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::UQXTN_Sd_Sn(opcode) => opcode.bits(),
            ASISDMISC::USQADD_Sd_Sn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDONE {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDONE::DUP_Sd_En(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDONE::DUP_Sd_En(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for ASISDPAIR {
    fn definition(&self) -> &'static Insn {
        match self {
            ASISDPAIR::ADDP_Sd_Vn(opcode) => opcode.definition(),
            ASISDPAIR::FADDP_Sd_S_S_Vn_V_2S(opcode) => opcode.definition(),
            ASISDPAIR::FADDP_Sd_Vn(opcode) => opcode.definition(),
            ASISDPAIR::FMAXNMP_Sd_S_S_Vn_V_2S(opcode) => opcode.definition(),
            ASISDPAIR::FMAXNMP_Sd_Vn(opcode) => opcode.definition(),
            ASISDPAIR::FMAXP_Sd_S_S_Vn_V_2S(opcode) => opcode.definition(),
            ASISDPAIR::FMAXP_Sd_Vn(opcode) => opcode.definition(),
            ASISDPAIR::FMINNMP_Sd_S_S_Vn_V_2S(opcode) => opcode.definition(),
            ASISDPAIR::FMINNMP_Sd_Vn(opcode) => opcode.definition(),
            ASISDPAIR::FMINP_Sd_S_S_Vn_V_2S(opcode) => opcode.definition(),
            ASISDPAIR::FMINP_Sd_Vn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            ASISDPAIR::ADDP_Sd_Vn(opcode) => opcode.bits(),
            ASISDPAIR::FADDP_Sd_S_S_Vn_V_2S(opcode) => opcode.bits(),
            ASISDPAIR::FADDP_Sd_Vn(opcode) => opcode.bits(),
            ASISDPAIR::FMAXNMP_Sd_S_S_Vn_V_2S(opcode) => opcode.bits(),
            ASISDPAIR::FMAXNMP_Sd_Vn(opcode) => opcode.bits(),
            ASISDPAIR::FMAXP_Sd_S_S_Vn_V_2S(opcode) => opcode.bits(),
            ASISDPAIR::FMAXP_Sd_Vn(opcode) => opcode.bits(),
            ASISDPAIR::FMINNMP_Sd_S_S_Vn_V_2S(opcode) => opcode.bits(),
            ASISDPAIR::FMINNMP_Sd_Vn(opcode) => opcode.bits(),
            ASISDPAIR::FMINP_Sd_S_S_Vn_V_2S(opcode) => opcode.bits(),
            ASISDPAIR::FMINP_Sd_Vn(opcode) => opcode.bits(),
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
            ASISDSAME::FABD_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FABD_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FACGE_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FACGE_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FACGT_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FACGT_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FCMEQ_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FCMGE_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FCMGE_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FCMGT_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FCMGT_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FMULX_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FMULX_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FRECPS_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FRECPS_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::FRSQRTS_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.definition(),
            ASISDSAME::FRSQRTS_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SQADD_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SQDMULH_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SQRDMULH_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SQRSHL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SQSHL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SQSUB_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SRSHL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SSHL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::SUB_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::UQADD_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::UQRSHL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::UQSHL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::UQSUB_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::URSHL_Sd_Sn_Sm(opcode) => opcode.definition(),
            ASISDSAME::USHL_Sd_Sn_Sm(opcode) => opcode.definition(),
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
            ASISDSAME::FABD_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FABD_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FACGE_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FACGE_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FACGT_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FACGT_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FCMEQ_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FCMGE_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FCMGE_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FCMGT_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FCMGT_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FMULX_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FMULX_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FRECPS_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FRECPS_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::FRSQRTS_Sd_S_S_Sn_S_S_Sm_S_S(opcode) => opcode.bits(),
            ASISDSAME::FRSQRTS_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SQADD_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SQDMULH_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SQRDMULH_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SQRSHL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SQSHL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SQSUB_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SRSHL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SSHL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::SUB_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::UQADD_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::UQRSHL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::UQSHL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::UQSUB_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::URSHL_Sd_Sn_Sm(opcode) => opcode.bits(),
            ASISDSAME::USHL_Sd_Sn_Sm(opcode) => opcode.bits(),
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
            ASISDSHF::SHL_Sd_Sn_IMM_VLSL(opcode) => opcode.definition(),
            ASISDSHF::SLI_Sd_Sn_IMM_VLSL(opcode) => opcode.definition(),
            ASISDSHF::SQRSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SQRSHRUN_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SQSHLU_Sd_Sn_IMM_VLSL(opcode) => opcode.definition(),
            ASISDSHF::SQSHL_Sd_Sn_IMM_VLSL(opcode) => opcode.definition(),
            ASISDSHF::SQSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SQSHRUN_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SRI_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SRSHR_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SRSRA_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SSHR_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::SSRA_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.definition(),
            ASISDSHF::UCVTF_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::UQRSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::UQSHL_Sd_Sn_IMM_VLSL(opcode) => opcode.definition(),
            ASISDSHF::UQSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::URSHR_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::URSRA_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::USHR_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
            ASISDSHF::USRA_Sd_Sn_IMM_VLSR(opcode) => opcode.definition(),
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
            ASISDSHF::SHL_Sd_Sn_IMM_VLSL(opcode) => opcode.bits(),
            ASISDSHF::SLI_Sd_Sn_IMM_VLSL(opcode) => opcode.bits(),
            ASISDSHF::SQRSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SQRSHRUN_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SQSHLU_Sd_Sn_IMM_VLSL(opcode) => opcode.bits(),
            ASISDSHF::SQSHL_Sd_Sn_IMM_VLSL(opcode) => opcode.bits(),
            ASISDSHF::SQSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SQSHRUN_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SRI_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SRSHR_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SRSRA_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SSHR_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::SSRA_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(opcode) => opcode.bits(),
            ASISDSHF::UCVTF_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::UQRSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::UQSHL_Sd_Sn_IMM_VLSL(opcode) => opcode.bits(),
            ASISDSHF::UQSHRN_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::URSHR_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::URSRA_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::USHR_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
            ASISDSHF::USRA_Sd_Sn_IMM_VLSR(opcode) => opcode.bits(),
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
            BRANCH_REG::DRPS(opcode) => opcode.definition(),
            BRANCH_REG::ERET(opcode) => opcode.definition(),
            BRANCH_REG::ERETAA(opcode) => opcode.definition(),
            BRANCH_REG::ERETAB(opcode) => opcode.definition(),
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
            BRANCH_REG::DRPS(opcode) => opcode.bits(),
            BRANCH_REG::ERET(opcode) => opcode.bits(),
            BRANCH_REG::ERETAA(opcode) => opcode.bits(),
            BRANCH_REG::ERETAB(opcode) => opcode.bits(),
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
