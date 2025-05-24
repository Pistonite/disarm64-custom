#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for MOVA_SME_ZA_array_off3_0_SME_Znx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME_MOV(SME_MOV::MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(
                MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN(
                MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_Zdnx2_SME_ZA_array_off3_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME2_MOV(SME2_MOV::MOVA_SME_Zdnx2_SME_ZA_array_off3_5(
                MOVA_SME_Zdnx2_SME_ZA_array_off3_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_Zdnx2_SME_ZA_array_off3_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN(
                MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVA_SME_Zdnx4_SME_ZA_array_off3_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mova",
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
            mnemonic: Mnemonic::r#mova,
            operation: Operation::SME2_MOV(SME2_MOV::MOVA_SME_Zdnx4_SME_ZA_array_off3_5(
                MOVA_SME_Zdnx4_SME_ZA_array_off3_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVA_SME_Zdnx4_SME_ZA_array_off3_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0060200,
        mask: 0xffff1f01,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrsb_1,
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
                        bitfield: InsnBitField::off3,
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
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1(
                MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0860200,
        mask: 0xffff1f01,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrss_1,
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
                        bitfield: InsnBitField::ZAn_2,
                        lsb: 6,
                        width: 2,
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
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1(
                MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "movaz",
        aliases: &[],
        opcode: 0xc0460200,
        mask: 0xffff1f01,
        class: InsnClass::SME2_MOVAZ,
        feature_set: InsnFeatureSet::SME2P1,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_vrsh_1,
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
                        bitfield: InsnBitField::ZAn_1,
                        lsb: 7,
                        width: 1,
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
            operation: Operation::SME2_MOVAZ(SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1(
                MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
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
