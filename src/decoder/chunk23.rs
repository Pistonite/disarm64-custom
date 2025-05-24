#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for LDRH_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78400400,
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
                qualifiers: &[InsnOperandQualifier::S_H],
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
            mnemonic: Mnemonic::r#ldrh,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRH_Rt_ADDR_SIMM9(
                LDRH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRH_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x79400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
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
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
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
            mnemonic: Mnemonic::r#ldrh,
            operation: Operation::LDST_POS(LDST_POS::LDRH_Rt_ADDR_UIMM12(
                LDRH_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRH_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSB_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38a00800,
        mask: 0xffa00c00,
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
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsb,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(
                LDRSB_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSB_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38800400,
        mask: 0xffa00400,
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
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsb,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(
                LDRSB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSB_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x39800000,
        mask: 0xff800000,
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
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsb,
            operation: Operation::LDST_POS(LDST_POS::LDRSB_Rt_ADDR_UIMM12(
                LDRSB_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSB_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSH_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78a00800,
        mask: 0xffa00c00,
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
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsh,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(
                LDRSH_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSH_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78800400,
        mask: 0xffa00400,
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
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsh,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(
                LDRSH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSH_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x79800000,
        mask: 0xff800000,
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
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsh,
            operation: Operation::LDST_POS(LDST_POS::LDRSH_Rt_ADDR_UIMM12(
                LDRSH_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSH_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0x98000000,
        mask: 0xff000000,
        class: InsnClass::LOADLIT,
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
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsw,
            operation: Operation::LOADLIT(LOADLIT::LDRSW_Rt_ADDR_PCREL19(
                LDRSW_Rt_ADDR_PCREL19::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
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
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrsw,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(
                LDRSW_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8800400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
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
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#ldrsw,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(
                LDRSW_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb9800000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
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
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#ldrsw,
            operation: Operation::LDST_POS(LDST_POS::LDRSW_Rt_ADDR_UIMM12(
                LDRSW_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldur",
        aliases: &[],
        opcode: 0xb8400000,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNSCALED,
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
            mnemonic: Mnemonic::r#ldur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(
                LDUR_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUR_Ft_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldur",
        aliases: &[],
        opcode: 0x3c400000,
        mask: 0x3f600c00,
        class: InsnClass::LDST_UNSCALED,
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
            mnemonic: Mnemonic::r#ldur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(
                LDUR_Ft_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUR_Ft_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldurb",
        aliases: &[],
        opcode: 0x38400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
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
            mnemonic: Mnemonic::r#ldurb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(
                LDURB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldurh",
        aliases: &[],
        opcode: 0x78400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
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
                qualifiers: &[InsnOperandQualifier::S_H],
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
            mnemonic: Mnemonic::r#ldurh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(
                LDURH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURSB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldursb",
        aliases: &[],
        opcode: 0x38800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
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
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldursb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(
                LDURSB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURSB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURSH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldursh",
        aliases: &[],
        opcode: 0x78800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
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
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldursh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(
                LDURSH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURSH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURSW_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldursw",
        aliases: &[],
        opcode: 0xb8800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
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
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#ldursw,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(
                LDURSW_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURSW_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsl",
        aliases: &[],
        opcode: 0x4038000,
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
                kind: InsnOperandKind::SVE_SHLIMM_PRED,
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
            mnemonic: Mnemonic::r#lsl,
            operation: Operation::SVE_SHIFT_PRED(
                SVE_SHIFT_PRED::LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED(
                    LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsl",
        aliases: &[],
        opcode: 0x4138000,
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
            mnemonic: Mnemonic::r#lsl,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsl",
        aliases: &[],
        opcode: 0x41b8000,
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
            mnemonic: Mnemonic::r#lsl,
            operation: Operation::SVE_SIZE_BHS(
                SVE_SIZE_BHS::LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(
                    LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LSL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lsl",
        aliases: &[],
        opcode: 0x4208c00,
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
            mnemonic: Mnemonic::r#lsl,
            operation: Operation::SVE_SIZE_BHS(SVE_SIZE_BHS::LSL_SVE_Zd_SVE_Zn_SVE_Zm_16(
                LSL_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
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
