#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0c60200,
        mask: 0xffff1f01,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrsd_1,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_D],
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
                        bitfield: InsnBitField::ZAn_3,
                        lsb: 5,
                        width: 3,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movaz,
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1(
                MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0060600,
        mask: 0xffff1f83,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrsb_2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_B],
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
                        bitfield: InsnBitField::off2,
                        lsb: 5,
                        width: 2,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movaz,
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2(
                MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0860600,
        mask: 0xffff1f83,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrss_2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
                        bitfield: InsnBitField::off2,
                        lsb: 5,
                        width: 2,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movaz,
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2(
                MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0460600,
        mask: 0xffff1f83,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrsh_2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_H],
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
                        bitfield: InsnBitField::ZAn,
                        lsb: 6,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::ol,
                        lsb: 5,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movaz,
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2(
                MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0c60600,
        mask: 0xffff1f03,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrsd_2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_D],
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
                        bitfield: InsnBitField::ZAn_3,
                        lsb: 5,
                        width: 3,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movaz,
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2(
                MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVI_Vd_SIMD_IMM_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movi",
        aliases: &[],
        opcode: 0xf000400,
        mask: 0xbff89c00,
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
                kind: InsnOperandKind::SIMD_IMM_SFT,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::LSL, InsnOperandQualifier::LSL],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movi,
            operation: Operation::ASIMDIMM(ASIMDIMM::MOVI_Vd_SIMD_IMM_SFT(
                MOVI_Vd_SIMD_IMM_SFT::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVI_Vd_SIMD_IMM_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movi",
        aliases: &[],
        opcode: 0xf008400,
        mask: 0xbff8dc00,
        class: InsnClass::ASIMDIMM,
        feature_set: InsnFeatureSet::SIMD,
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
                kind: InsnOperandKind::SIMD_IMM_SFT,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::LSL, InsnOperandQualifier::LSL],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movi,
            operation: Operation::ASIMDIMM(ASIMDIMM::MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL(
                MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movi",
        aliases: &[],
        opcode: 0xf00c400,
        mask: 0xbff8ec00,
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
                kind: InsnOperandKind::SIMD_IMM_SFT,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::MSL, InsnOperandQualifier::MSL],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movi,
            operation: Operation::ASIMDIMM(ASIMDIMM::MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL(
                MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movi",
        aliases: &[],
        opcode: 0xf00e400,
        mask: 0xbff8fc00,
        class: InsnClass::ASIMDIMM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_IMM_SFT,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::LSL, InsnOperandQualifier::LSL],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movi,
            operation: Operation::ASIMDIMM(ASIMDIMM::MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL(
                MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVI_Sd_SIMD_IMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movi",
        aliases: &[],
        opcode: 0x2f00e400,
        mask: 0xfff8fc00,
        class: InsnClass::ASIMDIMM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_IMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movi,
            operation: Operation::ASIMDIMM(ASIMDIMM::MOVI_Sd_SIMD_IMM(MOVI_Sd_SIMD_IMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for MOVI_Sd_SIMD_IMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVI_Vd_SIMD_IMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movi",
        aliases: &[],
        opcode: 0x6f00e400,
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
                kind: InsnOperandKind::SIMD_IMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movi,
            operation: Operation::ASIMDIMM(ASIMDIMM::MOVI_Vd_SIMD_IMM(MOVI_Vd_SIMD_IMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for MOVI_Vd_SIMD_IMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVK_Rd_HALF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movk",
        aliases: &[],
        opcode: 0x72800000,
        mask: 0x7f800000,
        class: InsnClass::MOVEWIDE,
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
                kind: InsnOperandKind::HALF,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm16_5,
                    lsb: 5,
                    width: 16,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movk,
            operation: Operation::MOVEWIDE(MOVEWIDE::MOVK_Rd_HALF(MOVK_Rd_HALF::from(bits))),
        }
    }
}
impl InsnOpcode for MOVK_Rd_HALF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVN_Rd_HALF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movn",
        aliases: &[],
        opcode: 0x12800000,
        mask: 0x7f800000,
        class: InsnClass::MOVEWIDE,
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
                kind: InsnOperandKind::HALF,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm16_5,
                    lsb: 5,
                    width: 16,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movn,
            operation: Operation::MOVEWIDE(MOVEWIDE::MOVN_Rd_HALF(MOVN_Rd_HALF::from(bits))),
        }
    }
}
impl InsnOpcode for MOVN_Rd_HALF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movprfx",
        aliases: &[],
        opcode: 0x4102000,
        mask: 0xff3ee000,
        class: InsnClass::SVE_MOVPRFX,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
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
                    InsnOperandQualifier::P_Z,
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_Z,
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_Z,
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_Z,
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
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::RESTRICTED_NEXT_INSN_SET.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movprfx,
            operation: Operation::SVE_MOVPRFX(SVE_MOVPRFX::MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn(
                MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVPRFX_SVE_Zd_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movprfx",
        aliases: &[],
        opcode: 0x420bc00,
        mask: 0xfffffc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
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
        flags: InsnFlags::const_from_bits(InsnFlags::RESTRICTED_NEXT_INSN_SET.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movprfx,
            operation: Operation::SVE_MISC(SVE_MISC::MOVPRFX_SVE_Zd_SVE_Zn(
                MOVPRFX_SVE_Zd_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVPRFX_SVE_Zd_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVT_Rt_SME_ZT0_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movt",
        aliases: &[],
        opcode: 0xc04c03e0,
        mask: 0xffff8fe0,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
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
                kind: InsnOperandKind::SME_ZT0_INDEX,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm3_12,
                    lsb: 12,
                    width: 3,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movt,
            operation: Operation::SME_MISC(SME_MISC::MOVT_Rt_SME_ZT0_INDEX(
                MOVT_Rt_SME_ZT0_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVT_Rt_SME_ZT0_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVT_SME_ZT0_INDEX_Rt {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movt",
        aliases: &[],
        opcode: 0xc04e03e0,
        mask: 0xffff8fe0,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZT0_INDEX,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm3_12,
                    lsb: 12,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movt,
            operation: Operation::SME_MISC(SME_MISC::MOVT_SME_ZT0_INDEX_Rt(
                MOVT_SME_ZT0_INDEX_Rt::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVT_SME_ZT0_INDEX_Rt {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVZ_Rd_HALF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movz",
        aliases: &[],
        opcode: 0x52800000,
        mask: 0x7f800000,
        class: InsnClass::MOVEWIDE,
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
                kind: InsnOperandKind::HALF,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm16_5,
                    lsb: 5,
                    width: 16,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#movz,
            operation: Operation::MOVEWIDE(MOVEWIDE::MOVZ_Rd_HALF(MOVZ_Rd_HALF::from(bits))),
        }
    }
}
impl InsnOpcode for MOVZ_Rd_HALF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MSUB_Rd_Rn_Rm_Ra {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "msub",
        aliases: &[],
        opcode: 0x1b008000,
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
            mnemonic: Mnemonic::r#msub,
            operation: Operation::DP_3SRC(DP_3SRC::MSUB_Rd_Rn_Rm_Ra(MSUB_Rd_Rn_Rm_Ra::from(bits))),
        }
    }
}
impl InsnOpcode for MSUB_Rd_Rn_Rm_Ra {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0x4100000,
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
            mnemonic: Mnemonic::r#mul,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0x4206000,
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
            mnemonic: Mnemonic::r#mul,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Zn_SVE_Zm_16(
                MUL_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0x4420f800,
        mask: 0xffa0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE2,
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
            mnemonic: Mnemonic::r#mul,
            operation: Operation::SVE_MISC(SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(
                MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0x44a0f800,
        mask: 0xffe0fc00,
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
            mnemonic: Mnemonic::r#mul,
            operation: Operation::SVE_MISC(SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(
                MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0x44e0f800,
        mask: 0xffe0fc00,
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
            mnemonic: Mnemonic::r#mul,
            operation: Operation::SVE_MISC(SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(
                MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_SVE_Zd_SVE_Zd_SVE_SIMM8 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0x2530c000,
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
                kind: InsnOperandKind::SVE_SIMM8,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_imm8,
                    lsb: 5,
                    width: 8,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#mul,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Zd_SVE_SIMM8(
                MUL_SVE_Zd_SVE_Zd_SVE_SIMM8::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MUL_SVE_Zd_SVE_Zd_SVE_SIMM8 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0xe209c00,
        mask: 0xbf20fc00,
        class: InsnClass::ASIMDSAME,
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
            mnemonic: Mnemonic::r#mul,
            operation: Operation::ASIMDSAME(ASIMDSAME::MUL_Vd_Vn_Vm(MUL_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for MUL_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MUL_Vd_Vn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mul",
        aliases: &[],
        opcode: 0xf008000,
        mask: 0xbf00f400,
        class: InsnClass::ASIMDELEM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
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
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em16,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_S,
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
            mnemonic: Mnemonic::r#mul,
            operation: Operation::ASIMDELEM(ASIMDELEM::MUL_Vd_Vn_Em16(MUL_Vd_Vn_Em16::from(bits))),
        }
    }
}
impl InsnOpcode for MUL_Vd_Vn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MVNI_Vd_SIMD_IMM_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mvni",
        aliases: &[],
        opcode: 0x2f000400,
        mask: 0xbff89c00,
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
                kind: InsnOperandKind::SIMD_IMM_SFT,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::LSL, InsnOperandQualifier::LSL],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#mvni,
            operation: Operation::ASIMDIMM(ASIMDIMM::MVNI_Vd_SIMD_IMM_SFT(
                MVNI_Vd_SIMD_IMM_SFT::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MVNI_Vd_SIMD_IMM_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mvni",
        aliases: &[],
        opcode: 0x2f008400,
        mask: 0xbff8dc00,
        class: InsnClass::ASIMDIMM,
        feature_set: InsnFeatureSet::SIMD,
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
                kind: InsnOperandKind::SIMD_IMM_SFT,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::LSL, InsnOperandQualifier::LSL],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#mvni,
            operation: Operation::ASIMDIMM(ASIMDIMM::MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL(
                MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mvni",
        aliases: &[],
        opcode: 0x2f00c400,
        mask: 0xbff8ec00,
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
                kind: InsnOperandKind::SIMD_IMM_SFT,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::MSL, InsnOperandQualifier::MSL],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#mvni,
            operation: Operation::ASIMDIMM(ASIMDIMM::MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL(
                MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl NEG_SVE_Zd_SVE_Pg3_SVE_Zn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "neg",
        aliases: &[],
        opcode: 0x417a000,
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
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#neg,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::NEG_SVE_Zd_SVE_Pg3_SVE_Zn(
                NEG_SVE_Zd_SVE_Pg3_SVE_Zn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for NEG_SVE_Zd_SVE_Pg3_SVE_Zn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl NEG_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "neg",
        aliases: &[],
        opcode: 0x2e20b800,
        mask: 0xbf3ffc00,
        class: InsnClass::ASIMDMISC,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#neg,
            operation: Operation::ASIMDMISC(ASIMDMISC::NEG_Vd_Vn(NEG_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for NEG_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl NEG_Sd_Sn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "neg",
        aliases: &[],
        opcode: 0x7e20b800,
        mask: 0xff3ffc00,
        class: InsnClass::ASISDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
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
            mnemonic: Mnemonic::r#neg,
            operation: Operation::ASISDMISC(ASISDMISC::NEG_Sd_Sn(NEG_Sd_Sn::from(bits))),
        }
    }
}
impl InsnOpcode for NEG_Sd_Sn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ORN_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "orn",
        aliases: &[],
        opcode: 0x2a200000,
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
            mnemonic: Mnemonic::r#orn,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(ORN_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ORN_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "orn",
        aliases: &[],
        opcode: 0x25804010,
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
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#orn,
            operation: Operation::SVE_MISC(SVE_MISC::ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
