#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for SDOT_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SDOT_Vd_Vn_Em {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sdot",
        aliases: &[],
        opcode: 0xf00e000,
        mask: 0xbf00f400,
        class: InsnClass::DOTPRODUCT,
        feature_set: InsnFeatureSet::DOTPROD,
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
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_4B, InsnOperandQualifier::S_4B],
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
            mnemonic: Mnemonic::r#sdot,
            operation: Operation::DOTPRODUCT(DOTPRODUCT::SDOT_Vd_Vn_Em(SDOT_Vd_Vn_Em::from(bits))),
        }
    }
}
impl InsnOpcode for SDOT_Vd_Vn_Em {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMADDL_Rd_Rn_Rm_Ra {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smaddl",
        aliases: &[],
        opcode: 0x9b200000,
        mask: 0xffe08000,
        class: InsnClass::DP_3SRC,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ra,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Ra,
                    lsb: 10,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#smaddl,
            operation: Operation::DP_3SRC(DP_3SRC::SMADDL_Rd_Rn_Rm_Ra(SMADDL_Rd_Rn_Rm_Ra::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for SMADDL_Rd_Rn_Rm_Ra {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULH_Rd_Rn_Rm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smulh",
        aliases: &[],
        opcode: 0x9b407c00,
        mask: 0xffe0fc00,
        class: InsnClass::DP_3SRC,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#smulh,
            operation: Operation::DP_3SRC(DP_3SRC::SMULH_Rd_Rn_Rm(SMULH_Rd_Rn_Rm::from(bits))),
        }
    }
}
impl InsnOpcode for SMULH_Rd_Rn_Rm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smulh",
        aliases: &[],
        opcode: 0x4120000,
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
            mnemonic: Mnemonic::r#smulh,
            operation: Operation::SVE_SIZE_BHSD(
                SVE_SIZE_BHSD::SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                    SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smulh",
        aliases: &[],
        opcode: 0x4206800,
        mask: 0xff20fc00,
        class: InsnClass::SVE_SIZE_BHSD,
        feature_set: InsnFeatureSet::SVE2,
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
                kind: InsnOperandKind::SVE_Zm_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
            mnemonic: Mnemonic::r#smulh,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16(
                SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULL_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smull",
        aliases: &[],
        opcode: 0xe20c000,
        mask: 0xff20fc00,
        class: InsnClass::ASIMDDIFF,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_2S,
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
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_2S,
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
            mnemonic: Mnemonic::r#smull,
            operation: Operation::ASIMDDIFF(ASIMDDIFF::SMULL_Vd_Vn_Vm(SMULL_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for SMULL_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULL_Vd_Vn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smull",
        aliases: &[],
        opcode: 0xf00a000,
        mask: 0xff00f400,
        class: InsnClass::ASIMDELEM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S, InsnOperandQualifier::V_2D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_2S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em16,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#smull,
            operation: Operation::ASIMDELEM(ASIMDELEM::SMULL_Vd_Vn_Em16(SMULL_Vd_Vn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for SMULL_Vd_Vn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULL2_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smull2",
        aliases: &[],
        opcode: 0x4e20c000,
        mask: 0xff20fc00,
        class: InsnClass::ASIMDDIFF,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_4S,
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
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_4S,
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
            mnemonic: Mnemonic::r#smull2,
            operation: Operation::ASIMDDIFF(ASIMDDIFF::SMULL2_Vd_Vn_Vm(SMULL2_Vd_Vn_Vm::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for SMULL2_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULL2_Vd_Vn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smull2",
        aliases: &[],
        opcode: 0x4f00a000,
        mask: 0xff00f400,
        class: InsnClass::ASIMDELEM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S, InsnOperandQualifier::V_2D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8H, InsnOperandQualifier::V_4S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em16,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#smull2,
            operation: Operation::ASIMDELEM(ASIMDELEM::SMULL2_Vd_Vn_Em16(SMULL2_Vd_Vn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for SMULL2_Vd_Vn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smullb",
        aliases: &[],
        opcode: 0x44a0c000,
        mask: 0xffe0f400,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE2,
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
                kind: InsnOperandKind::SVE_Zm3_11_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i3h2,
                        lsb: 19,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i3l,
                        lsb: 11,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_imm3,
                        lsb: 16,
                        width: 3,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#smullb,
            operation: Operation::SVE_MISC(SVE_MISC::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(
                SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smullb",
        aliases: &[],
        opcode: 0x44e0c000,
        mask: 0xffe0f400,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE2,
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
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm4_11_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i2h,
                        lsb: 20,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i3l,
                        lsb: 11,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_imm4,
                        lsb: 16,
                        width: 4,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#smullb,
            operation: Operation::SVE_MISC(SVE_MISC::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(
                SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smullb",
        aliases: &[],
        opcode: 0x45007000,
        mask: 0xff20fc00,
        class: InsnClass::SVE_SIZE_HSD,
        feature_set: InsnFeatureSet::SVE2,
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
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
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
            mnemonic: Mnemonic::r#smullb,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16(
                SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smullt",
        aliases: &[],
        opcode: 0x44a0c400,
        mask: 0xffe0f400,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE2,
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
                kind: InsnOperandKind::SVE_Zm3_11_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i3h2,
                        lsb: 19,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i3l,
                        lsb: 11,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_imm3,
                        lsb: 16,
                        width: 3,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#smullt,
            operation: Operation::SVE_MISC(SVE_MISC::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(
                SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smullt",
        aliases: &[],
        opcode: 0x44e0c400,
        mask: 0xffe0f400,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE2,
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
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm4_11_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i2h,
                        lsb: 20,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_i3l,
                        lsb: 11,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::SVE_imm4,
                        lsb: 16,
                        width: 4,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#smullt,
            operation: Operation::SVE_MISC(SVE_MISC::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(
                SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smullt",
        aliases: &[],
        opcode: 0x45007400,
        mask: 0xff20fc00,
        class: InsnClass::SVE_SIZE_HSD,
        feature_set: InsnFeatureSet::SVE2,
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
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
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
            mnemonic: Mnemonic::r#smullt,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16(
                SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x29000000,
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
            mnemonic: Mnemonic::r#stp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(
                STP_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STP_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x28800000,
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
            mnemonic: Mnemonic::r#stp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(
                    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Ft_Ft2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2d000000,
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
            mnemonic: Mnemonic::r#stp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(
                STP_Ft_Ft2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STP_Ft_Ft2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2c800000,
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
            mnemonic: Mnemonic::r#stp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(
                    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xe1200000,
        mask: 0xffff9c10,
        class: InsnClass::SME_STR,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::SME_STR(SME_STR::STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(
                STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_SME_ZT0_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xe13f8000,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::SME_MISC(SME_MISC::STR_SME_ZT0_SIMD_ADDR_SIMPLE(
                STR_SME_ZT0_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_SME_ZT0_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8200800,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::STR_Rt_ADDR_REGOFF(
                STR_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8000400,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::LDST_IMM9(LDST_IMM9::STR_Rt_ADDR_SIMM9(STR_Rt_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb9000000,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::LDST_POS(LDST_POS::STR_Rt_ADDR_UIMM12(STR_Rt_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_SVE_Pt_SVE_ADDR_RI_S9xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xe5800000,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::SVE_MISC(SVE_MISC::STR_SVE_Pt_SVE_ADDR_RI_S9xVL(
                STR_SVE_Pt_SVE_ADDR_RI_S9xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_SVE_Pt_SVE_ADDR_RI_S9xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_SVE_PNt_SVE_ADDR_RI_S9xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xe5800000,
        mask: 0xffc0e010,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_PNt,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::SVE_MISC(SVE_MISC::STR_SVE_PNt_SVE_ADDR_RI_S9xVL(
                STR_SVE_PNt_SVE_ADDR_RI_S9xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_SVE_PNt_SVE_ADDR_RI_S9xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_SVE_Zt_SVE_ADDR_RI_S9xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xe5804000,
        mask: 0xffc0e000,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zt,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zt,
                    lsb: 0,
                    width: 5,
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
            mnemonic: Mnemonic::r#str,
            operation: Operation::SVE_MISC(SVE_MISC::STR_SVE_Zt_SVE_ADDR_RI_S9xVL(
                STR_SVE_Zt_SVE_ADDR_RI_S9xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_SVE_Zt_SVE_ADDR_RI_S9xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Ft_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c200800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
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
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#str,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::STR_Ft_ADDR_REGOFF(
                STR_Ft_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_Ft_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Ft_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c000400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
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
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#str,
            operation: Operation::LDST_IMM9(LDST_IMM9::STR_Ft_ADDR_SIMM9(STR_Ft_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Ft_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Ft_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3d000000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
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
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#str,
            operation: Operation::LDST_POS(LDST_POS::STR_Ft_ADDR_UIMM12(STR_Ft_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Ft_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRB_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
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
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#strb,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::STRB_Rt_ADDR_REGOFF(
                STRB_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STRB_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38000400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
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
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
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
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#strb,
            operation: Operation::LDST_IMM9(LDST_IMM9::STRB_Rt_ADDR_SIMM9(
                STRB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
