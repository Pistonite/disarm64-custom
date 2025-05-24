#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for ANDQV_Vd_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDS_Rd_Rn_LIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x72000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::N,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immr,
                        lsb: 16,
                        width: 6,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imms,
                        lsb: 10,
                        width: 6,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ands,
            operation: Operation::LOG_IMM(LOG_IMM::ANDS_Rd_Rn_LIMM(ANDS_Rd_Rn_LIMM::from(bits))),
        }
    }
}
impl InsnOpcode for ANDS_Rd_Rn_LIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDS_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x6a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ands,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(ANDS_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ANDS_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x25404000,
        mask: 0xfff0c210,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Pd,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pd,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg4_10,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_Z],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg4_10,
                    lsb: 10,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pn,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pn,
                    lsb: 5,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pm,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pm,
                    lsb: 16,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ands,
            operation: Operation::SVE_MISC(SVE_MISC::ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDV_SVE_Vd_SVE_Pg3_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "andv",
        aliases: &[],
        opcode: 0x41a2000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_BHSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Vd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#andv,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::ANDV_SVE_Vd_SVE_Pg3_SVE_Zn(
                ANDV_SVE_Vd_SVE_Pg3_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ANDV_SVE_Vd_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asr",
        aliases: &[],
        opcode: 0x4008000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SHIFT_PRED,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
                kind: InsnOperandKind::SVE_SHRIMM_PRED,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_tszh,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_imm5,
                        lsb: 5,
                        width: 5,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#asr,
            operation: Operation::SVE_SHIFT_PRED(
                SVE_SHIFT_PRED::ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(
                    ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asr",
        aliases: &[],
        opcode: 0x4108000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_BHSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
            mnemonic: Mnemonic::r#asr,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asr",
        aliases: &[],
        opcode: 0x4188000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_BHS,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
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
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
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
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_D,
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
            mnemonic: Mnemonic::r#asr,
            operation: Operation::SVE_SIZE_BHS(
                SVE_SIZE_BHS::ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(
                    ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASR_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asr",
        aliases: &[],
        opcode: 0x4208000,
        mask: 0xff20fc00,
        class: InsnClass::SVE_SIZE_BHS,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
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
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
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
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_D,
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
            mnemonic: Mnemonic::r#asr,
            operation: Operation::SVE_SIZE_BHS(SVE_SIZE_BHS::ASR_SVE_Zd_SVE_Zn_SVE_Zm_16(
                ASR_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ASR_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asr",
        aliases: &[],
        opcode: 0x4209000,
        mask: 0xff20fc00,
        class: InsnClass::SVE_SHIFT_UNPRED,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
                kind: InsnOperandKind::SVE_SHRIMM_UNPRED,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_tszh,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm5,
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
            mnemonic: Mnemonic::r#asr,
            operation: Operation::SVE_SHIFT_UNPRED(
                SVE_SHIFT_UNPRED::ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(
                    ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asrd",
        aliases: &[],
        opcode: 0x4048000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SHIFT_PRED,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
                kind: InsnOperandKind::SVE_SHRIMM_PRED,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_tszh,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_imm5,
                        lsb: 5,
                        width: 5,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#asrd,
            operation: Operation::SVE_SHIFT_PRED(
                SVE_SHIFT_PRED::ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(
                    ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asrr",
        aliases: &[],
        opcode: 0x4148000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_BHSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
            mnemonic: Mnemonic::r#asrr,
            operation: Operation::SVE_SIZE_BHSD(
                SVE_SIZE_BHSD::ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                    ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASRV_Rd_Rn_Rm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asrv",
        aliases: &[],
        opcode: 0x1ac02800,
        mask: 0x7fe0fc00,
        class: InsnClass::DP_2SRC,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#asrv,
            operation: Operation::DP_2SRC(DP_2SRC::ASRV_Rd_Rn_Rm(ASRV_Rd_Rn_Rm::from(bits))),
        }
    }
}
impl InsnOpcode for ASRV_Rd_Rn_Rm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl B_ADDR_PCREL26 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "b",
        aliases: &[],
        opcode: 0x14000000,
        mask: 0xfc000000,
        class: InsnClass::BRANCH_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::ADDR_PCREL26,
            class: InsnOperandClass::ADDRESS,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm26,
                lsb: 0,
                width: 26,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#b,
            operation: Operation::BRANCH_IMM(BRANCH_IMM::B_ADDR_PCREL26(B_ADDR_PCREL26::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for B_ADDR_PCREL26 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfadd",
        aliases: &[],
        opcode: 0x65000000,
        mask: 0xffe0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::B16B16,
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
            mnemonic: Mnemonic::r#bfadd,
            operation: Operation::SVE_MISC(SVE_MISC::BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfadd",
        aliases: &[],
        opcode: 0x65008000,
        mask: 0xffffe000,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::B16B16,
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
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
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
            mnemonic: Mnemonic::r#bfadd,
            operation: Operation::SVE_MISC(SVE_MISC::BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfclamp",
        aliases: &[],
        opcode: 0x64202400,
        mask: 0xffe0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::B16B16,
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
            mnemonic: Mnemonic::r#bfclamp,
            operation: Operation::SVE_MISC(SVE_MISC::BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCVT_SVE_Zd_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfcvt",
        aliases: &[],
        opcode: 0xc160e000,
        mask: 0xfffffc20,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
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
                kind: InsnOperandKind::SME_Znx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn2,
                    lsb: 6,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#bfcvt,
            operation: Operation::SME_MISC(SME_MISC::BFCVT_SVE_Zd_SME_Znx2(
                BFCVT_SVE_Zd_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFCVT_SVE_Zd_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfcvt",
        aliases: &[],
        opcode: 0x658aa000,
        mask: 0xffffe000,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::BFLOAT16_SVE,
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
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#bfcvt,
            operation: Operation::SVE_MISC(SVE_MISC::BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn(
                BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCVT_Fd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfcvt",
        aliases: &[],
        opcode: 0x1e634000,
        mask: 0xfffffc00,
        class: InsnClass::BFLOAT16,
        feature_set: InsnFeatureSet::BFLOAT16,
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
                qualifiers: &[InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#bfcvt,
            operation: Operation::BFLOAT16(BFLOAT16::BFCVT_Fd_Fn(BFCVT_Fd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for BFCVT_Fd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCVTN_SVE_Zd_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfcvtn",
        aliases: &[],
        opcode: 0xc160e020,
        mask: 0xfffffc20,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
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
                kind: InsnOperandKind::SME_Znx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn2,
                    lsb: 6,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#bfcvtn,
            operation: Operation::SME_MISC(SME_MISC::BFCVTN_SVE_Zd_SME_Znx2(
                BFCVTN_SVE_Zd_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFCVTN_SVE_Zd_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCVTN_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfcvtn",
        aliases: &[],
        opcode: 0xea16800,
        mask: 0xfffffc00,
        class: InsnClass::BFLOAT16,
        feature_set: InsnFeatureSet::BFLOAT16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S],
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
            mnemonic: Mnemonic::r#bfcvtn,
            operation: Operation::BFLOAT16(BFLOAT16::BFCVTN_Vd_Vn(BFCVTN_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for BFCVTN_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCVTN2_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfcvtn2",
        aliases: &[],
        opcode: 0x4ea16800,
        mask: 0xfffffc00,
        class: InsnClass::BFLOAT16,
        feature_set: InsnFeatureSet::BFLOAT16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S],
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
            mnemonic: Mnemonic::r#bfcvtn2,
            operation: Operation::BFLOAT16(BFLOAT16::BFCVTN2_Vd_Vn(BFCVTN2_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for BFCVTN2_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfcvtnt",
        aliases: &[],
        opcode: 0x648aa000,
        mask: 0xffffe000,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::BFLOAT16_SVE,
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
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#bfcvtnt,
            operation: Operation::SVE_MISC(SVE_MISC::BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn(
                BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfdot",
        aliases: &[],
        opcode: 0xc1501018,
        mask: 0xfff09038,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn2,
                    lsb: 6,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm_INDEX2,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Zm,
                        lsb: 16,
                        width: 4,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm2_10,
                        lsb: 10,
                        width: 2,
                    },
                ],
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
            mnemonic: Mnemonic::r#bfdot,
            operation: Operation::SME_MISC(
                SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(
                    BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfdot",
        aliases: &[],
        opcode: 0xc1509018,
        mask: 0xfff09078,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn4,
                    lsb: 7,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm_INDEX2,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Zm,
                        lsb: 16,
                        width: 4,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm2_10,
                        lsb: 10,
                        width: 2,
                    },
                ],
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
            mnemonic: Mnemonic::r#bfdot,
            operation: Operation::SME_MISC(
                SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(
                    BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfdot",
        aliases: &[],
        opcode: 0xc1201010,
        mask: 0xfff09c18,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
                kind: InsnOperandKind::SVE_ZnxN,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm,
                    lsb: 16,
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
            mnemonic: Mnemonic::r#bfdot,
            operation: Operation::SME_MISC(SME_MISC::BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(
                BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfdot",
        aliases: &[],
        opcode: 0xc1301010,
        mask: 0xfff09c18,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
                kind: InsnOperandKind::SVE_ZnxN,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm,
                    lsb: 16,
                    width: 4,
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
            mnemonic: Mnemonic::r#bfdot,
            operation: Operation::SME_MISC(
                SME_MISC::BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(
                    BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfdot",
        aliases: &[],
        opcode: 0xc1a01010,
        mask: 0xffe19c38,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn2,
                    lsb: 6,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zmx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm2,
                    lsb: 17,
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
            mnemonic: Mnemonic::r#bfdot,
            operation: Operation::SME_MISC(SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(
                BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfdot",
        aliases: &[],
        opcode: 0xc1a11010,
        mask: 0xffe39c78,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn4,
                    lsb: 7,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zmx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm4,
                    lsb: 18,
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
            mnemonic: Mnemonic::r#bfdot,
            operation: Operation::SME_MISC(SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(
                BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfdot",
        aliases: &[],
        opcode: 0x64608000,
        mask: 0xffe0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::BFLOAT16_SVE,
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
            mnemonic: Mnemonic::r#bfdot,
            operation: Operation::SVE_MISC(SVE_MISC::BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
