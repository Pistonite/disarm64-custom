#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlsl",
        aliases: &[],
        opcode: 0xc1909018,
        mask: 0xfff09078,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off2x2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm2_0,
                        lsb: 0,
                        width: 2,
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
                kind: InsnOperandKind::SME_Zm_INDEX3_2,
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
                    BitfieldSpec {
                        bitfield: InsnBitField::imm1_2,
                        lsb: 2,
                        width: 1,
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
            mnemonic: Mnemonic::r#bfmlsl,
            operation: Operation::SME_MISC(
                SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(
                    BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlsl",
        aliases: &[],
        opcode: 0xc1200818,
        mask: 0xfff09c1c,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off2x2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm2_0,
                        lsb: 0,
                        width: 2,
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
            mnemonic: Mnemonic::r#bfmlsl,
            operation: Operation::SME_MISC(SME_MISC::BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(
                BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlsl",
        aliases: &[],
        opcode: 0xc1300818,
        mask: 0xfff09c1c,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off2x2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm2_0,
                        lsb: 0,
                        width: 2,
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
            mnemonic: Mnemonic::r#bfmlsl,
            operation: Operation::SME_MISC(
                SME_MISC::BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(
                    BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlsl",
        aliases: &[],
        opcode: 0xc1a00818,
        mask: 0xffe19c3c,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off2x2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm2_0,
                        lsb: 0,
                        width: 2,
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
            mnemonic: Mnemonic::r#bfmlsl,
            operation: Operation::SME_MISC(SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(
                BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlsl",
        aliases: &[],
        opcode: 0xc1a10818,
        mask: 0xffe39c7c,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off2x2,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm2_0,
                        lsb: 0,
                        width: 2,
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
            mnemonic: Mnemonic::r#bfmlsl,
            operation: Operation::SME_MISC(SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(
                BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlsl",
        aliases: &[],
        opcode: 0xc1200c18,
        mask: 0xfff09c18,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3x2,
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
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#bfmlsl,
            operation: Operation::SME_MISC(SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(
                BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlslb",
        aliases: &[],
        opcode: 0x64e0a000,
        mask: 0xffe0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SME2,
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
            mnemonic: Mnemonic::r#bfmlslb,
            operation: Operation::SVE_MISC(SVE_MISC::BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlslb",
        aliases: &[],
        opcode: 0x64e06000,
        mask: 0xffe0f400,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SME2,
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
            mnemonic: Mnemonic::r#bfmlslb,
            operation: Operation::SVE_MISC(SVE_MISC::BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(
                BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlslt",
        aliases: &[],
        opcode: 0x64e0a400,
        mask: 0xffe0fc00,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SME2,
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
            mnemonic: Mnemonic::r#bfmlslt,
            operation: Operation::SVE_MISC(SVE_MISC::BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmlslt",
        aliases: &[],
        opcode: 0x64e06400,
        mask: 0xffe0f400,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SME2,
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
            mnemonic: Mnemonic::r#bfmlslt,
            operation: Operation::SVE_MISC(SVE_MISC::BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(
                BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmmla",
        aliases: &[],
        opcode: 0x6460e400,
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
            mnemonic: Mnemonic::r#bfmmla,
            operation: Operation::SVE_MISC(SVE_MISC::BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMMLA_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmmla",
        aliases: &[],
        opcode: 0x6e40ec00,
        mask: 0xffe0fc00,
        class: InsnClass::BFLOAT16,
        feature_set: InsnFeatureSet::BFLOAT16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8H],
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
            mnemonic: Mnemonic::r#bfmmla,
            operation: Operation::BFLOAT16(BFLOAT16::BFMMLA_Vd_Vn_Vm(BFMMLA_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for BFMMLA_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmopa",
        aliases: &[],
        opcode: 0x81800000,
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
            mnemonic: Mnemonic::r#bfmopa,
            operation: Operation::SME_MISC(
                SME_MISC::BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
                    BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmops",
        aliases: &[],
        opcode: 0x81800010,
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
            mnemonic: Mnemonic::r#bfmops,
            operation: Operation::SME_MISC(
                SME_MISC::BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
                    BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmul",
        aliases: &[],
        opcode: 0x64202800,
        mask: 0xffa0fc00,
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
            mnemonic: Mnemonic::r#bfmul,
            operation: Operation::SVE_MISC(SVE_MISC::BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(
                BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmul",
        aliases: &[],
        opcode: 0x65000800,
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
            mnemonic: Mnemonic::r#bfmul,
            operation: Operation::SVE_MISC(SVE_MISC::BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfmul",
        aliases: &[],
        opcode: 0x65028000,
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
            mnemonic: Mnemonic::r#bfmul,
            operation: Operation::SVE_MISC(SVE_MISC::BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfsub",
        aliases: &[],
        opcode: 0x65000400,
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
            mnemonic: Mnemonic::r#bfsub,
            operation: Operation::SVE_MISC(SVE_MISC::BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfsub",
        aliases: &[],
        opcode: 0x65018000,
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
            mnemonic: Mnemonic::r#bfsub,
            operation: Operation::SVE_MISC(SVE_MISC::BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bfvdot",
        aliases: &[],
        opcode: 0xc1500018,
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
            mnemonic: Mnemonic::r#bfvdot,
            operation: Operation::SME_MISC(
                SME_MISC::BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(
                    BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0xa200000,
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#bic,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(BIC_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for BIC_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0x41b0000,
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
            mnemonic: Mnemonic::r#bic,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0x4e03000,
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
            mnemonic: Mnemonic::r#bic,
            operation: Operation::SVE_MISC(SVE_MISC::BIC_SVE_Zd_SVE_Zn_SVE_Zm_16(
                BIC_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BIC_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0x25004010,
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
            mnemonic: Mnemonic::r#bic,
            operation: Operation::SVE_MISC(SVE_MISC::BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0xe601c00,
        mask: 0xbfe0fc00,
        class: InsnClass::ASIMDSAME,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
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
            mnemonic: Mnemonic::r#bic,
            operation: Operation::ASIMDSAME(ASIMDSAME::BIC_Vd_Vn_Vm(BIC_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for BIC_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_Vd_SIMD_IMM_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0x2f001400,
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
            mnemonic: Mnemonic::r#bic,
            operation: Operation::ASIMDIMM(ASIMDIMM::BIC_Vd_SIMD_IMM_SFT(
                BIC_Vd_SIMD_IMM_SFT::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BIC_Vd_SIMD_IMM_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_Vd_V_4H_SIMD_IMM_SFT_LSL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0x2f009400,
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
            mnemonic: Mnemonic::r#bic,
            operation: Operation::ASIMDIMM(ASIMDIMM::BIC_Vd_V_4H_SIMD_IMM_SFT_LSL(
                BIC_Vd_V_4H_SIMD_IMM_SFT_LSL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BIC_Vd_V_4H_SIMD_IMM_SFT_LSL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BICS_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bics",
        aliases: &[],
        opcode: 0x6a200000,
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#bics,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(BICS_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for BICS_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bics",
        aliases: &[],
        opcode: 0x25404010,
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
            mnemonic: Mnemonic::r#bics,
            operation: Operation::SVE_MISC(SVE_MISC::BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIF_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bif",
        aliases: &[],
        opcode: 0x2ee01c00,
        mask: 0xbfe0fc00,
        class: InsnClass::ASIMDSAME,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
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
            mnemonic: Mnemonic::r#bif,
            operation: Operation::ASIMDSAME(ASIMDSAME::BIF_Vd_Vn_Vm(BIF_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for BIF_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIT_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bit",
        aliases: &[],
        opcode: 0x2ea01c00,
        mask: 0xbfe0fc00,
        class: InsnClass::ASIMDSAME,
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
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
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
            mnemonic: Mnemonic::r#bit,
            operation: Operation::ASIMDSAME(ASIMDSAME::BIT_Vd_Vn_Vm(BIT_Vd_Vn_Vm::from(bits))),
        }
    }
}
