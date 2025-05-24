#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for LSL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsl",
        aliases: &[],
        opcode: 0x4209c00,
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
                kind: InsnOperandKind::SVE_SHLIMM_UNPRED,
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
            mnemonic: Mnemonic::r#lsl,
            operation: Operation::SVE_SHIFT_UNPRED(
                SVE_SHIFT_UNPRED::LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED(
                    LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lslr",
        aliases: &[],
        opcode: 0x4178000,
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
            mnemonic: Mnemonic::r#lslr,
            operation: Operation::SVE_SIZE_BHSD(
                SVE_SIZE_BHSD::LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                    LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSLV_Rd_Rn_Rm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lslv",
        aliases: &[],
        opcode: 0x1ac02000,
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
            mnemonic: Mnemonic::r#lslv,
            operation: Operation::DP_2SRC(DP_2SRC::LSLV_Rd_Rn_Rm(LSLV_Rd_Rn_Rm::from(bits))),
        }
    }
}
impl InsnOpcode for LSLV_Rd_Rn_Rm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsr",
        aliases: &[],
        opcode: 0x4018000,
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
            mnemonic: Mnemonic::r#lsr,
            operation: Operation::SVE_SHIFT_PRED(
                SVE_SHIFT_PRED::LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(
                    LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsr",
        aliases: &[],
        opcode: 0x4118000,
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
            mnemonic: Mnemonic::r#lsr,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsr",
        aliases: &[],
        opcode: 0x4198000,
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
            mnemonic: Mnemonic::r#lsr,
            operation: Operation::SVE_SIZE_BHS(
                SVE_SIZE_BHS::LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(
                    LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSR_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsr",
        aliases: &[],
        opcode: 0x4208400,
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
            mnemonic: Mnemonic::r#lsr,
            operation: Operation::SVE_SIZE_BHS(SVE_SIZE_BHS::LSR_SVE_Zd_SVE_Zn_SVE_Zm_16(
                LSR_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LSR_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsr",
        aliases: &[],
        opcode: 0x4209400,
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
            mnemonic: Mnemonic::r#lsr,
            operation: Operation::SVE_SHIFT_UNPRED(
                SVE_SHIFT_UNPRED::LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(
                    LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsrr",
        aliases: &[],
        opcode: 0x4158000,
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
            mnemonic: Mnemonic::r#lsrr,
            operation: Operation::SVE_SIZE_BHSD(
                SVE_SIZE_BHSD::LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                    LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSRV_Rd_Rn_Rm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsrv",
        aliases: &[],
        opcode: 0x1ac02400,
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
            mnemonic: Mnemonic::r#lsrv,
            operation: Operation::DP_2SRC(DP_2SRC::LSRV_Rd_Rn_Rm(LSRV_Rd_Rn_Rm::from(bits))),
        }
    }
}
impl InsnOpcode for LSRV_Rd_Rn_Rm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MADD_Rd_Rn_Rm_Ra {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "madd",
        aliases: &[],
        opcode: 0x1b000000,
        mask: 0x7fe08000,
        class: InsnClass::DP_3SRC,
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
            InsnOperand {
                kind: InsnOperandKind::Ra,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Ra,
                    lsb: 10,
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
            mnemonic: Mnemonic::r#madd,
            operation: Operation::DP_3SRC(DP_3SRC::MADD_Rd_Rn_Rm_Ra(MADD_Rd_Rn_Rm_Ra::from(bits))),
        }
    }
}
impl InsnOpcode for MADD_Rd_Rn_Rm_Ra {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0000000,
        mask: 0xff3e0010,
        class: InsnClass::SME_MOV,
        feature_set: InsnFeatureSet::SME,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_dest,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_size_22,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Q,
                        lsb: 16,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
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
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[
                    InsnOperandQualifier::P_M,
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
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME_MOV(SME_MOV::MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(
                MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_ZA_HV_idx_destxN_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0040000,
        mask: 0xff3f1c38,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_destxN,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOV_SME_ZA_HV_idx_destxN_SME_Znx2(
                MOV_SME_ZA_HV_idx_destxN_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_ZA_HV_idx_destxN_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_ZA_array_off3_0_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0040800,
        mask: 0xffff9c38,
        class: InsnClass::SME2_MOV,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME2_MOV(SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx2(
                MOV_SME_ZA_array_off3_0_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_ZA_array_off3_0_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_ZA_HV_idx_destxN_SME_Znx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0040400,
        mask: 0xff3f1c78,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_destxN,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOV_SME_ZA_HV_idx_destxN_SME_Znx4(
                MOV_SME_ZA_HV_idx_destxN_SME_Znx4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_ZA_HV_idx_destxN_SME_Znx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_ZA_array_off3_0_SME_Znx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0040c00,
        mask: 0xffff9c78,
        class: InsnClass::SME2_MOV,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME2_MOV(SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx4(
                MOV_SME_ZA_array_off3_0_SME_Znx4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_ZA_array_off3_0_SME_Znx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0020000,
        mask: 0xff3e0200,
        class: InsnClass::SME_MOV,
        feature_set: InsnFeatureSet::SME,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
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
                    InsnOperandQualifier::P_M,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_src,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_size_22,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Q,
                        lsb: 16,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm4_5,
                        lsb: 5,
                        width: 4,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME_MOV(SME_MOV::MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(
                MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0060000,
        mask: 0xff3f1f01,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_srcxN,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_5,
                        lsb: 5,
                        width: 3,
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN(
                MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_Zdnx2_SME_ZA_array_off3_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0060800,
        mask: 0xffff9f01,
        class: InsnClass::SME2_MOV,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_5,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_5,
                        lsb: 5,
                        width: 3,
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME2_MOV(SME2_MOV::MOV_SME_Zdnx2_SME_ZA_array_off3_5(
                MOV_SME_Zdnx2_SME_ZA_array_off3_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_Zdnx2_SME_ZA_array_off3_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0060400,
        mask: 0xff3f1f03,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_srcxN,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_5,
                        lsb: 5,
                        width: 3,
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN(
                MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOV_SME_Zdnx4_SME_ZA_array_off3_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mov",
        aliases: &[],
        opcode: 0xc0060c00,
        mask: 0xffff9f03,
        class: InsnClass::SME2_MOV,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_5,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_5,
                        lsb: 5,
                        width: 3,
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
            mnemonic: Mnemonic::r#mov,
            operation: Operation::SME2_MOV(SME2_MOV::MOV_SME_Zdnx4_SME_ZA_array_off3_5(
                MOV_SME_Zdnx4_SME_ZA_array_off3_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOV_SME_Zdnx4_SME_ZA_array_off3_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
        aliases: &[],
        opcode: 0xc0000000,
        mask: 0xff3e0010,
        class: InsnClass::SME_MOV,
        feature_set: InsnFeatureSet::SME,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_dest,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_size_22,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Q,
                        lsb: 16,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
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
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[
                    InsnOperandQualifier::P_M,
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
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME_MOV(SME_MOV::MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(
                MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_ZA_HV_idx_destxN_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
        aliases: &[],
        opcode: 0xc0040000,
        mask: 0xff3f1c38,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_destxN,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOVA_SME_ZA_HV_idx_destxN_SME_Znx2(
                MOVA_SME_ZA_HV_idx_destxN_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_ZA_HV_idx_destxN_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_ZA_array_off3_0_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
        aliases: &[],
        opcode: 0xc0040800,
        mask: 0xffff9c38,
        class: InsnClass::SME2_MOV,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME2_MOV(SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx2(
                MOVA_SME_ZA_array_off3_0_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_ZA_array_off3_0_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_ZA_HV_idx_destxN_SME_Znx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
        aliases: &[],
        opcode: 0xc0040400,
        mask: 0xff3f1c78,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_HV_idx_destxN,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_V,
                        lsb: 15,
                        width: 1,
                    },
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOVA_SME_ZA_HV_idx_destxN_SME_Znx4(
                MOVA_SME_ZA_HV_idx_destxN_SME_Znx4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_ZA_HV_idx_destxN_SME_Znx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_ZA_array_off3_0_SME_Znx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
        aliases: &[],
        opcode: 0xc0040c00,
        mask: 0xffff9c78,
        class: InsnClass::SME2_MOV,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME2_MOV(SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx4(
                MOVA_SME_ZA_array_off3_0_SME_Znx4::from(bits),
            )),
        }
    }
}
