#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Sd_S_S_Sn_S_S_Sm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x5e20dc00,
        mask: 0xffa0fc00,
        class: InsnClass::ASISDSAME,
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
                kind: InsnOperandKind::Sm,
                class: InsnOperandClass::SISD_REG,
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
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASISDSAME(ASISDSAME::FMULX_Sd_S_S_Sn_S_S_Sm_S_S(
                FMULX_Sd_S_S_Sn_S_S_Sm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMULX_Sd_S_S_Sn_S_S_Sm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Vd_Vn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x2f009000,
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
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASIMDELEM(ASIMDELEM::FMULX_Vd_Vn_Em16(FMULX_Vd_Vn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMULX_Vd_Vn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Vd_Vn_Em {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x2f809000,
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
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASIMDELEM(ASIMDELEM::FMULX_Vd_Vn_Em(FMULX_Vd_Vn_Em::from(bits))),
        }
    }
}
impl InsnOpcode for FMULX_Vd_Vn_Em {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Sd_Sn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x7f009000,
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
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASISDELEM(ASISDELEM::FMULX_Sd_Sn_Em16(FMULX_Sd_Sn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMULX_Sd_Sn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Sd_Sn_Em {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x7f809000,
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
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASISDELEM(ASISDELEM::FMULX_Sd_Sn_Em(FMULX_Sd_Sn_Em::from(bits))),
        }
    }
}
impl InsnOpcode for FMULX_Sd_Sn_Em {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_SME_ZA_array_off3_0_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0xc1a01c08,
        mask: 0xffbf9c38,
        class: InsnClass::SME_FP_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Znx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn2,
                    lsb: 6,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::SME_FP_SD(SME_FP_SD::FSUB_SME_ZA_array_off3_0_SME_Znx2(
                FSUB_SME_ZA_array_off3_0_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FSUB_SME_ZA_array_off3_0_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_SME_ZA_array_off3_0_SME_Znx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0xc1a11c08,
        mask: 0xffbf9c78,
        class: InsnClass::SME_FP_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Znx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn4,
                    lsb: 7,
                    width: 3,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::SME_FP_SD(SME_FP_SD::FSUB_SME_ZA_array_off3_0_SME_Znx4(
                FSUB_SME_ZA_array_off3_0_SME_Znx4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FSUB_SME_ZA_array_off3_0_SME_Znx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0x65000400,
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
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(
                FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0x65018000,
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
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0x65198000,
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
                kind: InsnOperandKind::SVE_I1_HALF_ONE,
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
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::SVE_SIZE_HSD(
                SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(
                    FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0xec01400,
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
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::ASIMDSAME(ASIMDSAME::FSUB_Vd_Vn_Vm(FSUB_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for FSUB_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_Fd_Fn_Fm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0x1ee03800,
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
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::FLOATDP2(FLOATDP2::FSUB_Fd_Fn_Fm(FSUB_Fd_Fn_Fm::from(bits))),
        }
    }
}
impl InsnOpcode for FSUB_Fd_Fn_Fm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_Fd_S_S_Fn_S_S_Fm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0x1e203800,
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
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::FLOATDP2(FLOATDP2::FSUB_Fd_S_S_Fn_S_S_Fm_S_S(
                FSUB_Fd_S_S_Fn_S_S_Fm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FSUB_Fd_S_S_Fn_S_S_Fm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsub",
        aliases: &[],
        opcode: 0xea0d400,
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
            mnemonic: Mnemonic::r#fsub,
            operation: Operation::ASIMDSAME(ASIMDSAME::FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S(
                FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsubr",
        aliases: &[],
        opcode: 0x65038000,
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
            mnemonic: Mnemonic::r#fsubr,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fsubr",
        aliases: &[],
        opcode: 0x651b8000,
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
                kind: InsnOperandKind::SVE_I1_HALF_ONE,
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
            mnemonic: Mnemonic::r#fsubr,
            operation: Operation::SVE_SIZE_HSD(
                SVE_SIZE_HSD::FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(
                    FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldar",
        aliases: &[],
        opcode: 0x88dffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldar,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAR_Rt_ADDR_SIMPLE(
                LDAR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDARB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldarb",
        aliases: &[],
        opcode: 0x8dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldarb,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDARB_Rt_ADDR_SIMPLE(
                LDARB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDARB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDARH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldarh",
        aliases: &[],
        opcode: 0x48dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldarh,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDARH_Rt_ADDR_SIMPLE(
                LDARH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDARH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x29400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(
                LDP_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDP_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x28c00000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(
                    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Ft_Ft2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2d400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(
                LDP_Ft_Ft2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDP_Ft_Ft2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2cc00000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(
                    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDPSW_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x69400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldpsw,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(
                LDPSW_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDPSW_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x68c00000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldpsw,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(
                    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xe1000000,
        mask: 0xffff9c10,
        class: InsnClass::SME_LDR,
        feature_set: InsnFeatureSet::SME,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off4,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm4_0,
                        lsb: 0,
                        width: 4,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ADDR_RI_U4xVL,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm4_0,
                        lsb: 0,
                        width: 4,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SME_LDR(SME_LDR::LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(
                LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SME_ZT0_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xe11f8000,
        mask: 0xfffffc1f,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZT0,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SME_MISC(SME_MISC::LDR_SME_ZT0_SIMD_ADDR_SIMPLE(
                LDR_SME_ZT0_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SME_ZT0_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x18000000,
        mask: 0xbf000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_PCREL19,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm19,
                    lsb: 5,
                    width: 19,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LOADLIT(LOADLIT::LDR_Rt_ADDR_PCREL19(LDR_Rt_ADDR_PCREL19::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8600800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDR_Rt_ADDR_REGOFF(
                LDR_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8400400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDR_Rt_ADDR_SIMM9(LDR_Rt_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb9400000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_POS(LDST_POS::LDR_Rt_ADDR_UIMM12(LDR_Rt_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SVE_Pt_SVE_ADDR_RI_S9xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x85800000,
        mask: 0xffc0e010,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Pt,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pt,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_ADDR_RI_S9xVL,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
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
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SVE_MISC(SVE_MISC::LDR_SVE_Pt_SVE_ADDR_RI_S9xVL(
                LDR_SVE_Pt_SVE_ADDR_RI_S9xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SVE_Pt_SVE_ADDR_RI_S9xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
