#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for SSUBL_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SSUBL2_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ssubl2",
        aliases: &[],
        opcode: 0x4e202000,
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
            mnemonic: Mnemonic::r#ssubl2,
            operation: Operation::ASIMDDIFF(ASIMDDIFF::SSUBL2_Vd_Vn_Vm(SSUBL2_Vd_Vn_Vm::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for SSUBL2_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SSUBW_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ssubw",
        aliases: &[],
        opcode: 0xe203000,
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
                    InsnOperandQualifier::V_8H,
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
            mnemonic: Mnemonic::r#ssubw,
            operation: Operation::ASIMDDIFF(ASIMDDIFF::SSUBW_Vd_Vn_Vm(SSUBW_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for SSUBW_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SSUBW2_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ssubw2",
        aliases: &[],
        opcode: 0x4e203000,
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
                    InsnOperandQualifier::V_8H,
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
            mnemonic: Mnemonic::r#ssubw2,
            operation: Operation::ASIMDDIFF(ASIMDDIFF::SSUBW2_Vd_Vn_Vm(SSUBW2_Vd_Vn_Vm::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for SSUBW2_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST1_LVt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st1",
        aliases: &[],
        opcode: 0xc000000,
        mask: 0xbfff0000,
        class: InsnClass::ASISDLSE,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st1,
            operation: Operation::ASISDLSE(ASISDLSE::ST1_LVt_SIMD_ADDR_SIMPLE(
                ST1_LVt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST1_LVt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST1_LEt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st1",
        aliases: &[],
        opcode: 0xd000000,
        mask: 0xbfff2000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st1,
            operation: Operation::ASISDLSO(ASISDLSO::ST1_LEt_SIMD_ADDR_SIMPLE(
                ST1_LEt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST1_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST1_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st1",
        aliases: &[],
        opcode: 0xc800000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st1,
            operation: Operation::ASISDLSEP(ASISDLSEP::ST1_LVt_SIMD_ADDR_POST(
                ST1_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST1_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST1_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st1",
        aliases: &[],
        opcode: 0xd800000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st1,
            operation: Operation::ASISDLSOP(ASISDLSOP::ST1_LEt_SIMD_ADDR_POST(
                ST1_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST1_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2_LVt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2",
        aliases: &[],
        opcode: 0xc000000,
        mask: 0xbfff0000,
        class: InsnClass::ASISDLSE,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st2,
            operation: Operation::ASISDLSE(ASISDLSE::ST2_LVt_SIMD_ADDR_SIMPLE(
                ST2_LVt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2_LVt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2_LEt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2",
        aliases: &[],
        opcode: 0xd200000,
        mask: 0xbfff2000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
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
            mnemonic: Mnemonic::r#st2,
            operation: Operation::ASISDLSO(ASISDLSO::ST2_LEt_SIMD_ADDR_SIMPLE(
                ST2_LEt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2",
        aliases: &[],
        opcode: 0xc800000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st2,
            operation: Operation::ASISDLSEP(ASISDLSEP::ST2_LVt_SIMD_ADDR_POST(
                ST2_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2",
        aliases: &[],
        opcode: 0xda00000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
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
            mnemonic: Mnemonic::r#st2,
            operation: Operation::ASISDLSOP(ASISDLSOP::ST2_LEt_SIMD_ADDR_POST(
                ST2_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2G_Rt_SP_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag],
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
            mnemonic: Mnemonic::r#st2g,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::ST2G_Rt_SP_ADDR_SIMM13(
                ST2G_Rt_SP_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2G_Rt_SP_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag],
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
            mnemonic: Mnemonic::r#st2g,
            operation: Operation::LDST_IMM9(LDST_IMM9::ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(
                ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST3_LVt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st3",
        aliases: &[],
        opcode: 0xc000000,
        mask: 0xbfff0000,
        class: InsnClass::ASISDLSE,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st3,
            operation: Operation::ASISDLSE(ASISDLSE::ST3_LVt_SIMD_ADDR_SIMPLE(
                ST3_LVt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST3_LVt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST3_LEt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st3",
        aliases: &[],
        opcode: 0xd002000,
        mask: 0xbfff2000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st3,
            operation: Operation::ASISDLSO(ASISDLSO::ST3_LEt_SIMD_ADDR_SIMPLE(
                ST3_LEt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST3_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST3_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st3",
        aliases: &[],
        opcode: 0xc800000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st3,
            operation: Operation::ASISDLSEP(ASISDLSEP::ST3_LVt_SIMD_ADDR_POST(
                ST3_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST3_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST3_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st3",
        aliases: &[],
        opcode: 0xd802000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st3,
            operation: Operation::ASISDLSOP(ASISDLSOP::ST3_LEt_SIMD_ADDR_POST(
                ST3_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST3_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST4_LVt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st4",
        aliases: &[],
        opcode: 0xc000000,
        mask: 0xbfff0000,
        class: InsnClass::ASISDLSE,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st4,
            operation: Operation::ASISDLSE(ASISDLSE::ST4_LVt_SIMD_ADDR_SIMPLE(
                ST4_LVt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST4_LVt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST4_LEt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st4",
        aliases: &[],
        opcode: 0xd202000,
        mask: 0xbfff2000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
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
            mnemonic: Mnemonic::r#st4,
            operation: Operation::ASISDLSO(ASISDLSO::ST4_LEt_SIMD_ADDR_SIMPLE(
                ST4_LEt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST4_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST4_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st4",
        aliases: &[],
        opcode: 0xc800000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#st4,
            operation: Operation::ASISDLSEP(ASISDLSEP::ST4_LVt_SIMD_ADDR_POST(
                ST4_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST4_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST4_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st4",
        aliases: &[],
        opcode: 0xda02000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
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
            mnemonic: Mnemonic::r#st4,
            operation: Operation::ASISDLSOP(ASISDLSOP::ST4_LEt_SIMD_ADDR_POST(
                ST4_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST4_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STG_Rt_SP_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag],
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
            mnemonic: Mnemonic::r#stg,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STG_Rt_SP_ADDR_SIMM13(
                STG_Rt_SP_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STG_Rt_SP_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag],
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
            mnemonic: Mnemonic::r#stg,
            operation: Operation::LDST_IMM9(LDST_IMM9::STG_Rt_SP_X_ADDR_SIMM13_imm_tag(
                STG_Rt_SP_X_ADDR_SIMM13_imm_tag::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STGM_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stgm",
        aliases: &[],
        opcode: 0xd9a00000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
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
            mnemonic: Mnemonic::r#stgm,
            operation: Operation::LDSTEXCL(LDSTEXCL::STGM_Rt_ADDR_SIMPLE(
                STGM_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STGM_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STGP_Rt_Rt2_ADDR_SIMM11 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x69000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::MEMTAG,
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
                kind: InsnOperandKind::ADDR_SIMM11,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
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
            mnemonic: Mnemonic::r#stgp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::STGP_Rt_Rt2_ADDR_SIMM11(
                STGP_Rt_Rt2_ADDR_SIMM11::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STGP_Rt_Rt2_ADDR_SIMM11 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x68800000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::MEMTAG,
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
                kind: InsnOperandKind::ADDR_SIMM11,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
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
            mnemonic: Mnemonic::r#stgp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(
                    STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLLR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stllr",
        aliases: &[],
        opcode: 0x889f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
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
            mnemonic: Mnemonic::r#stllr,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLLR_Rt_ADDR_SIMPLE(
                STLLR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLLR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLLRB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stllrb",
        aliases: &[],
        opcode: 0x89f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
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
            mnemonic: Mnemonic::r#stllrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLLRB_Rt_ADDR_SIMPLE(
                STLLRB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLLRB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLLRH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stllrh",
        aliases: &[],
        opcode: 0x489f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
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
            mnemonic: Mnemonic::r#stllrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLLRH_Rt_ADDR_SIMPLE(
                STLLRH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLLRH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlr",
        aliases: &[],
        opcode: 0x889ffc00,
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
            mnemonic: Mnemonic::r#stlr,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLR_Rt_ADDR_SIMPLE(
                STLR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLRB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlrb",
        aliases: &[],
        opcode: 0x89ffc00,
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
            mnemonic: Mnemonic::r#stlrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLRB_Rt_ADDR_SIMPLE(
                STLRB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLRB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLRH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlrh",
        aliases: &[],
        opcode: 0x489ffc00,
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
            mnemonic: Mnemonic::r#stlrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLRH_Rt_ADDR_SIMPLE(
                STLRH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLRH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLUR_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0x99000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
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
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
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
            mnemonic: Mnemonic::r#stlur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLUR_Rt_ADDR_OFFSET(
                STLUR_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLUR_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLUR_Rt_X_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0xd9000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
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
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
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
            mnemonic: Mnemonic::r#stlur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLUR_Rt_X_ADDR_OFFSET(
                STLUR_Rt_X_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLUR_Rt_X_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLURB_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlurb",
        aliases: &[],
        opcode: 0x19000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
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
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
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
            mnemonic: Mnemonic::r#stlurb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLURB_Rt_ADDR_OFFSET(
                STLURB_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLURB_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLURH_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlurh",
        aliases: &[],
        opcode: 0x59000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
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
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
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
            mnemonic: Mnemonic::r#stlurh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLURH_Rt_ADDR_OFFSET(
                STLURH_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLURH_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlxp",
        aliases: &[],
        opcode: 0x88208000,
        mask: 0xbfe08000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
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
            mnemonic: Mnemonic::r#stlxp,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(
                STLXP_Rs_Rt_Rt2_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
