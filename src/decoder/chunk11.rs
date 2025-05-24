#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for BIT_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BL_ADDR_PCREL26 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bl",
        aliases: &[],
        opcode: 0x94000000,
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
            mnemonic: Mnemonic::r#bl,
            operation: Operation::BRANCH_IMM(BRANCH_IMM::BL_ADDR_PCREL26(BL_ADDR_PCREL26::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for BL_ADDR_PCREL26 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BLR_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "blr",
        aliases: &[],
        opcode: 0xd63f0000,
        mask: 0xfffffc1f,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#blr,
            operation: Operation::BRANCH_REG(BRANCH_REG::BLR_Rn(BLR_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for BLR_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BLRAA_Rn_Rd_SP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "blraa",
        aliases: &[],
        opcode: 0xd73f0800,
        mask: 0xfffffc00,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
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
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#blraa,
            operation: Operation::BRANCH_REG(BRANCH_REG::BLRAA_Rn_Rd_SP(BLRAA_Rn_Rd_SP::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for BLRAA_Rn_Rd_SP {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BLRAAZ_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "blraaz",
        aliases: &[],
        opcode: 0xd63f081f,
        mask: 0xfffffc1f,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#blraaz,
            operation: Operation::BRANCH_REG(BRANCH_REG::BLRAAZ_Rn(BLRAAZ_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for BLRAAZ_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BLRAB_Rn_Rd_SP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "blrab",
        aliases: &[],
        opcode: 0xd73f0c00,
        mask: 0xfffffc00,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
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
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#blrab,
            operation: Operation::BRANCH_REG(BRANCH_REG::BLRAB_Rn_Rd_SP(BLRAB_Rn_Rd_SP::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for BLRAB_Rn_Rd_SP {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BLRABZ_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "blrabz",
        aliases: &[],
        opcode: 0xd63f0c1f,
        mask: 0xfffffc1f,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#blrabz,
            operation: Operation::BRANCH_REG(BRANCH_REG::BLRABZ_Rn(BLRABZ_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for BLRABZ_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BR_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "br",
        aliases: &[],
        opcode: 0xd61f0000,
        mask: 0xfffffc1f,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#br,
            operation: Operation::BRANCH_REG(BRANCH_REG::BR_Rn(BR_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for BR_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRAA_Rn_Rd_SP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "braa",
        aliases: &[],
        opcode: 0xd71f0800,
        mask: 0xfffffc00,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
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
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#braa,
            operation: Operation::BRANCH_REG(BRANCH_REG::BRAA_Rn_Rd_SP(BRAA_Rn_Rd_SP::from(bits))),
        }
    }
}
impl InsnOpcode for BRAA_Rn_Rd_SP {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRAAZ_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "braaz",
        aliases: &[],
        opcode: 0xd61f081f,
        mask: 0xfffffc1f,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#braaz,
            operation: Operation::BRANCH_REG(BRANCH_REG::BRAAZ_Rn(BRAAZ_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for BRAAZ_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRAB_Rn_Rd_SP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brab",
        aliases: &[],
        opcode: 0xd71f0c00,
        mask: 0xfffffc00,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
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
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brab,
            operation: Operation::BRANCH_REG(BRANCH_REG::BRAB_Rn_Rd_SP(BRAB_Rn_Rd_SP::from(bits))),
        }
    }
}
impl InsnOpcode for BRAB_Rn_Rd_SP {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRABZ_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brabz",
        aliases: &[],
        opcode: 0xd61f0c1f,
        mask: 0xfffffc1f,
        class: InsnClass::BRANCH_REG,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brabz,
            operation: Operation::BRANCH_REG(BRANCH_REG::BRABZ_Rn(BRABZ_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for BRABZ_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRK_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brk",
        aliases: &[],
        opcode: 0xd4200000,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brk,
            operation: Operation::EXCEPTION(EXCEPTION::BRK_EXCEPTION(BRK_EXCEPTION::from(bits))),
        }
    }
}
impl InsnOpcode for BRK_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brka",
        aliases: &[],
        opcode: 0x25104000,
        mask: 0xffffc200,
        class: InsnClass::SVE_PRED_ZM,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Pd,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pd,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg4_10,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_Z, InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg4_10,
                    lsb: 10,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pn,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pn,
                    lsb: 5,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brka,
            operation: Operation::SVE_PRED_ZM(SVE_PRED_ZM::BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn(
                BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkas",
        aliases: &[],
        opcode: 0x25504000,
        mask: 0xffffc210,
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
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brkas,
            operation: Operation::SVE_MISC(SVE_MISC::BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn(
                BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkb",
        aliases: &[],
        opcode: 0x25904000,
        mask: 0xffffc200,
        class: InsnClass::SVE_PRED_ZM,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Pd,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pd,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg4_10,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::P_Z, InsnOperandQualifier::P_M],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg4_10,
                    lsb: 10,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pn,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pn,
                    lsb: 5,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brkb,
            operation: Operation::SVE_PRED_ZM(SVE_PRED_ZM::BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn(
                BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkbs",
        aliases: &[],
        opcode: 0x25d04000,
        mask: 0xffffc210,
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
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brkbs,
            operation: Operation::SVE_MISC(SVE_MISC::BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn(
                BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkn",
        aliases: &[],
        opcode: 0x25184000,
        mask: 0xffffc210,
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
                kind: InsnOperandKind::SVE_Pd,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pd,
                    lsb: 0,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brkn,
            operation: Operation::SVE_MISC(SVE_MISC::BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(
                BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkns",
        aliases: &[],
        opcode: 0x25584000,
        mask: 0xffffc210,
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
                kind: InsnOperandKind::SVE_Pd,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pd,
                    lsb: 0,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brkns,
            operation: Operation::SVE_MISC(SVE_MISC::BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(
                BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkpa",
        aliases: &[],
        opcode: 0x2500c000,
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
            mnemonic: Mnemonic::r#brkpa,
            operation: Operation::SVE_MISC(SVE_MISC::BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkpas",
        aliases: &[],
        opcode: 0x2540c000,
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
            mnemonic: Mnemonic::r#brkpas,
            operation: Operation::SVE_MISC(SVE_MISC::BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkpb",
        aliases: &[],
        opcode: 0x2500c010,
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
            mnemonic: Mnemonic::r#brkpb,
            operation: Operation::SVE_MISC(SVE_MISC::BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brkpbs",
        aliases: &[],
        opcode: 0x2540c010,
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
            mnemonic: Mnemonic::r#brkpbs,
            operation: Operation::SVE_MISC(SVE_MISC::BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(
                BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CBNZ_Rt_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cbnz",
        aliases: &[],
        opcode: 0x35000000,
        mask: 0x7f000000,
        class: InsnClass::COMPBRANCH,
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#cbnz,
            operation: Operation::COMPBRANCH(COMPBRANCH::CBNZ_Rt_ADDR_PCREL19(
                CBNZ_Rt_ADDR_PCREL19::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CBNZ_Rt_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CBZ_Rt_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cbz",
        aliases: &[],
        opcode: 0x34000000,
        mask: 0x7f000000,
        class: InsnClass::COMPBRANCH,
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#cbz,
            operation: Operation::COMPBRANCH(COMPBRANCH::CBZ_Rt_ADDR_PCREL19(
                CBZ_Rt_ADDR_PCREL19::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CBZ_Rt_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CCMN_Rn_Rm_NZCV_COND {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ccmn",
        aliases: &[],
        opcode: 0x3a400000,
        mask: 0x7fe00c10,
        class: InsnClass::CONDCMP_REG,
        feature_set: InsnFeatureSet::V8,
        operands: &[
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
                kind: InsnOperandKind::NZCV,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::nzcv,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::COND,
                class: InsnOperandClass::COND,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ccmn,
            operation: Operation::CONDCMP_REG(CONDCMP_REG::CCMN_Rn_Rm_NZCV_COND(
                CCMN_Rn_Rm_NZCV_COND::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CCMN_Rn_Rm_NZCV_COND {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CCMN_Rn_CCMP_IMM_NZCV_COND {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ccmn",
        aliases: &[],
        opcode: 0x3a400800,
        mask: 0x7fe00c10,
        class: InsnClass::CONDCMP_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
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
                kind: InsnOperandKind::CCMP_IMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm5,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::NZCV,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::nzcv,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::COND,
                class: InsnOperandClass::COND,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ccmn,
            operation: Operation::CONDCMP_IMM(CONDCMP_IMM::CCMN_Rn_CCMP_IMM_NZCV_COND(
                CCMN_Rn_CCMP_IMM_NZCV_COND::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CCMN_Rn_CCMP_IMM_NZCV_COND {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CCMP_Rn_Rm_NZCV_COND {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ccmp",
        aliases: &[],
        opcode: 0x7a400000,
        mask: 0x7fe00c10,
        class: InsnClass::CONDCMP_REG,
        feature_set: InsnFeatureSet::V8,
        operands: &[
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
                kind: InsnOperandKind::NZCV,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::nzcv,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::COND,
                class: InsnOperandClass::COND,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ccmp,
            operation: Operation::CONDCMP_REG(CONDCMP_REG::CCMP_Rn_Rm_NZCV_COND(
                CCMP_Rn_Rm_NZCV_COND::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CCMP_Rn_Rm_NZCV_COND {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CCMP_Rn_CCMP_IMM_NZCV_COND {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ccmp",
        aliases: &[],
        opcode: 0x7a400800,
        mask: 0x7fe00c10,
        class: InsnClass::CONDCMP_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
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
                kind: InsnOperandKind::CCMP_IMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm5,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::NZCV,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::nzcv,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::COND,
                class: InsnOperandClass::COND,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ccmp,
            operation: Operation::CONDCMP_IMM(CONDCMP_IMM::CCMP_Rn_CCMP_IMM_NZCV_COND(
                CCMP_Rn_CCMP_IMM_NZCV_COND::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CCMP_Rn_CCMP_IMM_NZCV_COND {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cdot",
        aliases: &[],
        opcode: 0x44801000,
        mask: 0xffa0f000,
        class: InsnClass::SVE_SIZE_SD,
        feature_set: InsnFeatureSet::SVE2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_IMM_ROT2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_rot2,
                    lsb: 10,
                    width: 2,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#cdot,
            operation: Operation::SVE_SIZE_SD(
                SVE_SIZE_SD::CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2(
                    CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cdot",
        aliases: &[],
        opcode: 0x44a04000,
        mask: 0xffe0f000,
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
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm3_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_IMM_ROT2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_rot2,
                    lsb: 10,
                    width: 2,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#cdot,
            operation: Operation::SVE_MISC(
                SVE_MISC::CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(
                    CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cdot",
        aliases: &[],
        opcode: 0x44e04000,
        mask: 0xffe0f000,
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
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm4_INDEX,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_IMM_ROT2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_rot2,
                    lsb: 10,
                    width: 2,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#cdot,
            operation: Operation::SVE_MISC(
                SVE_MISC::CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(
                    CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CFINV {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cfinv",
        aliases: &[],
        opcode: 0xd500401f,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: &[],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#cfinv,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::CFINV(CFINV::from(bits))),
        }
    }
}
impl InsnOpcode for CFINV {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CHKFEAT_X16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "chkfeat",
        aliases: &[],
        opcode: 0xd503251f,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::CHK,
        operands: &[InsnOperand {
            kind: InsnOperandKind::X16,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#chkfeat,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::CHKFEAT_X16(CHKFEAT_X16::from(bits))),
        }
    }
}
impl InsnOpcode for CHKFEAT_X16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "clasta",
        aliases: &[],
        opcode: 0x5288000,
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
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
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
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
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
            mnemonic: Mnemonic::r#clasta,
            operation: Operation::SVE_SIZE_BHSD(
                SVE_SIZE_BHSD::CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                    CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "clasta",
        aliases: &[],
        opcode: 0x52a8000,
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
                kind: InsnOperandKind::SVE_Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Vd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
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
            mnemonic: Mnemonic::r#clasta,
            operation: Operation::SVE_SIZE_BHSD(
                SVE_SIZE_BHSD::CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(
                    CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "clasta",
        aliases: &[],
        opcode: 0x530a000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_BHSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
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
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
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
            mnemonic: Mnemonic::r#clasta,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5(
                CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "clastb",
        aliases: &[],
        opcode: 0x5298000,
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
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
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
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
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
            mnemonic: Mnemonic::r#clastb,
            operation: Operation::SVE_SIZE_BHSD(
                SVE_SIZE_BHSD::CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                    CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
                ),
            ),
        }
    }
}
