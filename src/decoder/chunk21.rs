#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmops",
        aliases: &[],
        opcode: 0x80800010,
        mask: 0xffe0001c,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZAda_2b,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_ZAda_2b,
                    lsb: 0,
                    width: 2,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Pm,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Pm,
                    lsb: 13,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmops,
            operation: Operation::SME_MISC(
                SME_MISC::FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
                    FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmops",
        aliases: &[],
        opcode: 0x80c00010,
        mask: 0xffe00018,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME_F64F64,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZAda_3b,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_ZAda_3b,
                    lsb: 0,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Pm,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Pm,
                    lsb: 13,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmops,
            operation: Operation::SME_MISC(
                SME_MISC::FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
                    FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmops",
        aliases: &[],
        opcode: 0x81a00010,
        mask: 0xffe0001c,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZAda_2b,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_ZAda_2b,
                    lsb: 0,
                    width: 2,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Pm,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Pm,
                    lsb: 13,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmops,
            operation: Operation::SME_MISC(
                SME_MISC::FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                    FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H::from(
                        bits,
                    ),
                ),
            ),
        }
    }
}
impl InsnOpcode for FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Rd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1ee60000,
        mask: 0x7ffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOAT2INT(FLOAT2INT::FMOV_Rd_Fn(FMOV_Rd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for FMOV_Rd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Rd_W_Fn_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1e260000,
        mask: 0x7f3ffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOAT2INT(FLOAT2INT::FMOV_Rd_W_Fn_S_S(FMOV_Rd_W_Fn_S_S::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMOV_Rd_W_Fn_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Rd_VnD1 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x9eae0000,
        mask: 0xfffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::VnD1,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOAT2INT(FLOAT2INT::FMOV_Rd_VnD1(FMOV_Rd_VnD1::from(bits))),
        }
    }
}
impl InsnOpcode for FMOV_Rd_VnD1 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Fd_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1ee70000,
        mask: 0x7ffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOAT2INT(FLOAT2INT::FMOV_Fd_Rn(FMOV_Fd_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for FMOV_Fd_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Fd_S_S_Rn_W {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1e270000,
        mask: 0x7f3ffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOAT2INT(FLOAT2INT::FMOV_Fd_S_S_Rn_W(FMOV_Fd_S_S_Rn_W::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMOV_Fd_S_S_Rn_W {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_VdD1_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x9eaf0000,
        mask: 0xfffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::VdD1,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOAT2INT(FLOAT2INT::FMOV_VdD1_Rn(FMOV_VdD1_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for FMOV_VdD1_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Fd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1ee04000,
        mask: 0xfffffc00,
        class: InsnClass::FLOATDP1,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOATDP1(FLOATDP1::FMOV_Fd_Fn(FMOV_Fd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for FMOV_Fd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Fd_S_S_Fn_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1e204000,
        mask: 0xff3ffc00,
        class: InsnClass::FLOATDP1,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOATDP1(FLOATDP1::FMOV_Fd_S_S_Fn_S_S(FMOV_Fd_S_S_Fn_S_S::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMOV_Fd_S_S_Fn_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Fd_FPIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1ee01000,
        mask: 0xffe01fe0,
        class: InsnClass::FLOATIMM,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::FPIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm8,
                    lsb: 13,
                    width: 8,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOATIMM(FLOATIMM::FMOV_Fd_FPIMM(FMOV_Fd_FPIMM::from(bits))),
        }
    }
}
impl InsnOpcode for FMOV_Fd_FPIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Fd_S_S_FPIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x1e201000,
        mask: 0xff201fe0,
        class: InsnClass::FLOATIMM,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::FPIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm8,
                    lsb: 13,
                    width: 8,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::FLOATIMM(FLOATIMM::FMOV_Fd_S_S_FPIMM(FMOV_Fd_S_S_FPIMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMOV_Fd_S_S_FPIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Vd_SIMD_FPIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0xf00f400,
        mask: 0xbff8fc00,
        class: InsnClass::ASIMDIMM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_2S, InsnOperandQualifier::V_4S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_FPIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::ASIMDIMM(ASIMDIMM::FMOV_Vd_SIMD_FPIMM(FMOV_Vd_SIMD_FPIMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMOV_Vd_SIMD_FPIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Vd_V_2D_SIMD_FPIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0x6f00f400,
        mask: 0xfff8fc00,
        class: InsnClass::ASIMDIMM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_2D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_FPIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::ASIMDIMM(ASIMDIMM::FMOV_Vd_V_2D_SIMD_FPIMM(
                FMOV_Vd_V_2D_SIMD_FPIMM::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMOV_Vd_V_2D_SIMD_FPIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMOV_Vd_V_4H_SIMD_FPIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmov",
        aliases: &[],
        opcode: 0xf00fc00,
        mask: 0xbff8fc00,
        class: InsnClass::ASIMDIMM,
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
                kind: InsnOperandKind::SIMD_FPIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmov,
            operation: Operation::ASIMDIMM(ASIMDIMM::FMOV_Vd_V_4H_SIMD_FPIMM(
                FMOV_Vd_V_4H_SIMD_FPIMM::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMOV_Vd_V_4H_SIMD_FPIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmsb",
        aliases: &[],
        opcode: 0x6520a000,
        mask: 0xff20e000,
        class: InsnClass::SVE_SIZE_HSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_5,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Za_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Za_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmsb,
            operation: Operation::SVE_SIZE_HSD(
                SVE_SIZE_HSD::FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16(
                    FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMSUB_Fd_Fn_Fm_Fa {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmsub",
        aliases: &[],
        opcode: 0x1fc08000,
        mask: 0xffe08000,
        class: InsnClass::FLOATDP3,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fa,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Ra,
                    lsb: 10,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmsub,
            operation: Operation::FLOATDP3(FLOATDP3::FMSUB_Fd_Fn_Fm_Fa(FMSUB_Fd_Fn_Fm_Fa::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMSUB_Fd_Fn_Fm_Fa {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmsub",
        aliases: &[],
        opcode: 0x1f008000,
        mask: 0xff208000,
        class: InsnClass::FLOATDP3,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fa,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Ra,
                    lsb: 10,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmsub,
            operation: Operation::FLOATDP3(FLOATDP3::FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(
                FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x64202000,
        mask: 0xffa0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm3_22_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i3h,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_Zm_16,
                        lsb: 16,
                        width: 5,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::SVE_MISC(SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(
                FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x64a02000,
        mask: 0xffe0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm3_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::SVE_MISC(SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(
                FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x64e02000,
        mask: 0xffe0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm4_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::SVE_MISC(SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(
                FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x65000800,
        mask: 0xff20fc00,
        class: InsnClass::SVE_SIZE_HSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(
                FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x65028000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_HSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_5,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x651a8000,
        mask: 0xff3fe3c0,
        class: InsnClass::SVE_SIZE_HSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_I1_HALF_TWO,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_i1,
                    lsb: 5,
                    width: 1,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::SVE_SIZE_HSD(
                SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO(
                    FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x2e401c00,
        mask: 0xbfe0fc00,
        class: InsnClass::ASIMDSAME,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
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
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::ASIMDSAME(ASIMDSAME::FMUL_Vd_Vn_Vm(FMUL_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for FMUL_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Fd_Fn_Fm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x1ee00800,
        mask: 0xffe0fc00,
        class: InsnClass::FLOATDP2,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::FLOATDP2(FLOATDP2::FMUL_Fd_Fn_Fm(FMUL_Fd_Fn_Fm::from(bits))),
        }
    }
}
impl InsnOpcode for FMUL_Fd_Fn_Fm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Fd_S_S_Fn_S_S_Fm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x1e200800,
        mask: 0xff20fc00,
        class: InsnClass::FLOATDP2,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::FLOATDP2(FLOATDP2::FMUL_Fd_S_S_Fn_S_S_Fm_S_S(
                FMUL_Fd_S_S_Fn_S_S_Fm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMUL_Fd_S_S_Fn_S_S_Fm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x2e20dc00,
        mask: 0xbfa0fc00,
        class: InsnClass::ASIMDSAME,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
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
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::ASIMDSAME(ASIMDSAME::FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S(
                FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Vd_Vn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0xf009000,
        mask: 0xbfc0f400,
        class: InsnClass::ASIMDELEM,
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
                kind: InsnOperandKind::Em16,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
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
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::ASIMDELEM(ASIMDELEM::FMUL_Vd_Vn_Em16(FMUL_Vd_Vn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMUL_Vd_Vn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Vd_Vn_Em {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0xf809000,
        mask: 0xbf80f400,
        class: InsnClass::ASIMDELEM,
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
                kind: InsnOperandKind::Em,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
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
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::ASIMDELEM(ASIMDELEM::FMUL_Vd_Vn_Em(FMUL_Vd_Vn_Em::from(bits))),
        }
    }
}
impl InsnOpcode for FMUL_Vd_Vn_Em {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Sd_Sn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x5f009000,
        mask: 0xffc0f400,
        class: InsnClass::ASISDELEM,
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
                kind: InsnOperandKind::Em16,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::ASISDELEM(ASISDELEM::FMUL_Sd_Sn_Em16(FMUL_Sd_Sn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMUL_Sd_Sn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMUL_Sd_Sn_Em {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmul",
        aliases: &[],
        opcode: 0x5f809000,
        mask: 0xff80f400,
        class: InsnClass::ASISDELEM,
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
                kind: InsnOperandKind::Em,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmul,
            operation: Operation::ASISDELEM(ASISDELEM::FMUL_Sd_Sn_Em(FMUL_Sd_Sn_Em::from(bits))),
        }
    }
}
